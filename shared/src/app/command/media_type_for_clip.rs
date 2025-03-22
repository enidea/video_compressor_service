use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, EnumIter, EnumString, Display)]
pub enum MediaTypeForClip {
    Gif,
    #[default]
    Webm,
}
