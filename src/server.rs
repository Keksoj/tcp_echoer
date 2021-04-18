use anyhow::Result;
use lib::{create_socket, generate_vector_of_strings, CustomFrame};
use rand::{thread_rng, Rng};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{debug, info, warn};

// this is almost a copy of https://tokio.rs/tokio/tutorial/io
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    loop {
        let socket = lib::create_socket();
        let listener = TcpListener::bind(&socket).await.unwrap();

        // we call it socket but it's a TcpStream, it implements AsyncRead and AsyncWrite
        let (mut socket, _) = listener.accept().await.unwrap();

        
        let mut buf = vec![0; 1024];

        let received_frame = read_stream(socket).await;

        // spawn a wait+send taks here, that would be rad
    }
    Ok(())
}

async fn read_stream(mut stream: TcpStream) -> anyhow::Result<CustomFrame> {
    let mut buf = vec![0; 1024];

    match stream.read(&mut buf).await {
        // Ok(0) means the remote has closed
        Ok(0) => anyhow::bail!("Nothing to read here"),
        Ok(n) => {
            let received_data = &buf[..n];
            let received_frame = CustomFrame::from_bytes(received_data);
            info!("Received frame with word: {}", received_frame.data);
            Ok(received_frame)
        }
        // same, little we  can do
        Err(reading_error) => anyhow::bail!("reading on the socket failed so much"),
    }
}

/*
tokio::spawn(async move {
    lib::random_sleep_up_to(2).await;
    let data_to_echo = &cloned_buffer[..n];
    let mut frame = lib::CustomFrame::from_bytes(data_to_echo);
    info!("Received frame with word: {}", frame.data);
    frame.mix_up();
    println!("Data to echo: {}", frame);
    // copy the data to the socket
    let socket = lib::create_socket();
    let mut stream = TcpStream::connect(socket).await.unwrap();

    if stream.write_all(&frame.to_bytes()).await.is_err() {
        // not mutch to do in case of an error
        anyhow::bail!("writing on the stream failed so much");
    }
    Ok(())
})
.await
*/
