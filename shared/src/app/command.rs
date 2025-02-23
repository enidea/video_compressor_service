use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumString, EnumIter, Display, Serialize, Deserialize)]
pub enum Command {
    Compress,
    Resize,
    ChangeAspectRatio,
    ConvertToAudio,
    ConvertToGifOrWebmWithTimeRange,
}
