use std::path::{Path, PathBuf};

use shared::app;

use crate::ffmpeg;

pub struct CommandProcessor;

impl CommandProcessor {
    pub fn process(
        command: app::Command,
        input_file_path: &Path,
        output_file_path: &Path,
    ) -> anyhow::Result<PathBuf> {
        let mut transcoder_options_builder = ffmpeg::OptionsBuilder::default();

        match command {
            app::Command::Compress => {
                transcoder_options_builder
                    .crf(ffmpeg::Crf::new(28)?)
                    .preset(ffmpeg::Preset::Slower);
            }
            app::Command::Resize { resolution } => {
                transcoder_options_builder.resolution(ffmpeg::Resolution::new(
                    resolution.width(),
                    resolution.height(),
                )?);
            }
            app::Command::ChangeAspectRatio {
                aspect_ratio,
                aspect_ratio_fit,
            } => {
                transcoder_options_builder
                    .aspect_ratio(ffmpeg::AspectRatio::new(
                        aspect_ratio.width(),
                        aspect_ratio.height(),
                    )?)
                    .aspect_ratio_fit(aspect_ratio_fit);
            }
            app::Command::ConvertToAudio => {
                transcoder_options_builder
                    .audio_codec(ffmpeg::AudioCodec::Mp3)
                    .vbr_quality(ffmpeg::VbrQuality::new(2)?);
            }
            app::Command::ConvertToGifOrWebmWithTimeRange { clip_range } => {
                transcoder_options_builder.clip_range(ffmpeg::ClipRange::new(
                    clip_range.start(),
                    clip_range.end(),
                )?);
            }
        };

        ffmpeg::convert(
            input_file_path,
            output_file_path,
            transcoder_options_builder.build()?,
        )
    }
}
