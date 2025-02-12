use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumString, EnumIter, Display)]
pub enum Command {
    Compress,
    Resize,
    ChangeAspectRatio,
    ConvertToAudio,
    ConvertToGifOrWebmWithTimeRange,
}
