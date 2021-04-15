use futures::future::join_all;
use lib::{create_socket, generate_vector_of_strings, CustomFrame};
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::{
        // channels
        mpsc,    // Multiple Producer, Single Consumer
        oneshot, // one producer, one receiver
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = create_socket();

    // I want this to listen
    // let listener = TcpListener::bind(&socket).await.unwrap();

    let stream = TcpStream::connect(socket).await.unwrap();

    let text = generate_vector_of_strings();

    println!("We will send those words by a TCP channel: {:?}", text);

    let list_of_futures = send_a_text(stream, text).await;
    println!("We have created a list of futures");

    join_all(list_of_futures).await;

    Ok(())
}

async fn send_a_text(stream: TcpStream, text: Vec<String>) -> Vec<tokio::task::JoinHandle<()>> {
    let mut futures = vec![];

    for word in text.iter() {
        println!("Let's print {}", word);
        let socket = create_socket();
        let stream = TcpStream::connect(socket).await.unwrap();
        let word = word.clone();
        futures.push(tokio::spawn(send_a_word(stream, word)));
    }
    futures
}

async fn send_a_word(mut stream: TcpStream, word: String) {
    tokio::spawn(async move {
        println!("Let's print {}", word);

        let frame = CustomFrame::from_str(&word);
        stream.write_all(&frame.to_bytes()).await.unwrap();
    });
}

// not yet in use
async fn listen() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142);
    let listener = TcpListener::bind(&socket).await.unwrap();
}

// implement the same channels between tasks as in https://tokio.rs/tokio/tutorial/channels
// a sending handle of a oneshot channel
type Responder<T> = oneshot::Sender<T>;

struct Command {
    frame: CustomFrame,
    resp_tx: Responder<()>,
}
