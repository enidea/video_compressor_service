mod cli_args;
mod command_prompter;
mod video_file_validator;

use clap::Parser;
use cli_args::CliArgs;
use command_prompter::CommandPrompter;
use serde_json::json;
use shared::{app, mmp, util};
use video_file_validator::VideoFileValidator;

use std::{net::TcpStream, path::Path};

pub fn run() -> anyhow::Result<()> {
    let app_config = app::Config::new()?;

    let cli_args = CliArgs::parse();

    let video_file_path = Path::new(&cli_args.file_path);

    VideoFileValidator::validate(video_file_path, app_config.max_file_size_gb)?;

    let command = CommandPrompter::prompt()?;

    let tcp_stream = TcpStream::connect(app_config.server_addr)?;

    let mut mmp_stream = mmp::Stream::new(tcp_stream, app_config.max_packet_size);

    let packet = mmp::Packet::new(
        mmp::Json::new(json!(app::Request::new(command)))?,
        mmp::MediaType::Mp4,
        mmp::Payload::new(video_file_path.to_path_buf())?,
    );

    mmp_stream.send_packet(&packet)?;

    let converted_file_path =
        util::file_path::add_prefix_to_file_path(video_file_path, "converted_")?;

    let (response, _converted_file) = mmp_stream.receive_packet(&converted_file_path)?;

    let response_json: mmp::Response = serde_json::from_value(response.json.data)?;

    match response_json.status {
        mmp::Status::Ok => {
            println!("File converted successfully!");
        }
        mmp::Status::BadRequest => {
            eprintln!("Error uploading file!");
        }
        mmp::Status::InternalServerError => {
            eprintln!("Server error!");
        }
    }

    Ok(())
}
