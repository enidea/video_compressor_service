use std::{
    fs::{create_dir_all, remove_dir_all},
    net::TcpListener,
    path::Path,
};

use serde_json::json;

use crate::app::tcp_stream_handler::TcpStreamHandler;

pub struct VideoTranscoder;

impl VideoTranscoder {
    pub fn run() -> anyhow::Result<()> {
        let app_config = shared::app::Config::new()?;

        let tcp_listener = TcpListener::bind(app_config.server_addr.clone())?;

        for tcp_stream in tcp_listener.incoming() {
            match tcp_stream {
                Ok(tcp_stream) => {
                    let client_addr = tcp_stream.peer_addr()?;
                    println!("Accepted connection from: {}", client_addr);

                    let mut mmp_stream =
                        shared::mmp::Stream::new(tcp_stream, app_config.max_packet_size);

                    let download_dir_path = Path::new(&app_config.download_dir).join(format!(
                        "{}_{}",
                        client_addr.to_string().replace(".", "_"),
                        chrono::Local::now().format("%Y%m%d%H%M%S"),
                    ));

                    if !download_dir_path.exists() {
                        create_dir_all(&download_dir_path)?;
                    }

                    if let Err(error) =
                        TcpStreamHandler::handle(&mut mmp_stream, &download_dir_path)
                    {
                        println!("Error: {}", error);

                        let response_packet = shared::mmp::Packet::new(
                            shared::mmp::Json::new(json!(shared::mmp::Response::new(
                                shared::mmp::Status::BadRequest,
                                Some(error.to_string())
                            )))?,
                            None,
                        );

                        mmp_stream.send_packet(&response_packet)?;
                        remove_dir_all(&download_dir_path)?;
                    }
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }

        Ok(())
    }
}
