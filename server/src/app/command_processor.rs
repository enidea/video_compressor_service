use std::path::{Path, PathBuf};

use shared::app;

use crate::ffmpeg;

pub struct CommandProcessor;

impl CommandProcessor {
    pub fn process(
        command: app::Command,
        input_file_path: &Path,
        output_file_path_without_ext: &Path,
    ) -> anyhow::Result<PathBuf> {
        let mut transcoder_options_builder = ffmpeg::OptionsBuilder::default();
        let mut extension = input_file_path
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        match command {
            app::Command::Compress => {
                transcoder_options_builder
                    .video_codec(ffmpeg::VideoCodec::H264)
                    .crf(ffmpeg::Crf::new(28)?)
                    .preset(ffmpeg::Preset::Slower);
            }
            app::Command::Resize { resolution } => {
                transcoder_options_builder
                    .video_codec(ffmpeg::VideoCodec::H264)
                    .crf(ffmpeg::Crf::default())
                    .preset(ffmpeg::Preset::Medium)
                    .resolution(ffmpeg::Resolution::new(
                        resolution.width(),
                        resolution.height(),
                    )?);
            }
            app::Command::ChangeAspectRatio {
                aspect_ratio,
                aspect_ratio_fit,
            } => {
                transcoder_options_builder
                    .video_codec(ffmpeg::VideoCodec::H264)
                    .crf(ffmpeg::Crf::default())
                    .preset(ffmpeg::Preset::Medium)
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

                extension = shared::mmp::MediaType::Mp3.to_string();
            }
            app::Command::Clip {
                clip_range,
                media_type,
            } => {
                transcoder_options_builder.clip_range(ffmpeg::ClipRange::new(
                    clip_range.start(),
                    clip_range.end(),
                )?);

                match media_type {
                    shared::app::MediaTypeForClip::Gif => {
                        extension = shared::mmp::MediaType::Gif.to_string();
                    }
                    shared::app::MediaTypeForClip::Webm => {
                        extension = shared::mmp::MediaType::Webm.to_string();

                        transcoder_options_builder
                            .video_codec(ffmpeg::VideoCodec::Vp9)
                            .audio_codec(ffmpeg::AudioCodec::Opus)
                            .crf(ffmpeg::Crf::new(30)?);
                    }
                }
            }
        };

        let output_file_path = output_file_path_without_ext.with_extension(extension);

        ffmpeg::convert(
            input_file_path,
            &output_file_path,
            transcoder_options_builder.build()?,
        )?;

        Ok(output_file_path)
    }
}
