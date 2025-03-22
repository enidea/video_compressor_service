use derive_more::Display;

#[derive(Debug, Clone, Copy, Display, PartialEq)]
pub enum PixelFormat {
    #[display("yuv420p")]
    Yuv420p,
}
