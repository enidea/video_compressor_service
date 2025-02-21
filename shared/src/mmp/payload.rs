use std::{fs::File, path::PathBuf};

#[derive(Debug)]
pub struct Payload {
    pub media_file_path: PathBuf,
}

impl Payload {
    pub const HEADER_SIZE_BYTES: usize = 8;
    const MAX_SIZE: usize = (1 << (5 * 8)) - 1;

    pub fn new(media_file_path: PathBuf) -> anyhow::Result<Self> {
        if File::open(&media_file_path)?.metadata()?.len() as usize > Self::MAX_SIZE {
            return Err(anyhow::anyhow!("Payload data is too large"));
        }

        Ok(Self { media_file_path })
    }

    pub fn get_size(&self) -> u64 {
        self.media_file_path.metadata().unwrap().len()
    }
}
