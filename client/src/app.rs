mod cli_args;
mod video_file_validator;

use clap::Parser;
use cli_args::CliArgs;
use shared::FileUploader;
use video_file_validator::VideoFilePathValidator;

use std::{env, fs::File, net::TcpStream};

pub fn run() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let cli_args = CliArgs::parse();

    VideoFilePathValidator::validate(&cli_args.file_path)?;

    let mut video_file = File::open(&cli_args.file_path)?;
    let mut tcp_stream = TcpStream::connect(env::var("SERVER_ADDR")?)?;

    FileUploader::upload_file(&mut tcp_stream, &mut video_file)
}
