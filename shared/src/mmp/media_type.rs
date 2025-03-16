use std::{path::Path, str::FromStr};

use anyhow::Context;
use derive_more::Display;
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, EnumString, Display)]
pub enum MediaType {
    #[strum(serialize = "mp4")]
    #[display("mp4")]
    Mp4,
    #[strum(serialize = "mp3")]
    #[display("mp3")]
    Mp3,
}

impl MediaType {
    pub const HEADER_SIZE_BYTES: usize = 1;

    pub fn generate_from_path(path: &Path) -> anyhow::Result<Self> {
        let extension = path.extension().context("Failed to get file extension")?;

        extension
            .to_str()
            .context("Failed to convert extension to str")?
            .parse()
            .context("Failed to parse MediaType")
    }

    pub fn get_size(&self) -> u8 {
        self.to_string().len() as u8
    }

    pub fn generate_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    pub fn generate_from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        MediaType::from_str(std::str::from_utf8(bytes)?).context("Failed to parse MediaType")
    }
}
