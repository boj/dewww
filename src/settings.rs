use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub file: String,
    pub url: String,
    pub max_conns: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Client {
    pub timeout: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Runner {
    pub delay: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub runner: Runner,
    pub client: Client,
    pub database: Database,
}

impl Settings {
    pub fn new(file: &str, prefix: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(file))
            .add_source(Environment::with_prefix(prefix))
            .build()?;

        s.try_deserialize()
    }
}