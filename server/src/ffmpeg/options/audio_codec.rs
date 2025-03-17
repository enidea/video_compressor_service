use derive_more::Display;

#[derive(Debug, Clone, Copy, Display, PartialEq)]
pub enum AudioCodec {
    #[display("libmp3lame")]
    Mp3,
}

impl AudioCodec {
    pub fn extension_str(&self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
        }
    }
}
