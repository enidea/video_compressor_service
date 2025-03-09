use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, EnumString, EnumIter, Serialize, Deserialize, Default)]
pub enum Resolution {
    Uhd4k,
    Qhd2k,
    #[default]
    Fhd1080,
    Hd720,
    Sd480,
}

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Resolution::Uhd4k => "4K (3840x2160)",
                Resolution::Qhd2k => "2K (2560x1440)",
                Resolution::Fhd1080 => "1080p (1920x1080)",
                Resolution::Hd720 => "720p (1280x720)",
                Resolution::Sd480 => "480p (854x480)",
            }
        )
    }
}

impl Resolution {
    pub fn get_width(&self) -> u32 {
        match self {
            Resolution::Uhd4k => 3840,
            Resolution::Qhd2k => 2560,
            Resolution::Fhd1080 => 1920,
            Resolution::Hd720 => 1280,
            Resolution::Sd480 => 854,
        }
    }

    pub fn get_height(&self) -> u32 {
        match self {
            Resolution::Uhd4k => 2160,
            Resolution::Qhd2k => 1440,
            Resolution::Fhd1080 => 1080,
            Resolution::Hd720 => 720,
            Resolution::Sd480 => 480,
        }
    }

    pub fn get_scale(&self) -> String {
        self.get_width().to_string() + ":" + &self.get_height().to_string()
    }
}
