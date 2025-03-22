use derive_more::Display;

use super::Preset;

#[derive(Debug, Clone, Copy, Display)]
pub enum VideoCodec {
    #[display("libx264")]
    H264,
    #[display("libvpx-vp9")]
    Vp9,
}

impl VideoCodec {
    pub fn allowed_presets(&self) -> Vec<Preset> {
        match self {
            Self::H264 => vec![Preset::Medium, Preset::Slower],
            _ => vec![],
        }
    }

    pub fn allowed_pixel_formats(&self) -> Vec<super::PixelFormat> {
        match self {
            Self::H264 | Self::Vp9 => vec![super::PixelFormat::Yuv420p],
        }
    }

    pub fn default_pixel_format(&self) -> Option<super::PixelFormat> {
        Some(match self {
            Self::H264 | Self::Vp9 => super::PixelFormat::Yuv420p,
        })
    }
}
