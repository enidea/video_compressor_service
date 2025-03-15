use derive_more::Display;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumIter, EnumString, Serialize, Deserialize, Default, Display)]
pub enum AspectRatio {
    #[display("16:9 (Wide Screen)")]
    WideScreen16_9,
    #[display("4:3 (Standard)")]
    Standard4_3,
    #[display("21:9 (Cinematic)")]
    Cinematic21_9,
    #[default]
    #[display("9:16 (Vertical)")]
    Vertical9_16,
    #[display("1:1 (Square)")]
    Square1_1,
    #[display("2.35:1 (Cinema)")]
    Cinema2_35_1,
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
