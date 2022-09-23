use std::str::FromStr;
use std::time::Duration;

use crate::crawler::*;
use crate::database::*;
use crate::settings::*;

use reqwest::Client;
use sqlx::{Pool, Sqlite};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Handler {
    pub root: String,
    pub pool: Pool<Sqlite>,
    pub client: Client,
    pub timeout: u64,
}

impl Handler {
    pub fn build(
        settings: &Settings,
        pool: Pool<Sqlite>,
        root: String,
    ) -> Result<Handler, reqwest::Error> {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .timeout(Duration::new(settings.client.timeout, 0))
            .build()?;

        Ok(Handler {
            root,
            pool,
            client,
            timeout: settings.client.timeout,
        })
    }

    pub async fn run(&self, _sender: Sender<()>) {
        let mut crl = Crawler::new(self.root.clone());
        let res = crl.crawl(&self.client, self.timeout).await;
        //println!("crawwwler: {:?}", crl);
        match res {
            Ok(_) => {
                let _ = insert(&self.pool, &crl).await;
            }
            Err(_) => {}
        };
    }
}
