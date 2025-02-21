use std::str::FromStr;

use anyhow::Context;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, EnumString, Display)]
pub enum MediaType {
    Mp4,
}

impl MediaType {
    pub const HEADER_SIZE_BYTES: usize = 1;

    pub fn generate_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    pub fn generate_from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        MediaType::from_str(std::str::from_utf8(bytes)?).context("Failed to parse MediaType")
    }
}
