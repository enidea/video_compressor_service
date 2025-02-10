mod cli_args;
mod video_file_validator;

use clap::Parser;
use cli_args::CliArgs;
use shared::{config_loader, FileUploader, TcpUtil};
use video_file_validator::VideoFilePathValidator;

use std::{fs::File, net::TcpStream};

pub fn run() -> anyhow::Result<()> {
    let app_config = config_loader::load_config()?;

    let cli_args = CliArgs::parse();

    VideoFilePathValidator::validate(&cli_args.file_path)?;

    let mut video_file = File::open(&cli_args.file_path)?;
    let mut tcp_stream = TcpStream::connect(app_config.server_addr)?;

    FileUploader::upload_file(&mut tcp_stream, &mut video_file)?;

    let response = protocol::Response::from_bytes(&TcpUtil::read_bytes(&mut tcp_stream)?)?;

    match response.status {
        protocol::Status::Ok => {
            println!("File uploaded successfully!");
        }
        protocol::Status::BadRequest => {
            eprintln!("Error uploading file!");
        }
        protocol::Status::InternalServerError => {
            eprintln!("Server error!");
        }
    }

    Ok(())
}
