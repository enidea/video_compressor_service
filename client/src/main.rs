mod prompts;

use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

fn prompt(message_prompt: &str) -> anyhow::Result<String> {
    println!("{}", message_prompt);

    let mut message = String::new();
    std::io::stdin().read_line(&mut message)?;

    if message.is_empty() {
        return Err(anyhow::anyhow!("Empty input"));
    }

    Ok(message)
}

fn main() -> anyhow::Result<()> {
    let video_path: String = prompt(prompts::VIDEO_PATH_PROMPT)?.trim().to_string();

    let mut video_file = File::open(video_path)?;

    let mut tcp_stream = TcpStream::connect(shared::SERVER_ADDR)?;

    let mut buf = [0; 4096];

    loop {
        let len = video_file.read(&mut buf)?;

        if len == 0 {
            break;
        }

        tcp_stream.write_all(&buf[..len])?;
    }

    Ok(())
}
