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
