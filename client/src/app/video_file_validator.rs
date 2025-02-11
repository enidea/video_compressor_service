use std::{fs::File, path::Path};

use shared::{config_loader, data_size};

pub struct VideoFilePathValidator;

impl VideoFilePathValidator {
    pub fn validate(video_file_path: &str) -> anyhow::Result<()> {
        let app_config = config_loader::load_config()?;

        let video_path = Path::new(video_file_path);

        if !video_path.exists() {
            return Err(anyhow::anyhow!("The video file does not exist"));
        }

        if !video_path.is_file() {
            return Err(anyhow::anyhow!("The video file is not a file"));
        }

        if video_path.extension().unwrap() != "mp4" {
            return Err(anyhow::anyhow!("The video file is not an mp4 file"));
        }

        if File::open(video_file_path)?.metadata()?.len()
            > (data_size::gb_to_bytes(app_config.max_file_size_gb as f64)) as u64
        {
            return Err(anyhow::anyhow!(format!(
                "The video file size exceeds the maximum file size of {} GB",
                app_config.max_file_size_gb
            )));
        }

        Ok(())
    }
}
