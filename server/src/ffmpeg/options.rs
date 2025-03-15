mod aspect_ratio;
mod crf;
mod preset;
mod resolution;

pub use aspect_ratio::AspectRatio;
pub use crf::Crf;
pub use preset::Preset;
pub use resolution::Resolution;

use derive_builder::Builder;
#[derive(Debug, Clone, Builder)]
pub struct Options {
    #[builder(setter(into, strip_option), default = Crf::new(23).unwrap())]
    pub crf: Crf,
    #[builder(setter(into, strip_option), default = Preset::Medium)]
    pub preset: Preset,
    #[builder(setter(into, strip_option), default)]
    pub resolution: Option<Resolution>,
    #[builder(setter(into, strip_option), default)]
    pub aspect_ratio: Option<AspectRatio>,
    #[builder(setter(into, strip_option), default)]
    pub aspect_ratio_fit: Option<shared::app::AspectRatioFit>,
}
