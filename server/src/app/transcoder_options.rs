use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct TranscoderOptions {
    #[builder(setter(into, strip_option), default)]
    pub bitrate: Option<usize>,
    #[builder(setter(into, strip_option), default)]
    pub preset: Option<String>,
}
