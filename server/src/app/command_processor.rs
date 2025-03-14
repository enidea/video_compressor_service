use std::path::Path;

use anyhow::Ok;
use shared::app;

use crate::ffmpeg;

pub struct CommandProcessor;

impl CommandProcessor {
    pub fn process(
        command: app::Command,
        input_file_path: &Path,
        output_file_path: &Path,
    ) -> anyhow::Result<()> {
        let mut transcoder_options_builder = ffmpeg::OptionsBuilder::default();

        match command {
            app::Command::Compress => {
                transcoder_options_builder
                    .crf(ffmpeg::Crf::new(28)?)
                    .preset(ffmpeg::Preset::Slower);
            }
            app::Command::Resize { resolution } => {
                transcoder_options_builder.resolution(resolution);
            }
            app::Command::ChangeAspectRatio {
                aspect_ratio,
                aspect_ratio_fit,
            } => {
                transcoder_options_builder
                    .aspect_ratio(aspect_ratio)
                    .aspect_ratio_fit(aspect_ratio_fit);
            }
            // app::Command::ConvertToAudio => {
            //     println!("Converting file to audio...");
            //     Ok(())
            // }
            // app::Command::ConvertToGifOrWebmWithTimeRange => {
            //     println!("Converting file to GIF or WebM with time range...");
            //     Ok(())
            // }
            _ => {}
        };

        ffmpeg::convert(
            input_file_path,
            output_file_path,
            transcoder_options_builder.build()?,
        )?;

        Ok(())
    }
}
