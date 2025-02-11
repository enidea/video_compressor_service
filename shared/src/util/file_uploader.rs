use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

use crate::app;

pub struct FileUploader;

impl FileUploader {
    pub fn upload_file(tcp_stream: &mut TcpStream, file: &mut File) -> anyhow::Result<()> {
        let app_config = app::Config::new()?;

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
