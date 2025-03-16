use std::{fs::File, io::Write, path::Path};

use super::TcpStreamWrapper;

pub struct FileDownloader;

impl FileDownloader {
    pub fn download_file(
        tcp_stream: &mut TcpStreamWrapper,
        file_path: &Path,
        file_size: usize,
        max_packet_size: usize,
    ) -> anyhow::Result<File> {
        let mut file = File::create(file_path)?;
        let mut rest = file_size;

        loop {
            file.write_all(&tcp_stream.receive_exact(max_packet_size.min(rest))?)?;

            if rest <= max_packet_size {
                break;
            }

            rest -= max_packet_size;
        }

        Ok(file)
    }
}
