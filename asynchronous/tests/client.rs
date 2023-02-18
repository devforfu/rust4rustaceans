use std::io::Write;
use std::net;

fn main() -> std::io::Result<()> {
    let addr = "localhost:8080";
    {
        let mut stream = net::TcpStream::connect(addr)?;
        stream.write_all(b"Hello, world!")?;
    }
    Ok(())
}

fn start_server() {}
