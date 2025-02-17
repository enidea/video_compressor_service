use std::str::FromStr;

use super::MediaType;

#[derive(Debug, Clone)]
pub struct Packet {
    pub json: serde_json::Value,
    pub media_type: MediaType,
}

impl Packet {
    pub fn new(json: serde_json::Value, media_type: MediaType) -> Self {
        Self { json, media_type }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let json = self.json.to_string();
        let media_type = self.media_type.to_string();

        let mut bytes = vec![];
        bytes.extend_from_slice((json.len() as u16).to_be_bytes().as_ref());
        bytes.extend_from_slice((media_type.len() as u8).to_be_bytes().as_ref());
        bytes.extend_from_slice(json.as_bytes());
        bytes.extend_from_slice(media_type.as_bytes());

        println!("to_bytes: {:?}", bytes);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.is_empty() {
            return Err(anyhow::anyhow!("Packet is empty"));
        }

        let json_size = u16::from_be_bytes([bytes[0], bytes[1]]) as usize;
        let media_type_size = u8::from_be_bytes([bytes[2]]) as usize;

        let json = serde_json::from_slice(&bytes[3..3 + json_size])?;
        let media_type = MediaType::from_str(&String::from_utf8_lossy(
            &bytes[3 + json_size..3 + json_size + media_type_size],
        ))?;

        Ok(Self::new(json, media_type))
    }
}
