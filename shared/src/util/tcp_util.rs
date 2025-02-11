use super::config_loader;
use std::io::Read;

pub struct TcpUtil;

impl TcpUtil {
    pub fn read_bytes(tcp_stream: &mut std::net::TcpStream) -> anyhow::Result<Vec<u8>> {
        let app_config = config_loader::load_config()?;

        let mut buf = vec![0; app_config.max_packet_size];

        let len = tcp_stream.read(&mut buf)?;

        Ok(buf[..len].to_vec())
    }
}
