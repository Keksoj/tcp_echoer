# tcp_echoer

Let's try what we've learned with the tokio tutorials.
The goal is to have two binaries:

- A client sends several "words" on the same TcpSender, and listens on it
- A server listens and echoes each hello asynchronously, with different executing times per task

    cargo run --bin client
    cargo run --bin server