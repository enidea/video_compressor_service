mod aspect_ratio;
mod audio_codec;
mod clip_range;
mod codec;
mod crf;
mod preset;
mod resolution;
mod vbr_quality;

pub use aspect_ratio::AspectRatio;
pub use audio_codec::AudioCodec;
pub use clip_range::ClipRange;
pub use codec::Codec;
pub use crf::Crf;
pub use preset::Preset;
pub use resolution::Resolution;
pub use vbr_quality::VbrQuality;

use derive_builder::Builder;
#[derive(Debug, Clone, Builder)]
pub struct Options {
    #[builder(setter(into, strip_option), default)]
    pub codec: Option<Codec>,
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
    #[builder(setter(into, strip_option), default)]
    pub audio_codec: Option<AudioCodec>,
    #[builder(setter(into, strip_option), default)]
    pub vbr_quality: Option<VbrQuality>,
    #[builder(setter(into, strip_option), default)]
    pub clip_range: Option<ClipRange>,
}
