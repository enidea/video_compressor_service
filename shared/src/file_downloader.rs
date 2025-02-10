use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
};

use crate::config_loader;

pub struct FileDownloader;

impl FileDownloader {
    pub fn download_file(tcp_stream: &mut TcpStream, file_path: &PathBuf) -> anyhow::Result<File> {
        let app_config = config_loader::load_config()?;

        let mut file = File::create(file_path)?;

        let mut buf = vec![0; app_config.max_packet_size];

        loop {
            let len = tcp_stream.read(&mut buf)?;

            if len < app_config.max_packet_size {
                break;
            }

            file.write_all(&buf[..len])?;
        }

        Ok(file)
    }
}
