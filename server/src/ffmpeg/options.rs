mod crf;
mod preset;

pub use crf::Crf;
pub use preset::Preset;

use derive_builder::Builder;
#[derive(Debug, Clone, Builder)]
pub struct Options {
    #[builder(setter(into, strip_option), default = Crf::new(23).unwrap())]
    pub crf: Crf,
    #[builder(setter(into, strip_option), default = Preset::Medium)]
    pub preset: Preset,
    #[builder(setter(into, strip_option), default)]
    pub resolution: Option<shared::app::Resolution>,
    #[builder(setter(into, strip_option), default)]
    pub aspect_ratio: Option<shared::app::AspectRatio>,
}
