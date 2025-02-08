use std::{
    env,
    fs::{self},
    net::TcpListener,
    path::Path,
};

use shared::FileDownloader;

pub fn run() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let download_dir = env::var("DOWNLOAD_DIR")?;
    let video_file_name = env::var("VIDEO_FILE_NAME")?;

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
                    chrono::Local::now().format("%Y%m%d%H%M%S"),
                    "mp4"
                );

                let file_path = Path::new(&download_dir).join(file_name);

                FileDownloader::download_file(&mut tcp_stream, &file_path)?;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
