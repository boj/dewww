use std::str::FromStr;

use tokio::sync::mpsc::channel;

mod crawler;
mod database;
mod runner;
mod settings;
mod types;
mod web;

use runner::*;
use settings::Settings;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let settings = Settings::new("config.toml", "DEWWW").expect("Loading config file failed");

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(settings.database.max_conns)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(&settings.database.url)
                .expect("Database URL failed")
                .filename(&settings.database.file)
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Couldn't run database migrations");

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    // TODO: Kick off process with a seed URL for now
    let handler = HandlerBuilder::new()
        .root(String::from("https://blog.bojo.wtf"))
        .pool(pool)
        .build(&settings, APP_USER_AGENT)?;

    // Spawn the web server
    tokio::spawn(web::run());

    // Register ctrl+c handler
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
    });

    // https://tokio.rs/tokio/topics/shutdown
    let (send, mut recv) = channel(1);
    // Spawn the crawler
    handler.run(send.clone()).await;
    // Cleanup
    drop(send);
    let _ = recv.recv().await;

    Ok(())
}
