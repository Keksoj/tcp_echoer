use anyhow::Result;
use futures::future::{join_all, JoinAll};
use lib::{create_socket, generate_vector_of_strings, CustomFrame};
use std::error::Error;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{
        // channels
        mpsc::{self, Receiver, Sender}, // Multiple Producer, Single Consumer
        oneshot,                        // one producer, one receiver
    },
    task::JoinHandle,
};
use tracing::{debug, info, warn};

#[derive(Debug)]
struct Command {
    frame: CustomFrame,
    oneshot_tx: oneshot::Sender<CustomFrame>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let (mpsc_tx, mut mpsc_rx): (Sender<Command>, Receiver<Command>) = mpsc::channel(32);

    let text = generate_vector_of_strings();
    info!("We will send those words by a TCP channel: {:?}", text);

    // this command manager receives frames from all other tasks,
    // send those frame to the TCP server,
    // listens on the TCP connection for an incoming frame from the server,
    // sends this returned frame to the commanding task
    let manager = tokio::spawn(async move {
        while let Some(command) = mpsc_rx.recv().await {
            // println!("Executing {:#?}", command);
            // executing the command

            let socket = create_socket(); 
            let frame_to_send = command.frame;

            // send on TCP
            info!("Connecting to socket {:?}…", socket);
            let mut stream = TcpStream::connect(socket).await.unwrap();
            info!("Connected.");
            stream.write_all(&frame_to_send.to_bytes()).await.unwrap();

            // receive on the same socket
            let mut buf = vec![0; 1024];

            // receive on TCP
            // todo: check that the received frame matches the sent one
            match stream.read(&mut buf).await {
                Ok(bytes_read) => {
                    let received_data = &buf[..bytes_read];
                    let received_frame = lib::CustomFrame::from_bytes(received_data);
                    if frame_to_send.id == received_frame.id {
                        let _ = command.oneshot_tx.send(received_frame);
                    }
                }
                Err(err) => anyhow::bail!("{}", err),
            }
            // Ok(())
        }
        Ok(())
    });

    info!("so far so good");

    // for each word in the text,
    // spawn a task that builds a frame around it and sends it to the manager
    // Gather those tasks in a vector
    let mut list_of_word_sending_futures = Vec::new();
    for word in text.iter() {
        // clone the arguments passed to the task
        let cloned_word = word.clone();
        let cloned_mpsc_tx = mpsc_tx.clone();

        // create the task handle
        let sending_task: JoinHandle<()> = tokio::spawn(async move {
            let (oneshot_tx, oneshot_rx) = oneshot::channel();
            let frame = CustomFrame::from_str(&cloned_word);
            let command = Command {
                frame: frame.clone(),
                oneshot_tx,
            };
            info!("Sending the frame containing: {:?}", command.frame.data);
            // send the command to the manager task
            cloned_mpsc_tx.send(command).await.unwrap();
            // receive a bool from the manager
            let returned_frame = oneshot_rx.await.unwrap();
            info!(
                "For the frame\n{}\n we received the frame: \n{}\n", 
                frame, returned_frame
            );
        });

        // push the task to the list
        list_of_word_sending_futures.push(sending_task);
    }

    // launch all word-sending tasks
    for future in list_of_word_sending_futures {
        future.await?;
    }

    // launch the manager
    manager.await??;

    Ok(())
}
