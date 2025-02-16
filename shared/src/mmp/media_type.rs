use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, EnumString, Display)]
pub enum MediaType {
    Mp4,
}
