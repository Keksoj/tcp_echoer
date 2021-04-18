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

## What I want

I want the client to:

-   send all words at once, or well, almost at once, using the same TCP socket, one asynchronous task per word.
-   log every send when it happens

I want the server to:

-   receive all words at once, or well, almost at once,
-   log the receiving of each word, and for each word, spawn a task that:
    -   sleeps for a randow time between 0 and 2 seconds
    -   echo the word
    -   log the sending of each echoed word

This means that within 2 seconds, all words will be echoed in a random order.

The client:

-   listens on the same TCP connection and logs