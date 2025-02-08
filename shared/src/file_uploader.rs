use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

use crate::config_loader;

pub struct FileUploader;

impl FileUploader {
    pub fn upload_file(tcp_stream: &mut TcpStream, file: &mut File) -> anyhow::Result<()> {
        let app_config = config_loader::load_config()?;

        let mut buf = vec![0; app_config.max_packet_size];

        loop {
            let len = file.read(&mut buf)?;

            if len == 0 {
                break;
            }

            tcp_stream.write_all(&buf[..len])?;
        }

        Ok(())
    }
}
