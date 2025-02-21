use std::{fs::File, net::TcpStream, path::PathBuf};

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
            .write_all(&packet.generate_header().generate_bytes())?;
        self.tcp_stream.write_all(&packet.json.generate_bytes())?;
        self.tcp_stream
            .write_all(&packet.media_type.generate_bytes())?;

        util::FileUploader::upload_file(
            &mut self.tcp_stream.stream,
            &mut File::open(&packet.payload.media_file_path)?,
        )?;

        Ok(())
    }

    pub fn receive_packet(&mut self, temp_file_path: PathBuf) -> anyhow::Result<(Packet, File)> {
        let header = PacketHeader::generate_from_bytes(
            &self
                .tcp_stream
                .receive_exact(PacketHeader::HEADER_SIZE_BYTES)?,
        )?;

        let json = Json::generate_from_bytes(&self.tcp_stream.receive_exact(header.json_size)?)?;

        let media_type = MediaType::generate_from_bytes(
            &self.tcp_stream.receive_exact(header.media_type_size)?,
        )?;

        let tmp_file = util::FileDownloader::download_file(
            &mut self.tcp_stream,
            &temp_file_path,
            header.payload_size,
            self.max_packet_size,
        )?;

        Ok((
            Packet {
                json,
                media_type,
                payload: Payload::new(temp_file_path)?,
            },
            tmp_file,
        ))
    }
}
