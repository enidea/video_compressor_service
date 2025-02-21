use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct TcpStreamWrapper {
    pub stream: TcpStream,
}

impl TcpStreamWrapper {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn write_all(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        self.stream.write_all(buf)?;

        Ok(())
    }

    pub fn receive_exact(&mut self, byte_size: usize) -> anyhow::Result<Vec<u8>> {
        let mut buf = vec![0; byte_size];

        self.stream.read_exact(&mut buf)?;

        Ok(buf)
    }
}
