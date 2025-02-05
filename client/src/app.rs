mod prompt;
mod prompts;
mod video_file_validator;

use prompt::prompt;
use video_file_validator::VideoFileValidator;

use std::{
    env,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

pub fn run() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let max_packet_size = env::var("MAX_PACKET_SIZE")?.parse::<usize>()?;

    let video_path_str = prompt(prompts::VIDEO_PATH_PROMPT)?;

    VideoFileValidator::validate(&video_path_str)?;

    let mut video_file = File::open(video_path_str)?;

    let mut tcp_stream = TcpStream::connect(env::var("SERVER_ADDR")?)?;

    let mut buf = vec![0; max_packet_size];

    loop {
        let len = video_file.read(&mut buf)?;

        if len == 0 {
            break;
        }

        tcp_stream.write_all(&buf[..len])?;
    }

    Ok(())
}
