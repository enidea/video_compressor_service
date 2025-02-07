use std::{
    env,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

pub struct FileUploader;

impl FileUploader {
    pub fn upload_file(tcp_stream: &mut TcpStream, file: &mut File) -> anyhow::Result<()> {
        dotenvy::dotenv()?;

        let max_packet_size = env::var("MAX_PACKET_SIZE")?.parse::<usize>()?;

        let mut buf = vec![0; max_packet_size];

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
