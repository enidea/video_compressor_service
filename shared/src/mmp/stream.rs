use std::{
    fs::File,
    net::TcpStream,
    path::{Path, PathBuf},
};

use crate::util;

use super::{packet::PacketHeader, Json, MediaType, Packet, Payload};

pub struct Stream {
    max_packet_size: usize,
    tcp_stream: util::TcpStreamWrapper,
}

impl Stream {
    pub fn new(tcp_stream: TcpStream, max_packet_size: usize) -> Self {
        Self {
            max_packet_size,
            tcp_stream: util::TcpStreamWrapper::new(tcp_stream),
        }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> anyhow::Result<()> {
        self.tcp_stream
            .write_all(&packet.generate_header()?.generate_bytes())?;
        self.tcp_stream.write_all(&packet.json.generate_bytes())?;
        self.tcp_stream
            .write_all(&packet.media_type.generate_bytes())?;

        util::FileUploader::upload_file(
            &mut self.tcp_stream.stream,
            &mut File::open(&packet.payload.media_file_path)?,
        )?;

        Ok(())
    }

    pub fn receive_packet(&mut self, file_path: &Path) -> anyhow::Result<(Packet, PathBuf)> {
        let header = PacketHeader::generate_from_bytes(
            &self
                .tcp_stream
                .receive_exact(PacketHeader::HEADER_SIZE_BYTES)?,
        )?;

        println!("Header: {:?}", header);

        let json =
            Json::generate_from_bytes(&self.tcp_stream.receive_exact(header.json_size as usize)?)?;

        let media_type = MediaType::generate_from_bytes(
            &self
                .tcp_stream
                .receive_exact(header.media_type_size as usize)?,
        )?;

        let file_path_with_extension = file_path.with_extension(media_type.to_string());

        util::FileDownloader::download_file(
            &mut self.tcp_stream,
            &file_path_with_extension,
            header.payload_size as usize,
            self.max_packet_size,
        )?;

        Ok((
            Packet {
                json,
                media_type,
                payload: Payload::new(file_path.to_path_buf())?,
            },
            file_path_with_extension,
        ))
    }
}
