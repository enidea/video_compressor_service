use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct TranscoderOptions {
    #[builder(setter(into, strip_option), default)]
    pub bitrate: Option<usize>,
    #[builder(setter(into, strip_option), default)]
    pub preset: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub width: Option<u32>,
    #[builder(setter(into, strip_option), default)]
    pub height: Option<u32>,
}
