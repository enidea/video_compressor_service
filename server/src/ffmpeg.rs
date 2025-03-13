mod options;

use options::Options;
pub use options::OptionsBuilder;

use std::{path::Path, process::Command};

pub fn convert(
    input_file_path: &Path,
    output_file_path: &Path,
    options: Options,
) -> anyhow::Result<()> {
    let mut args = vec![
        String::from("-i"),
        input_file_path.to_str().unwrap().to_string(),
    ];

    if let (Some(width), Some(height)) = (options.width, options.height) {
        args.push(String::from("-s"));
        args.push(format!("{}x{}", width, height));
    }

    args.push(output_file_path.to_str().unwrap().to_string());

    let status = Command::new("ffmpeg").args(args).status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to convert the file"));
    }

    Ok(())
}
