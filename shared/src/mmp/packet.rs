use std::str::FromStr;

use crate::util::slice::SliceExt;

use super::{Json, MediaType};

#[derive(Debug, Clone)]
pub struct Packet {
    pub json: Json,
    pub media_type: MediaType,
}

impl Packet {
    pub fn new(json: Json, media_type: MediaType) -> Self {
        Self { json, media_type }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let json = self.json.data.to_string();
        let media_type = self.media_type.to_string();

        let mut bytes = vec![];
        bytes.extend_from_slice((json.len() as u16).to_be_bytes().as_ref());
        bytes.push(media_type.len() as u8);
        bytes.extend_from_slice(json.as_bytes());
        bytes.extend_from_slice(media_type.as_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &mut &[u8]) -> anyhow::Result<Self> {
        if bytes.is_empty() {
            return Err(anyhow::anyhow!("Packet is empty"));
        }

        let json_size = u16::from_be_bytes(
            bytes
                .split_off_first_at(Json::HEADER_SIZE_BYTES)
                .try_into()
                .unwrap(),
        ) as usize;

        let media_type_size = bytes.split_off_first_at(MediaType::HEADER_SIZE_BYTES)[0] as usize;

        let json = Json::new(serde_json::from_slice(bytes.split_off_first_at(json_size))?)?;
        let media_type = MediaType::from_str(&String::from_utf8_lossy(
            bytes.split_off_first_at(media_type_size),
        ))?;

        Ok(Self::new(json, media_type))
    }
}
