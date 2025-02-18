use std::{
    io::{Read, Write},
    net::TcpStream,
};

use super::Packet;

pub struct Stream {
    tcp_stream: TcpStream,
}

impl Stream {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self { tcp_stream }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> anyhow::Result<()> {
        self.tcp_stream.write_all(&packet.to_bytes())?;

        Ok(())
    }

    pub fn receive_packet(&mut self) -> anyhow::Result<Packet> {
        let mut buf = vec![0; 1024];

        let size = self.tcp_stream.read(&mut buf)?;

        let packet = Packet::from_bytes(&mut &buf[..size])?;

        Ok(packet)
    }
}
