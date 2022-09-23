use std::str::FromStr;
use std::time::Duration;

use tokio::sync::mpsc::channel;

mod crawler;
mod database;
mod runner;
mod settings;
mod web;

use runner::Handler;
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

    // Kick off process with a seed URL for now
    if let Ok(handler) = Handler::build(&settings, pool, "https://blog.bojo.wtf".to_string()) {
        let (send, mut recv) = channel(1);

        // Spawn the crawler
        handler.run(send.clone()).await;

        // Spawn the web server
        tokio::spawn(web::run());

        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Could not register ctrl+c handler");
        });

        // https://tokio.rs/tokio/topics/shutdown
        drop(send);
        let _ = recv.recv().await;
    }

    Ok(())
}
