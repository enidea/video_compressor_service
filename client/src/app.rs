mod cli_args;
mod command_prompter;
mod video_file_validator;

use clap::Parser;
use cli_args::CliArgs;
use command_prompter::CommandPrompter;
use serde_json::json;
use shared::{app, mmp};
use video_file_validator::VideoFilePathValidator;

use std::{fs::File, net::TcpStream};

pub fn run() -> anyhow::Result<()> {
    let app_config = app::Config::new()?;

    let cli_args = CliArgs::parse();

    VideoFilePathValidator::validate(&cli_args.file_path)?;

    let command = CommandPrompter::prompt()?;

    println!("Command: {:?}", command);

    let video_file = File::open(&cli_args.file_path)?;
    let tcp_stream = TcpStream::connect(app_config.server_addr)?;

    let mut mmp_stream = mmp::Stream::new(tcp_stream);

    let packet = mmp::Packet::new(json!({
        "command": command,
    }));

    mmp_stream.send_packet(&packet);

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
