use anyhow::{Context, Ok};
use config::Environment;

#[derive(Debug, serde::Deserialize)]
pub struct AppConfig {
    pub server_addr: String,
    pub download_dir: String,
    pub video_file_name: String,
    pub max_packet_size: usize,
}

pub fn load_config() -> anyhow::Result<AppConfig> {
    dotenvy::dotenv()?;

    let config = config::Config::builder()
        .add_source(Environment::default())
        .build()
        .context("Failed to build the configuration")?;

    Ok(config.try_deserialize()?)
}
