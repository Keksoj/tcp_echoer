use futures::future::{join_all, JoinAll};
use lib::{create_socket, generate_vector_of_strings, CustomFrame};
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{
        // channels
        mpsc,    // Multiple Producer, Single Consumer
        oneshot, // one producer, one receiver
    },
};

#[derive(Debug)]
struct Command {
    frame: CustomFrame,
    oneshot_tx: oneshot::Sender<bool>, // so the manager either sends a true or false
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (mpsc_tx, mpsc_rx) = mpsc::channel(32);

    let text = generate_vector_of_strings();
    println!("We will send those words by a TCP channel: {:?}", text);

    for word in text.iter() {
        send_a_word(word.clone(), mpsc_tx.clone()).await;
    }

    manager(mpsc_rx).await;

    Ok(())
}

async fn send_a_word(word: String, mpsc_tx: mpsc::Sender<Command>) {
    tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();

        let command = Command {
            frame: CustomFrame::from_str(&word),
            oneshot_tx,
        };

        // send the command to the manager task
        if mpsc_tx.send(command).await.is_err() {
            println!("Could not send {}", word);
        };

        // receive a bool from the manager
        let ok_from_the_manager = oneshot_rx.await.unwrap();

        println!(
            "The manager task succeeded in sending the word: {}",
            ok_from_the_manager
        );
    });
}

// the manager receives commands from other tasks on the MPSC receiver
async fn manager(mut mpsc_rx: mpsc::Receiver<Command>) {
    while let Some(command) = mpsc_rx.recv().await {
        println!("Executing {:#?}", command);

        // executing the command
        let returned_frame = tcp_send_and_receive(command.frame).await;

        let _ = command.oneshot_tx.send(returned_frame);
    }
}

async fn tcp_send_and_receive(frame_to_send: CustomFrame) 
// -> Result<CustomFrame, dyn Error> 
{
    tokio::spawn(async move {
        let socket = create_socket();

        // send
        let mut stream = TcpStream::connect(socket).await.unwrap();
        stream.write_all(&frame_to_send.to_bytes()).await.unwrap();

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
                Err(error) => return Err(error),
            }
        }
    });
    
}
