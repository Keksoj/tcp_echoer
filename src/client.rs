use lib::CustomFrame;
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
    let socket = lib::create_socket();

    // I want this to listen
    let listener = TcpListener::bind(&socket).await.unwrap();

    let stream = TcpStream::connect(socket).await.unwrap();

    let text = vec![
        "When",
        "shall",
        "we",
        "three",
        "meet",
        "again?", // Did you read Macbeth?
        "In",
        "thunder,",
        "lightning,",
        "or",
        "in",
        "rain?",
    ];

    println!("We will send those words by a TCP channel: {:?}", text);

    let send_a_word_task = send_a_word(stream, text[0].to_string());
    send_a_word_task.await;
    Ok(())
}

async fn send_a_word(mut stream: TcpStream, word: String) {
    tokio::spawn(async move {
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
