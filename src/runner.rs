use crate::crawler::*;
use crate::database::*;

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
