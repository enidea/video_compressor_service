use serde::{Deserialize, Serialize};

use super::status::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: Status,
    pub message: Option<String>,
}

impl Response {
    pub fn new(status: Status, message: Option<String>) -> Self {
        Self { status, message }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let status = self.status as u16;

        let mut bytes = vec![];
        bytes.extend_from_slice(&status.to_be_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let status = Status::try_from_u16(u16::from_be_bytes([bytes[0], bytes[1]]))?;
        let message = String::from_utf8_lossy(&bytes[2..]).to_string();

        Ok(Self {
            status,
            message: Some(message),
        })
    }
}
