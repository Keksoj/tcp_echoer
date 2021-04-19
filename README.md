# TCP echoer

Let's try what we've learned with the tokio tutorials.
The goal is to have two binaries:

-   A client sends several "words" on the same TcpSender, and listens on it
-   A server listens and echoes each hello asynchronously, with different executing times per task

Run the server first:

    cargo run --bin server

And then the client, in a different terminal:

    cargo run --bin client

Survey file changes with [cargo watch](https://devjunhong.github.io/rust/cargo-watch/)

    cargo watch -x 'run --bin client'
    cargo watch -x 'run --bin server'


## What I want

I want the client to:

-   send all words at once, or well, almost at once, using the same TCP socket, one asynchronous task per word. (or maybe not?)
-   log every send when it happens
-   listens on the same TCP socket

I want the server to:

-   receive all words at once, or well, almost at once,
-   log the receiving of each word, and for each word, spawn a task that:
    -   sleeps for a random time between 0 and 2 seconds
    -   echoes the word back to the client on the same TCP socket
    -   log the sending of each echoed word

All echoing tasks would run simultaneously. This means that within 2 seconds, all words will be echoed in a random order.

Meanwhile, the client:

-   listens on the same TCP connection 
-   logs every incoming word

## How it's going now

Word sending is synchronous.

The client sends the words all right (does not listen yet), the server receives them (it seems), but when echoing them, the server "hears" them, and re-echoes them, and so on and so on.