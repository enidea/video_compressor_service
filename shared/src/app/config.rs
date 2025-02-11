use anyhow::{Context, Ok};
use config::Environment;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub download_dir: String,
    pub video_file_name: String,
    pub max_packet_size: usize,
    pub max_file_size_gb: f64,
}

impl Config {
    pub fn new() -> anyhow::Result<Config> {
        dotenvy::dotenv()?;

        let config = config::Config::builder()
            .add_source(Environment::default())
            .build()
            .context("Failed to build the configuration")?;

        Ok(config.try_deserialize()?)
    }
}
