use std::fs::File;

use shared::app;

pub struct CommandProcessor;

impl CommandProcessor {
    pub fn process(command: app::Command, file: File) {
        match command {
            app::Command::Compress => {
                println!("Compressing file...");
            }
            app::Command::Resize => {
                println!("Resizing file...");
            }
            app::Command::ChangeAspectRatio => {
                println!("Changing aspect ratio...");
            }
            app::Command::ConvertToAudio => {
                println!("Converting file to audio...");
            }
            app::Command::ConvertToGifOrWebmWithTimeRange => {
                println!("Converting file to GIF or WebM with time range...");
            }
        }
    }
}
