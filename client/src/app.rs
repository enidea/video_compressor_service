mod cli_args;
mod command_prompter;
mod video_file_validator;

use clap::Parser;
use cli_args::CliArgs;
use command_prompter::CommandPrompter;
use shared::{app, protocol, util};
use video_file_validator::VideoFilePathValidator;

use std::{fs::File, net::TcpStream};

pub fn run() -> anyhow::Result<()> {
    let app_config = app::Config::new()?;

    let cli_args = CliArgs::parse();

    VideoFilePathValidator::validate(&cli_args.file_path)?;

    let command = CommandPrompter::prompt()?;

    println!("Command: {:?}", command);

    let mut video_file = File::open(&cli_args.file_path)?;
    let mut tcp_stream = TcpStream::connect(app_config.server_addr)?;

    util::FileUploader::upload_file(&mut tcp_stream, &mut video_file)?;

    let response = protocol::Response::from_bytes(&util::TcpUtil::read_bytes(&mut tcp_stream)?)?;

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
