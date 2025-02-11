use std::{
    fs::{self},
    io::Write,
    net::TcpListener,
    path::Path,
};

use shared::{app, protocol, util};

pub fn run() -> anyhow::Result<()> {
    let app_config = app::Config::new()?;

    let download_dir = app_config.download_dir;
    let video_file_name = app_config.video_file_name;

    let tcp_listener = TcpListener::bind(app_config.server_addr)?;

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

                match util::FileDownloader::download_file(&mut tcp_stream, &file_path) {
                    Ok(_) => tcp_stream
                        .write_all(&protocol::Response::new(protocol::Status::Ok).to_bytes())?,
                    Err(e) => {
                        tcp_stream.write_all(
                            &protocol::Response::new(protocol::Status::BadRequest).to_bytes(),
                        )?;

                        return Err(anyhow::anyhow!("Error downloading file: {}", e));
                    }
                };
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
