use serde::{Deserialize, Serialize};

use super::status::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: Status,
}

impl Response {
    pub fn new(status: Status) -> Self {
        Self { status }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let status = self.status as u16;

        let mut bytes = vec![];
        bytes.extend_from_slice(&status.to_be_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let status = Status::try_from_u16(u16::from_be_bytes([bytes[0], bytes[1]]))?;

        Ok(Self { status })
    }
}
