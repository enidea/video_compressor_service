mod aspect_ratio;
mod aspect_ratio_fit;
mod clip_range;
mod media_type_for_clip;
mod resolution;

pub use aspect_ratio::AspectRatio;
pub use aspect_ratio_fit::AspectRatioFit;
pub use clip_range::ClipRange;
pub use media_type_for_clip::MediaTypeForClip;
pub use resolution::Resolution;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumString, EnumIter, Display, Serialize, Deserialize)]
pub enum Command {
    Compress,
    Resize {
        resolution: Resolution,
    },
    ChangeAspectRatio {
        aspect_ratio: AspectRatio,
        aspect_ratio_fit: AspectRatioFit,
    },
    ConvertToAudio,
    Clip {
        clip_range: ClipRange,
        media_type: MediaTypeForClip,
    },
}
