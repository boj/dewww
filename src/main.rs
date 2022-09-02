use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc::channel;

mod crawler;
mod database;
mod runner;
mod settings;

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

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .timeout(Duration::new(settings.client.timeout, 0))
        .build()?;

    let (send, mut recv) = channel(1);

    // Kick off process with a seed URL for now
    let handler = Handler {
        root: "https://blog.bojo.wtf".to_string(),
        pool,
        client,
        timeout: settings.runner.delay,
    };
    handler.run(send.clone()).await;

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
    });

    // https://tokio.rs/tokio/topics/shutdown
    drop(send);
    let _ = recv.recv().await;

    Ok(())
}
