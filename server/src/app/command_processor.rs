use std::path::Path;

use anyhow::Ok;
use shared::app;

use super::{converter, transcoder_options::TranscoderOptions};

pub struct CommandProcessor;

impl CommandProcessor {
    pub fn process(
        command: app::Command,
        input_file_path: &Path,
        output_file_path: &Path,
    ) -> anyhow::Result<()> {
        ffmpeg_next::init()?;

        let transcoder_options = match command {
            app::Command::Compress => {
                TranscoderOptions::new(Some(500_000), Some("veryslow".to_string()))
            }
            _ => TranscoderOptions::new(None, None),
            // app::Command::Resize => {
            //     println!("Resizing file...");
            //     Ok(())
            // }
            // app::Command::ChangeAspectRatio => {
            //     println!("Changing aspect ratio...");
            //     Ok(())
            // }
            // app::Command::ConvertToAudio => {
            //     println!("Converting file to audio...");
            //     Ok(())
            // }
            // app::Command::ConvertToGifOrWebmWithTimeRange => {
            //     println!("Converting file to GIF or WebM with time range...");
            //     Ok(())
            // }
        };

        converter::convert(input_file_path, output_file_path, transcoder_options)?;

        Ok(())
    }
}
