use std::{
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    str::FromStr,
};

use shared::{app, mmp};

pub fn run() -> anyhow::Result<()> {
    let app_config = app::Config::new()?;

    let tcp_listener = TcpListener::bind(app_config.server_addr)?;

    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {
                let client_addr = tcp_stream.peer_addr()?;
                println!("Accepted connection from: {}", client_addr);

                let mut mmp_stream = mmp::Stream::new(tcp_stream, app_config.max_packet_size);

                if !Path::new(&app_config.download_dir).exists() {
                    fs::create_dir(&app_config.download_dir)?;
                }

                let temp_video_file_name = format!(
                    "{}_{}_{}.{}",
                    app_config.temp_file_name,
                    client_addr,
                    chrono::Local::now().format("%Y%m%d%H%M%S"),
                    "mp4"
                );

                let temp_file_path =
                    PathBuf::from_str(&app_config.download_dir)?.join(temp_video_file_name);

                let (packet, _temp_file) = mmp_stream.receive_packet(temp_file_path.clone())?;

                println!("Received packet: {:?}", packet);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
