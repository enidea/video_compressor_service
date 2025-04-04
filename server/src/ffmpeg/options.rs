mod aspect_ratio;
mod audio_codec;
mod clip_range;
mod crf;
mod pixel_format;
mod preset;
mod resolution;
mod vbr_quality;
mod video_codec;

pub use aspect_ratio::AspectRatio;
pub use audio_codec::AudioCodec;
pub use clip_range::ClipRange;
pub use crf::Crf;
pub use pixel_format::PixelFormat;
pub use preset::Preset;
pub use resolution::Resolution;
pub use vbr_quality::VbrQuality;
pub use video_codec::VideoCodec;

use derive_builder::Builder;
#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Options {
    #[builder(setter(into, strip_option), default)]
    pub video_codec: Option<VideoCodec>,
    #[builder(setter(into, strip_option), field(build = "self.build_pixel_format()"))]
    pub pixel_format: Option<PixelFormat>,
    #[builder(setter(into, strip_option), default)]
    pub crf: Option<Crf>,
    #[builder(setter(into, strip_option), default)]
    pub preset: Option<Preset>,
    #[builder(setter(into, strip_option), default)]
    pub resolution: Option<Resolution>,
    #[builder(setter(into, strip_option), default)]
    pub aspect_ratio: Option<AspectRatio>,
    #[builder(setter(into, strip_option), default)]
    pub aspect_ratio_fit: Option<shared::app::AspectRatioFit>,
    #[builder(setter(into, strip_option), default)]
    pub clip_range: Option<ClipRange>,
    #[builder(setter(into, strip_option), default)]
    pub audio_codec: Option<AudioCodec>,
    #[builder(setter(into, strip_option), default)]
    pub vbr_quality: Option<VbrQuality>,
}

impl OptionsBuilder {
    fn build_pixel_format(&self) -> Option<PixelFormat> {
        self.pixel_format.flatten().or_else(|| {
            self.video_codec
                .flatten()
                .and_then(|vc| vc.default_pixel_format())
        })
    }

    fn validate(&self) -> anyhow::Result<(), String> {
        let video_codec = self.video_codec.flatten();
        let pixel_format = self.pixel_format.flatten();
        let preset = self.preset.flatten();

        if video_codec.is_some()
            && pixel_format.is_some()
            && !video_codec
                .unwrap()
                .allowed_pixel_formats()
                .contains(&pixel_format.unwrap())
        {
            return Err(format!(
                "The pixel format {} is not allowed for the video codec {}",
                pixel_format.unwrap(),
                video_codec.unwrap()
            ));
        }

        if video_codec.is_some()
            && preset.is_some()
            && !video_codec
                .unwrap()
                .allowed_presets()
                .contains(&preset.unwrap())
        {
            return Err(format!(
                "The preset {} is not allowed for the video codec {}",
                preset.unwrap(),
                video_codec.unwrap()
            ));
        }

        Ok(())
    }
}
