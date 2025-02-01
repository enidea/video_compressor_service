use std::{
    fs::File,
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> anyhow::Result<()> {
    let tcp_listener = TcpListener::bind(shared::SERVER_ADDR)?;

    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(mut tcp_stream) => {
                println!("Accepted connection from: {}", tcp_stream.peer_addr()?);

                let mut file = File::create("download/video.mp4")?;
                let mut buf = [0; 4096];

                loop {
                    let len = tcp_stream.read(&mut buf)?;

                    if len == 0 {
                        break;
                    }

                    file.write_all(&buf[..len])?;
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
