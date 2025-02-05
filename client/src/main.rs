mod prompts;

use std::{
    env,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

fn prompt(message_prompt: &str) -> anyhow::Result<String> {
    println!("{}", message_prompt);

    let mut message = String::new();
    std::io::stdin().read_line(&mut message)?;

    if message.is_empty() {
        return Err(anyhow::anyhow!("Empty input"));
    }

    Ok(message.trim().to_string())
}

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let max_packet_size = env::var("MAX_PACKET_SIZE")?.parse::<usize>()?;

    let video_path_str = prompt(prompts::VIDEO_PATH_PROMPT)?;

    let video_path = Path::new(&video_path_str);

    if !video_path.exists() {
        return Err(anyhow::anyhow!("File does not exist"));
    }

    if video_path.extension().unwrap() == "mp4" {
        return Err(anyhow::anyhow!("Video file must be an mp4 file"));
    }

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
