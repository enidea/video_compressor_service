mod options;

use options::Options;
pub use options::*;

use ffmpeg_next::{self as ffmpeg};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn convert(
    input_file_path: &Path,
    output_file_path_without_ext: &Path,
    options: Options,
) -> anyhow::Result<PathBuf> {
    let output_file_path = generate_output_file_path_from(output_file_path_without_ext, &options);

    let args = generate_args(input_file_path, &output_file_path, &options)?;

    let mut command = Command::new("ffmpeg");
    command.args(&args);

    println!("command: {:?}", &command);

    if !command.status()?.success() {
        return Err(anyhow::anyhow!("Failed to convert the file"));
    }

    Ok(output_file_path)
}

fn generate_args(
    input_file_path: &Path,
    output_file_path: &Path,
    options: &Options,
) -> anyhow::Result<Vec<String>> {
    let mut args = vec![
        String::from("-i"),
        input_file_path.to_str().unwrap().to_string(),
    ];
    match options.audio_codec {
        Some(audio_codec) => {
            args.push(String::from("-c:a"));
            args.push(audio_codec.to_string());

            if let Some(vbr_quality) = options.vbr_quality {
                args.push(String::from("-q:a"));
                args.push(vbr_quality.value().to_string());
            }
        }
        _ => {
            args.extend([
                String::from("-c:v"),
                String::from("libx264"),
                String::from("-pix_fmt"),
                String::from("yuv420p"),
                String::from("-preset"),
                options.preset.to_string(),
                String::from("-crf"),
                options.crf.value().to_string(),
            ]);

            if let Some(resolution) = options.resolution {
                args.push(String::from("-s"));
                args.push(format!("{}", resolution));
            }

            if let (Some(aspect_ratio), Some(aspect_ratio_fit)) =
                (options.aspect_ratio, options.aspect_ratio_fit)
            {
                let resolution = get_video_resolution(input_file_path)?;

                args.push(String::from("-vf"));
                args.push(generate_aspect_ratio_filter(
                    resolution,
                    &aspect_ratio,
                    &aspect_ratio_fit,
                ));
            }

            if let Some(clip_range) = options.clip_range {
                args.push(String::from("-ss"));
                args.push(clip_range.formatted_start());
                args.push(String::from("-to"));
                args.push(clip_range.formatted_end());
            }
        }
    }

    args.push(output_file_path.to_str().unwrap().to_string());

    Ok(args)
}

fn generate_aspect_ratio_filter(
    original_resolution: Resolution,
    aspect_ratio: &AspectRatio,
    aspect_ratio_fit: &shared::app::AspectRatioFit,
) -> String {
    let resolution = if original_resolution.width() > original_resolution.height() {
        Resolution::new(
            original_resolution.width(),
            (original_resolution.width() as f64 / aspect_ratio.width() as f64
                * aspect_ratio.height() as f64) as u32,
        )
    } else {
        Resolution::new(
            (original_resolution.height() as f64 / aspect_ratio.height() as f64
                * aspect_ratio.width() as f64) as u32,
            original_resolution.height(),
        )
    }
    .unwrap();

    match aspect_ratio_fit {
        shared::app::AspectRatioFit::ForceFit => {
            format!("scale={}, setsar=1:1", resolution)
        }
        shared::app::AspectRatioFit::BlackPadding => {
            format!("pad={}:(ow-iw)/2:(oh-ih)/2", resolution)
        }
    }
}

fn get_video_resolution(video_file_path: &Path) -> anyhow::Result<Resolution> {
    ffmpeg::init()?;

    let input = ffmpeg::format::input(&video_file_path)?;

    for stream in input.streams() {
        if let Ok(codec) = ffmpeg::codec::Context::from_parameters(stream.parameters()) {
            if let Ok(video) = codec.decoder().video() {
                return Resolution::new(video.width(), video.height());
            }
        }
    }

    Err(anyhow::anyhow!("Failed to get the video resolution"))
}

fn generate_output_file_path_from(
    output_file_path_without_ext: &Path,
    options: &Options,
) -> PathBuf {
    let extension = if let Some(audio_codec) = options.audio_codec {
        audio_codec.extension_str()
    } else {
        "mp4"
    };

    output_file_path_without_ext.with_extension(extension)
}
