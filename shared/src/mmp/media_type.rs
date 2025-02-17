use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, EnumString, Display)]
pub enum MediaType {
    Mp4,
}

impl MediaType {
    pub const HEADER_SIZE_BYTES: usize = 1;
}
