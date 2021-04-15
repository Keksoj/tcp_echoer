use rand::{thread_rng, Rng};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    // time::{sleep, Duration},
};


// this is almost a copy of https://tokio.rs/tokio/tutorial/io
#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = lib::create_socket();
    let listener = TcpListener::bind(&socket).await.unwrap();

    loop {
        // we call it socket but it's a TcpStream, it implements AsyncRead and AsyncWrite
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            lib::random_sleep().await;

            loop {
                match socket.read(&mut buf).await {
                    // Ok(0) means the remote has closed
                    Ok(0) => return,
                    Ok(n) => {
                        let data_to_echo = &buf[..n];
                        let frame = lib::CustomFrame::from_bytes(data_to_echo);
                        println!("Data to echo: {}", frame);
                        // copy the data to the socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // not mutch to do in case of an error
                            return;
                        }
                    }
                    // same, little we  can do
                    Err(_) => return,
                }
            }
        });
    }
}
