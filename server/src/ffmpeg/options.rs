mod preset;

use derive_builder::Builder;
pub use preset::Preset;

#[derive(Debug, Clone, Builder)]
pub struct Options {
    #[builder(setter(into, strip_option), default)]
    pub bitrate: Option<usize>,
    #[builder(setter(into, strip_option), default = Preset::Medium)]
    pub preset: Preset,
    #[builder(setter(into, strip_option), default)]
    pub width: Option<u32>,
    #[builder(setter(into, strip_option), default)]
    pub height: Option<u32>,
}
