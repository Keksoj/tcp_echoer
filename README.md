# tcp_echoer

Let's try what we've learned with the tokio tutorials.
The goal is to have two binaries:

- A client sends several "words" on the same TcpSender, and listens on it
- A server listens and echoes each hello asynchronously, with different executing times per task

Run the server first:

    cargo run --bin server

And then the client, in a different terminal:

    cargo run --bin client