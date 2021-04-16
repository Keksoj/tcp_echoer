use anyhow::Result;
use futures::future::{join_all, JoinAll};
use lib::{create_socket, generate_vector_of_strings, CustomFrame};
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{
        // channels
        mpsc,    // Multiple Producer, Single Consumer
        oneshot, // one producer, one receiver
    },
};

#[derive(Debug)]
struct Command {
    frame: CustomFrame,
    oneshot_tx: oneshot::Sender<CustomFrame>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (mpsc_tx, mpsc_rx) = mpsc::channel(32);

    let text = generate_vector_of_strings();
    println!("We will send those words by a TCP channel: {:?}", text);

    manager(mpsc_rx).await?;
    for word in text.iter() {
        send_a_word(word.clone(), mpsc_tx.clone()).await?;
    }


    Ok(())
}

async fn send_a_word(word: String, mpsc_tx: mpsc::Sender<Command>) -> anyhow::Result<()> {
    tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();

        let command = Command {
            frame: CustomFrame::from_str(&word),
            oneshot_tx,
        };
        println!("Sending the frame: {:#?}", command.frame);

        // send the command to the manager task
        mpsc_tx.send(command).await?;

        // receive a bool from the manager
        let returned_frame = oneshot_rx.await?;

        println!(
            "The manager task succeeded in sending the word: {}",
            returned_frame.data
        );
        Ok(())
    })
    .await?
}

// the manager receives commands from other tasks on the MPSC receiver
async fn manager(mut mpsc_rx: mpsc::Receiver<Command>) -> anyhow::Result<()> {
    println!("Sending the frame: {:#?}", command.frame);

    while let Some(command) = mpsc_rx.recv().await {
        println!("Executing {:#?}", command);

        // executing the command
        let returned_frame = tcp_send_and_receive(command.frame).await?;

        let _ = command.oneshot_tx.send(returned_frame);
    }
    Ok(())
}

async fn tcp_send_and_receive(frame_to_send: CustomFrame) -> anyhow::Result<CustomFrame> {
    tokio::spawn(async move {
        let socket = create_socket();

        // send
        println!("Connecting to socket {:?}…", socket);

        let mut stream = TcpStream::connect(socket).await?;
        stream.write_all(&frame_to_send.to_bytes()).await?;

        // receive on the same socket - how?
        let mut buf = vec![0; 1024];

        loop {
            match stream.read(&mut buf).await {
                Ok(bytes_read) => {
                    let received_data = &buf[..bytes_read];
                    let received_frame = lib::CustomFrame::from_bytes(received_data);
                    if frame_to_send.id == received_frame.id {
                        return Ok(received_frame);
                    }
                }
                Err(_) => {}
            }
        }
    })
    .await?
}
