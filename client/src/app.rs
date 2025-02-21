mod cli_args;
mod command_prompter;
mod video_file_validator;

use clap::Parser;
use cli_args::CliArgs;
use command_prompter::CommandPrompter;
use serde_json::json;
use shared::{app, mmp};
use video_file_validator::VideoFileValidator;

use std::{net::TcpStream, path::PathBuf, str::FromStr};

pub fn run() -> anyhow::Result<()> {
    let app_config = app::Config::new()?;

    let cli_args = CliArgs::parse();

    let video_file_path = PathBuf::from_str(&cli_args.file_path)?;

    VideoFileValidator::validate(&video_file_path, app_config.max_file_size_gb)?;

    let command = CommandPrompter::prompt()?;

    let tcp_stream = TcpStream::connect(app_config.server_addr)?;

    let mut mmp_stream = mmp::Stream::new(tcp_stream, app_config.max_packet_size);

    let packet = mmp::Packet::new(
        mmp::Json::new(json!({
            "command": command,
        }))?,
        mmp::MediaType::Mp4,
        mmp::Payload::new(video_file_path)?,
    );

    mmp_stream.send_packet(&packet)?;

    // util::FileUploader::upload_file(&mut tcp_stream, &mut video_file)?;

    // let response = mmp::Response::from_bytes(&util::TcpUtil::read_bytes(&mut tcp_stream)?)?;

    // match response.status {
    //     mmp::Status::Ok => {
    //         println!("File uploaded successfully!");
    //     }
    //     mmp::Status::BadRequest => {
    //         eprintln!("Error uploading file!");
    //     }
    //     mmp::Status::InternalServerError => {
    //         eprintln!("Server error!");
    //     }
    // }

    Ok(())
}
