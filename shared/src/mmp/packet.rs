use super::{Json, MediaType, Payload};

#[derive(Debug)]
pub struct PacketHeader {
    pub json_size: usize,
    pub media_type_size: usize,
    pub payload_size: usize,
}

impl PacketHeader {
    pub const HEADER_SIZE_BYTES: usize =
        Json::HEADER_SIZE_BYTES + MediaType::HEADER_SIZE_BYTES + Payload::HEADER_SIZE_BYTES;

    pub fn generate_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.push(self.json_size as u8);
        bytes.extend_from_slice(&(self.media_type_size as u16).to_be_bytes());
        bytes.extend_from_slice(&(self.payload_size as u64).to_be_bytes());

        bytes
    }

    pub fn generate_from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() < Self::HEADER_SIZE_BYTES {
            panic!("Not enough bytes to generate PacketHeader");
        }

        let (json_size_bytes, bytes) = bytes.split_at(Json::HEADER_SIZE_BYTES);
        let (media_type_size_bytes, bytes) = bytes.split_at(MediaType::HEADER_SIZE_BYTES);

        Ok(Self {
            json_size: u16::from_be_bytes(json_size_bytes.try_into().unwrap()) as usize,
            media_type_size: u8::from_be_bytes(media_type_size_bytes.try_into().unwrap()) as usize,
            payload_size: u64::from_be_bytes(bytes.try_into().unwrap()) as usize,
        })
    }
}

#[derive(Debug)]
pub struct Packet {
    pub json: Json,
    pub media_type: MediaType,
    pub payload: Payload,
}

impl Packet {
    pub fn new(json: Json, media_type: MediaType, payload: Payload) -> Self {
        Self {
            json,
            media_type,
            payload,
        }
    }

    pub fn generate_header(&self) -> PacketHeader {
        PacketHeader {
            json_size: self.json.data.to_string().len(),
            media_type_size: self.media_type.to_string().len(),
            payload_size: self.payload.media_file_path.metadata().unwrap().len() as usize,
        }
    }
}
