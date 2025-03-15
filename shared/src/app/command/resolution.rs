use derive_more::Display;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, EnumString, EnumIter, Serialize, Deserialize, Default, Display,
)]
pub enum Resolution {
    #[display("4K (3840x2160)")]
    Uhd4k,
    #[display("2K (2560x1440)")]
    Qhd2k,
    #[default]
    #[display("1080p (1920x1080)")]
    Fhd1080,
    #[display("720p (1280x720)")]
    Hd720,
    #[display("480p (854x480)")]
    Sd480,
}

impl Resolution {
    pub fn width(&self) -> u32 {
        match self {
            Resolution::Uhd4k => 3840,
            Resolution::Qhd2k => 2560,
            Resolution::Fhd1080 => 1920,
            Resolution::Hd720 => 1280,
            Resolution::Sd480 => 854,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Resolution::Uhd4k => 2160,
            Resolution::Qhd2k => 1440,
            Resolution::Fhd1080 => 1080,
            Resolution::Hd720 => 720,
            Resolution::Sd480 => 480,
        }
    }

    pub fn get_scale(&self) -> String {
        self.width().to_string() + ":" + &self.height().to_string()
    }
}
