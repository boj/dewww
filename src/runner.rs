use std::time::Duration;

use crate::crawler::*;
use crate::database::*;
use crate::settings::*;
use crate::types::*;

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
    pub async fn run(&self, _sender: Sender<()>) {
        let mut site = SiteData::new(self.root.clone());
        let res = crawl(&mut site, &self.client, self.timeout).await;
        //println!("crawwwler: {:?}", crl);
        match res {
            Ok(_) => {
                let _ = insert(&self.pool, &site).await;
            }
            Err(_) => {}
        };
    }
}

pub struct HandlerBuilder {
    pub root: String,
    pub pool: Option<Pool<Sqlite>>,
}

impl HandlerBuilder {
    pub fn new() -> HandlerBuilder {
        HandlerBuilder {
            root: String::from(""),
            pool: None,
        }
    }

    pub fn root(mut self, root: String) -> HandlerBuilder {
        self.root = root;
        self
    }

    pub fn pool(mut self, pool: Pool<Sqlite>) -> HandlerBuilder {
        self.pool = Some(pool);
        self
    }

    pub fn build(self, settings: &Settings, agent: &str) -> Result<Handler, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(agent)
            .timeout(Duration::new(settings.client.timeout, 0))
            .build()?;

        if let Some(pool) = self.pool {
            Ok(Handler {
                root: self.root,
                pool,
                client,
                timeout: settings.runner.delay,
            })
        } else {
            panic!("Cannot use Handler without a proper DB Pool")
        }
    }
}
