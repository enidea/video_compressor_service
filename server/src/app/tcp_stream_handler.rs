use super::command_processor;

use std::{fs::remove_dir_all, path::Path};

use serde_json::json;
use shared::{
    app,
    mmp::{self},
};
pub struct TcpStreamHandler;

impl TcpStreamHandler {
    pub fn handle(mmp_stream: &mut mmp::Stream, download_dir_path: &Path) -> anyhow::Result<()> {
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

        remove_dir_all(download_dir_path)?;

        Ok(())
    }
}
