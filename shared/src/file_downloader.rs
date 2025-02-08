use std::{
    env,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
};

pub struct FileDownloader;

impl FileDownloader {
    pub fn download_file(tcp_stream: &mut TcpStream, file_path: &PathBuf) -> anyhow::Result<()> {
        dotenvy::dotenv()?;

        let max_packet_size = env::var("MAX_PACKET_SIZE")?.parse::<usize>()?;

        let mut file = File::create(file_path)?;

        let mut buf = vec![0; max_packet_size];

        loop {
            let len = tcp_stream.read(&mut buf)?;

            if len == 0 {
                break;
            }

            file.write_all(&buf[..len])?;
        }

        Ok(())
    }
}
