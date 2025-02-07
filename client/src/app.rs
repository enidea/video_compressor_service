mod prompt;
mod prompts;
mod video_file_validator;

use prompt::prompt;
use shared::file_uploader::FileUploader;
use video_file_validator::VideoFilePathValidator;

use std::{env, fs::File, net::TcpStream};

pub fn run() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let video_path_str = prompt(prompts::VIDEO_PATH_PROMPT)?;

    VideoFilePathValidator::validate(&video_path_str)?;

    let mut video_file = File::open(video_path_str)?;
    let mut tcp_stream = TcpStream::connect(env::var("SERVER_ADDR")?)?;

    FileUploader::upload_file(&mut tcp_stream, &mut video_file)
}
