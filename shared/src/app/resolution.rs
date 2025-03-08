use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, EnumString, EnumIter, Serialize, Deserialize, Default, Display,
)]
pub enum Resolution {
    UHD_4K,
    QHD_2K,
    #[default]
    FHD_1080,
    HD_720,
    SD_480,
}
