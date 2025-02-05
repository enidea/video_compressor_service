use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    net::TcpListener,
    path::Path,
};

pub fn run() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let download_dir = env::var("DOWNLOAD_DIR")?;
    let video_file_name = env::var("VIDEO_FILE_NAME")?;
    let max_packet_size = env::var("MAX_PACKET_SIZE")?.parse::<usize>()?;

    let tcp_listener = TcpListener::bind(env::var("SERVER_ADDR")?)?;

    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(mut tcp_stream) => {
                println!("Accepted connection from: {}", tcp_stream.peer_addr()?);

                if !Path::new(&download_dir).exists() {
                    fs::create_dir(&download_dir)?;
                }

                let file_name = format!(
                    "{}_{}.{}",
                    video_file_name,
                    chrono::Local::now().timestamp(),
                    "mp4"
                );

                let file_path = Path::new(&download_dir).join(file_name);

                let mut file = File::create(file_path)?;
                let mut buf = vec![0; max_packet_size];

                loop {
                    let len = tcp_stream.read(&mut buf)?;

                    if len == 0 {
                        break;
                    }

                    file.write_all(&buf[..len])?;
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
