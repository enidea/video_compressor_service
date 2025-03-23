use super::{Json, MediaType, Payload};

#[derive(Debug)]
pub struct PacketHeader {
    pub json_size: u16,
    pub media_type_size: u8,
    pub payload_size: u64,
}

impl PacketHeader {
    pub const HEADER_SIZE_BYTES: usize =
        Json::HEADER_SIZE_BYTES + MediaType::HEADER_SIZE_BYTES + Payload::HEADER_SIZE_BYTES;

    pub fn generate_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&self.json_size.to_be_bytes());
        bytes.push(self.media_type_size);
        bytes.extend_from_slice(&self.payload_size.to_be_bytes());

        bytes
    }

    pub fn generate_from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() < Self::HEADER_SIZE_BYTES {
            panic!("Not enough bytes to generate PacketHeader");
        }

        let (json_size_bytes, bytes) = bytes.split_at(Json::HEADER_SIZE_BYTES);
        let (media_type_size_bytes, bytes) = bytes.split_at(MediaType::HEADER_SIZE_BYTES);

        Ok(Self {
            json_size: u16::from_be_bytes(json_size_bytes.try_into().unwrap()),
            media_type_size: u8::from_be_bytes(media_type_size_bytes.try_into().unwrap()),
            payload_size: u64::from_be_bytes(bytes.try_into().unwrap()),
        })
    }
}

#[derive(Debug)]
pub struct Packet {
    pub json: Json,
    pub media_type: Option<MediaType>,
    pub payload: Option<Payload>,
}

impl Packet {
    pub fn new(json: Json, payload: Option<Payload>) -> Self {
        let media_type = payload.as_ref().map(|payload| MediaType::generate_from_path(&payload.media_file_path).unwrap());

        Self {
            json,
            media_type,
            payload,
        }
    }

    pub fn generate_header(&self) -> anyhow::Result<PacketHeader> {
        Ok(PacketHeader {
            json_size: self.json.get_size(),
            media_type_size: match &self.media_type {
                Some(media_type) => media_type.get_size(),
                None => 0,
            },
            payload_size: match &self.payload {
                Some(payload) => payload.get_size(),
                None => 0,
            },
        })
    }
}
