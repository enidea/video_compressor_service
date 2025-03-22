use derive_more::Display;

#[derive(Debug, Clone, Copy, Display, PartialEq)]
pub enum AudioCodec {
    #[display("libmp3lame")]
    Mp3,
    #[display("libopus")]
    Opus,
}
