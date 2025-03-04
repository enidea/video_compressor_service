use crate::app;
use std::io::Read;

pub struct TcpUtil;

impl TcpUtil {
    pub fn read_bytes(tcp_stream: &mut std::net::TcpStream) -> anyhow::Result<Vec<u8>> {
        let app_config = app::Config::new()?;

        let mut buf = vec![0; app_config.max_packet_size];

        let len = tcp_stream.read(&mut buf)?;

        Ok(buf[..len].to_vec())
    }
}
