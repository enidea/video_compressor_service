use std::{fs::File, net::TcpStream, path::Path};

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

        if let Some(media_type) = &packet.media_type {
            self.tcp_stream.write_all(&media_type.generate_bytes())?;
        }

        if let Some(payload) = &packet.payload {
            util::FileUploader::upload_file(
                &mut self.tcp_stream.stream,
                &mut File::open(&payload.media_file_path)?,
            )?;
        }

        Ok(())
    }

    pub fn receive_packet(&mut self, file_path: &Path) -> anyhow::Result<Packet> {
        let header = PacketHeader::generate_from_bytes(
            &self
                .tcp_stream
                .receive_exact(PacketHeader::HEADER_SIZE_BYTES)?,
        )?;

        println!("Header: {:?}", header);

        let json =
            Json::generate_from_bytes(&self.tcp_stream.receive_exact(header.json_size as usize)?)?;

        let media_type_bytes = self
            .tcp_stream
            .receive_exact(header.media_type_size as usize)?;

        if media_type_bytes.is_empty() {
            return Ok(Packet::new(json, None));
        }

        let media_type = MediaType::generate_from_bytes(&media_type_bytes)?;

        let file_path_with_extension = file_path.with_extension(media_type.to_string());

        util::FileDownloader::download_file(
            &mut self.tcp_stream,
            &file_path_with_extension,
            header.payload_size as usize,
            self.max_packet_size,
        )?;

        Ok(Packet::new(
            json,
            Some(Payload::new(file_path_with_extension.to_path_buf())?),
        ))
    }
}
