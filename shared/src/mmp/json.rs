use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Json {
    pub data: serde_json::Value,
}

impl Json {
    pub const HEADER_SIZE_BYTES: usize = 2;
    const MAX_SIZE: usize = u16::MAX as usize;

    pub fn new(data: serde_json::Value) -> anyhow::Result<Self> {
        if data.to_string().len() > Self::MAX_SIZE {
            return Err(anyhow::anyhow!("Json data is too large"));
        }

        Ok(Self { data })
    }

    pub fn generate_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self.data).unwrap()
    }

    pub fn generate_from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let data = serde_json::from_slice(bytes).context("Failed to parse JSON")?;

        Ok(Self { data })
    }
}
