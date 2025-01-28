use std::net::TcpListener;

fn main() -> anyhow::Result<()> {
    let tcp_listener = TcpListener::bind(shared::SERVER_ADDR)?;

    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {
                println!("Accepted connection from: {}", tcp_stream.peer_addr()?);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
