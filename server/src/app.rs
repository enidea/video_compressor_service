use std::{fs, net::TcpListener, path::Path};

use shared::{
    app,
    mmp::{self},
};

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

                let temp_file_path = Path::new(&app_config.download_dir).join(temp_video_file_name);

                let (received_packet, temp_file) = mmp_stream.receive_packet(&temp_file_path)?;

                println!("Received packet: {:?}", received_packet);

                let request_json: app::RequestJson =
                    serde_json::from_value(received_packet.json.data)?;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
