use std::{fs::File, path::Path};

use shared::util;

pub struct VideoFileValidator;

impl VideoFileValidator {
    pub fn validate(video_file_path: &Path, max_file_size_gb: f64) -> anyhow::Result<()> {
        if !video_file_path.exists() {
            return Err(anyhow::anyhow!("The video file does not exist"));
        }

        if !video_file_path.is_file() {
            return Err(anyhow::anyhow!("The video file is not a file"));
        }

        if video_file_path.extension().unwrap() != "mp4" {
            return Err(anyhow::anyhow!("The video file is not an mp4 file"));
        }

        if File::open(video_file_path)?.metadata()?.len()
            > (util::data_size::gb_to_bytes(max_file_size_gb)) as u64
        {
            return Err(anyhow::anyhow!(format!(
                "The video file size exceeds the maximum file size of {} GB",
                max_file_size_gb
            )));
        }

        Ok(())
    }
}
