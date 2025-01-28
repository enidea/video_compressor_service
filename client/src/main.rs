use std::{io::Write, net::TcpStream};

use anyhow::Ok;

fn main() -> anyhow::Result<()> {
    let mut tcp_stream = TcpStream::connect(shared::SERVER_ADDR)?;

    tcp_stream.write_all(b"Hello, server!")?;

    Ok(())
}
