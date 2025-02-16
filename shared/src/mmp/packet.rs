#[derive(Debug, Clone)]
pub struct Packet {
    pub json: serde_json::Value,
}

impl Packet {
    pub fn new(json: serde_json::Value) -> Self {
        Self { json }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let json = self.json.to_string();

        let mut bytes = vec![];
        bytes.extend_from_slice(json.as_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.is_empty() {
            return Err(anyhow::anyhow!("Packet is empty"));
        }

        let json_data = bytes;

        let json = serde_json::from_slice(json_data)?;

        Ok(Self::new(json))
    }
}
