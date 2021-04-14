use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142);
    let mut stream = TcpStream::connect(socket).await?;

    stream.write_all(b"hello").await?;

    Ok(())
}
