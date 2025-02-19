use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::app;

use super::Packet;

pub struct Stream {
    max_packet_size: usize,
    tcp_stream: TcpStream,
}

impl Stream {
    pub fn new(tcp_stream: TcpStream) -> Self {
        let max_packet_size = app::Config::new().unwrap().max_packet_size;

        Self {
            max_packet_size,
            tcp_stream,
        }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> anyhow::Result<()> {
        self.tcp_stream.write_all(&packet.generate_bytes());

        Ok(())
    }

    pub fn receive_packet(&mut self) -> anyhow::Result<Packet> {
        let mut buf = vec![0; 1024];

        let size = self.tcp_stream.read(&mut buf)?;

        let packet = Packet::from_bytes(&mut &buf[..size])?;

        Ok(packet)
    }
}
