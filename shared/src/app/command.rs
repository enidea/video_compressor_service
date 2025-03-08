use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use super::Resolution;

#[derive(Debug, Clone, Copy, EnumString, EnumIter, Display, Serialize, Deserialize)]
pub enum Command {
    Compress,
    Resize { resolution: Resolution },
    ChangeAspectRatio,
    ConvertToAudio,
    ConvertToGifOrWebmWithTimeRange,
}
