use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumIter, EnumString, Serialize, Deserialize, Default)]
pub enum AspectRatio {
    WideScreen16_9,
    Standard4_3,
    Cinematic21_9,
    #[default]
    Vertical9_16,
    Square1_1,
    Cinema2_35_1,
}

impl Display for AspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AspectRatio::WideScreen16_9 => "16:9 (Wide Screen)",
                AspectRatio::Standard4_3 => "4:3 (Standard)",
                AspectRatio::Cinematic21_9 => "21:9 (Cinematic)",
                AspectRatio::Vertical9_16 => "9:16 (Vertical)",
                AspectRatio::Square1_1 => "1:1 (Square)",
                AspectRatio::Cinema2_35_1 => "2.35:1 (Cinema)",
            }
        )
    }
}

impl AspectRatio {
    pub fn width(&self) -> u32 {
        match self {
            AspectRatio::WideScreen16_9 => 16,
            AspectRatio::Standard4_3 => 4,
            AspectRatio::Cinematic21_9 => 21,
            AspectRatio::Vertical9_16 => 9,
            AspectRatio::Square1_1 => 1,
            // Convert 2.35:1 to integer ratio
            AspectRatio::Cinema2_35_1 => 235,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            AspectRatio::WideScreen16_9 => 9,
            AspectRatio::Standard4_3 => 3,
            AspectRatio::Cinematic21_9 => 9,
            AspectRatio::Vertical9_16 => 16,
            AspectRatio::Square1_1 => 1,
            // Convert 2.35:1 to integer ratio
            AspectRatio::Cinema2_35_1 => 100,
        }
    }
}
