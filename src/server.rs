use anyhow::{Context, Result};
use lib::{create_socket, CustomFrame};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{debug, info};

// this is almost a copy of https://tokio.rs/tokio/tutorial/io
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    loop {
        let socket = create_socket();
        let listener = TcpListener::bind(&socket).await?;

        // we call it socket but it's a TcpStream, it implements AsyncRead and AsyncWrite
        let (mut socket, _) = listener.accept().await?;

        // receive a frame
        let mut buffer = vec![0; 1024];
        let mut frame: CustomFrame = match socket.read(&mut buffer).await {
            // Ok(0) means the remote has closed
            Ok(0) => anyhow::bail!("Nothing to read here"),
            Ok(n) => {
                let received_data = &buffer[..n];
                let received_frame = CustomFrame::from_bytes(received_data)?;
                info!("Received frame with word: {}", received_frame.data);
                received_frame
            }
            // same, little we  can do
            Err(reading_error) => anyhow::bail!("reading on the socket failed so much"),
        };

        // spawn a wait+send taks here, that would be rad

        tokio::spawn(async move {
            lib::random_sleep_up_to(2).await;
            frame.mix_up();
            info!("Data to echo: {}", frame);
            // copy the data to the socket
            let socket = lib::create_socket();

            // connect to the socket
            let mut stream = TcpStream::connect(socket)
                .await
                .context("Could not connect to the socket for sending a word.")
                .unwrap();

            // write the frame on the stream
            stream
                .write_all(&frame.to_bytes().unwrap())
                .await
                .context("writing on the stream failed so much")
                .unwrap();
        });
    }
    Ok(())
}

/*
async fn read_stream(mut stream: TcpStream, mut buffer: &Vec<u8>) -> anyhow::Result<CustomFrame> {
    let mut buf = vec![0; 1024];

    match stream.read(&mut  *buffer).await {
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
*/

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
