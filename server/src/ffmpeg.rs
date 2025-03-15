mod options;

use options::Options;
pub use options::*;

use ffmpeg_next as ffmpeg;
use std::{path::Path, process::Command};

pub fn convert(
    input_file_path: &Path,
    output_file_path: &Path,
    options: Options,
) -> anyhow::Result<()> {
    let mut args = vec![
        String::from("-i"),
        input_file_path.to_str().unwrap().to_string(),
        String::from("-c:v"),
        String::from("libx264"),
        String::from("-pix_fmt"),
        String::from("yuv420p"),
        String::from("-preset"),
        options.preset.to_string(),
        String::from("-crf"),
        options.crf.value().to_string(),
    ];

    if let Some(resolution) = options.resolution {
        args.push(String::from("-s"));
        args.push(format!("{}x{}", resolution.width(), resolution.height()));
    }

    if let (Some(aspect_ratio), Some(aspect_ratio_fit)) =
        (options.aspect_ratio, options.aspect_ratio_fit)
    {
        let (video_width, video_height) = get_video_resolution(input_file_path)?;

        args.push(String::from("-vf"));
        args.push(generate_aspect_ratio_filter(
            video_width,
            video_height,
            &aspect_ratio,
            &aspect_ratio_fit,
        ));
    }

    args.push(output_file_path.to_str().unwrap().to_string());

    let mut command = Command::new("ffmpeg");
    command.args(&args);

    println!("command: {:?}", &command);

    if !command.status()?.success() {
        return Err(anyhow::anyhow!("Failed to convert the file"));
    }

    Ok(())
}

fn generate_aspect_ratio_filter(
    original_width: u32,
    original_height: u32,
    aspect_ratio: &shared::app::AspectRatio,
    aspect_ratio_fit: &shared::app::AspectRatioFit,
) -> String {
    let (width, height) = if original_width > original_height {
        (
            original_width,
            (original_width as f64 / aspect_ratio.width() as f64 * aspect_ratio.height() as f64)
                as u32,
        )
    } else {
        (
            (original_height as f64 / aspect_ratio.height() as f64 * aspect_ratio.width() as f64)
                as u32,
            original_height,
        )
    };

    match aspect_ratio_fit {
        shared::app::AspectRatioFit::ForceFit => {
            format!(
                "scale={}:{}, setdar={}:{}",
                width,
                height,
                aspect_ratio.width(),
                aspect_ratio.height()
            )
        }
        shared::app::AspectRatioFit::BlackPadding => {
            format!("pad={}:{}:(ow-iw)/2:(oh-ih)/2", width, height)
        }
    }
}

fn get_video_resolution(video_file_path: &Path) -> anyhow::Result<(u32, u32)> {
    ffmpeg::init()?;

    let input = ffmpeg::format::input(&video_file_path)?;

    for stream in input.streams() {
        if let Ok(codec) = ffmpeg::codec::Context::from_parameters(stream.parameters()) {
            if let Ok(video) = codec.decoder().video() {
                return Ok((video.width(), video.height()));
            }
        }
    }

    Err(anyhow::anyhow!("Failed to get the video resolution"))
}
