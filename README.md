# tcp_echoer

Let's try what we've learned with the tokio tutorials.
The goal is to have two binaries:

- A client sends several "hellos" on the same TcpSender 
- A server listens and echoes each hello asynchronously

    cargo run --bin client
    cargo run --bin server