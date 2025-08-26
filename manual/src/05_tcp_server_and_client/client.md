# The Client

The client is the simplest part. If you've done synchronous TCP programming before, this will look familiar.

> The part that *always* trips me up are Tokio's `AsyncReadExt` and `AsyncWriteExt` traits. I don't think I've ever remembered that I need them until the compiler yells at me.

```rust
// Don't be Herbert. Remember these:
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    tokio::spawn(server());
    // Give the server time to start. You don't actually need this!
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.25)).await;
    client().await;
}

async fn server() {
    // Server code will go here
}

async fn client() {
    let mut socket = tokio::net::TcpStream::connect("127.0.0.1:3001").await.unwrap();
    socket.write_all(b"Hello, world!").await.unwrap();
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await.unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
}
```

The basic flow is pretty simple: you connect to the server, send the bytes "Hello, world!", and then read the response back. The server will echo back whatever you send it.

There *is* no server, so this will currently crash:

```
thread 'main' panicked at src/main.rs:18:77:
called `Result::unwrap()` on an `Err` value: Os { code: 111, kind: ConnectionRefused, message: "Connection refused" }
```

That's actually good - you can see it *tried*!