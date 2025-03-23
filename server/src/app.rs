mod command_processor;

use std::{
    fs::{self, remove_dir_all},
    net::TcpListener,
    path::Path,
};

use serde_json::json;
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

                let download_dir_path = Path::new(&app_config.download_dir).join(format!(
                    "{}_{}",
                    client_addr.to_string().replace(".", "_"),
                    chrono::Local::now().format("%Y%m%d%H%M%S"),
                ));

                if !download_dir_path.exists() {
                    fs::create_dir(&download_dir_path)?;
                }

                let input_file_path_without_ext = download_dir_path.join("input");
                let output_file_path_without_ext = download_dir_path.join("output");

                let received_packet = mmp_stream.receive_packet(&input_file_path_without_ext)?;

                println!("Received packet: {:?}", received_packet);

                let input_file_path: &Path = received_packet.payload.media_file_path.as_ref();
                let request_json: app::Request = serde_json::from_value(received_packet.json.data)?;

                let output_file_path = command_processor::CommandProcessor::process(
                    request_json.command,
                    input_file_path,
                    &output_file_path_without_ext,
                )?;

                let response_packet = mmp::Packet::new(
                    mmp::Json::new(json!(mmp::Response::new(mmp::Status::Ok)))?,
                    mmp::Payload::new(output_file_path.to_path_buf())?,
                );

                mmp_stream.send_packet(&response_packet)?;

                remove_dir_all(&download_dir_path)?;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
