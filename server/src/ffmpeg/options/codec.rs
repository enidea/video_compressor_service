use derive_more::Display;

#[derive(Debug, Clone, Copy, Display)]
pub enum Codec {
    #[display("libx264")]
    H264,
}
