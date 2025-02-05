use std::path::Path;

pub struct VideoFileValidator;

impl VideoFileValidator {
    pub fn validate(video_path: &str) -> anyhow::Result<()> {
        let video_path = Path::new(video_path);

        if !video_path.exists() {
            return Err(anyhow::anyhow!("The video file does not exist"));
        }

        if !video_path.is_file() {
            return Err(anyhow::anyhow!("The video file is not a file"));
        }

        Ok(())
    }
}
