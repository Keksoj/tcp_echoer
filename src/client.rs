use anyhow::{Context, Result};
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
    let socket = create_socket();

    let text = generate_vector_of_strings();
    info!("We will send those words by a TCP channel: {:?}", text);

    // this command manager receives frames from all other tasks,
    // send those frame to the TCP server,
    // listens on the TCP connection for an incoming frame from the server,
    // sends this returned frame to the commanding task
    tokio::spawn(async move {
        // receive command on the MCPSC receiving end
        while let Some(command) = mpsc_rx.recv().await {
            // launch a word send-and-receive task
            tokio::spawn(async move {
                // println!("Executing {:#?}", command);
                // executing the command
                let frame_to_send = command.frame;
                info!("[manager] trying to send: {}", frame_to_send.data);

                // send on TCP
                info!("[manager] Connecting to socket {:?}…", socket);

                // connect to the stream
                let mut stream = TcpStream::connect(socket)
                    .await
                    .context("[manager] could not connect to the socket")
                    .unwrap();
                info!("[manager] Connected! Writing on the stream…");

                // write the frame in the stream
                stream
                    .write_all(&frame_to_send.to_bytes().unwrap())
                    .await
                    .context("[manager] could not write the frame into the stream")
                    .unwrap();

                info!("[manager] Word is sent! listening on the socket…");

                // receive on the same socket
                let mut buf = vec![0; 1024];

                // receive on TCP
                // todo: check that the received frame matches the sent one
                info!("Waiting for a response…");
                match stream.read(&mut buf).await {
                    Ok(0) => anyhow::bail!("[manager] Nothing to read on the stream"),
                    Ok(bytes_read) => {
                        let received_data = &buf[..bytes_read];
                        info!("[manager] Received data: {:?}", received_data);
                        let received_frame = lib::CustomFrame::from_bytes(received_data)
                            .context("[manager] Parsing the buffer data went wrong")?;
                        // if frame_to_send.id == received_frame.id {
                        //     let _ = command.oneshot_tx.send(received_frame);
                        // }
                    }
                    Err(err) => anyhow::bail!("{}", err),
                }
                Ok(())
            });
        }
    });

    // manager.await?;
    info!("[main] so far so good");

    // let mut futures = Vec::new();
    for word in text.iter() {
        // clone the arguments passed to the asynchronous send_a_word()
        let cloned_word = word.clone();
        let cloned_mpsc_tx = mpsc_tx.clone();

        tokio::spawn(async move {
            let (oneshot_tx, oneshot_rx) = oneshot::channel();

            let frame = CustomFrame::from_str(&cloned_word);
            let command = Command {
                frame: frame.clone(),
                oneshot_tx,
            };
            info!(
                "[word] Sending a command for the frame: {:?}",
                command.frame.data
            );
            // send the command to the manager task
            cloned_mpsc_tx
                .send(command)
                .await
                .context("[word] Could not send the command to the task manager")
                .unwrap();

            // receive a frame from the manager
            let returned_frame = oneshot_rx
                .await
                .context(
                    "[word] Could not receive a frame from the task manager on the oneshot channel",
                )
                .unwrap();
            info!(
                "[word] For the frame {}  we received the frame: {}\n",
                frame.data, returned_frame.data
            );
        });

        // futures.push(send_a_word);
    }

    info!("[main] so far so good");
    // let futures = make_a_list_of_futures(text, mpsc_tx);
    // launch all word-sending futures
    // for future in futures {
    //     // info!("Launching a future");

    //     future.await;
    // }

    Ok(())
}

fn make_a_list_of_futures(
    text: Vec<String>,
    mpsc_tx: Sender<Command>,
) -> Vec<impl std::future::Future> {
    let mut futures = Vec::new();

    for word in text.iter() {
        // clone the arguments passed to the asynchronous send_a_word()
        let cloned_word = word.clone();
        let cloned_mpsc_tx = mpsc_tx.clone();

        let send_a_word = send_a_word(cloned_mpsc_tx, cloned_word);

        futures.push(send_a_word);
    }
    futures
}

async fn send_a_word(mpsc_tx: Sender<Command>, word: String) -> anyhow::Result<()> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel();
    let frame = CustomFrame::from_str(&word);
    let command = Command {
        frame: frame.clone(),
        oneshot_tx,
    };
    info!(
        "[word] Sending a command for the frame: {:?}",
        command.frame.data
    );

    // send the command to the manager task
    mpsc_tx
        .send(command)
        .await
        .context("[word] Could not send the command to the task manager")?;

    // receive a bool from the manager
    let returned_frame = oneshot_rx
        .await
        .context("[word] Could not receive a frame from the task manager on the oneshot channel")?;

    info!(
        "[word] For the frame {}  we received the frame: {}\n",
        frame.data, returned_frame.data
    );
    Ok(())
}
