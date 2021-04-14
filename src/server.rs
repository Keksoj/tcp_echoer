use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr}
    
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    time::{sleep, Duration},
};
use rand::{thread_rng, Rng};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142);
    let listener = TcpListener::bind(&socket).await.unwrap();

    loop {
        // we call it socket but it's a TcpStream, it implements AsyncRead and AsyncWrite
        let (mut socket, _) = listener.accept().await?;

        

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            random_sleep().await;

            loop {
                match socket.read(&mut buf).await {
                    // Ok(0) means the remote has closed
                    Ok(0) => return,
                    Ok(n) => {
                        println!("Data to echo: {:?}", &buf[..n]);
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

// sleeps from 0 to 256 milliseconds
async fn random_sleep() {
    let random_duration = rand::random::<u8>();
    println!("Sleeping for {} milliseconds", random_duration);
    let duration = Duration::from_millis(random_duration as u64);
    sleep(duration).await;
}