mod aspect_ratio;
mod resolution;

pub use aspect_ratio::AspectRatio;
pub use resolution::Resolution;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumString, EnumIter, Display, Serialize, Deserialize)]
pub enum Command {
    Compress,
    Resize { resolution: Resolution },
    ChangeAspectRatio { aspect_ratio: AspectRatio },
    ConvertToAudio,
    ConvertToGifOrWebmWithTimeRange,
}
