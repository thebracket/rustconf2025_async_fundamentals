# The Server

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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            while let Ok(n) = socket.read(buf.as_mut()).await {
                if n == 0 {
                    return; // Connection closed
                }
                socket.write_all(&buf[..n]).await.unwrap();
            }
        });
    }
}

async fn client() {
    let mut socket = tokio::net::TcpStream::connect("127.0.0.1:3001").await.unwrap();
    socket.write_all(b"Hello, world!").await.unwrap();
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await.unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
}
```

---

Let's break down the server code:

```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
```

We create a TCP listener (binding with the sockets system).

Now we loop forever, accepting incoming connections:

```rust
loop {
    let (mut socket, _) = listener.accept().await.unwrap();
```

We won't be consuming CPU time while waiting for connections; the combination of Tokio's reactor and the OS will handle that for us. Notice how it returns two values: the socket and the address of the client. We don't need the address, so we use `_` to ignore it.

Once we have a connection, we spawn a new task to handle it:

```rust
tokio::spawn(async move {
```

> It's common to use a function here.

We read from the socket in a loop, echoing back whatever we receive:

```rust
let mut buf = [0; 1024];
while let Ok(n) = socket.read(buf.as_mut()).await {
    if n == 0 {
        return; // Connection closed
    }
    socket.write_all(&buf[..n]).await.unwrap();
}
```

The "accept/spawn" pattern is *very* common - and very performant. Network programs spend most of their time waiting for I/O - so this pattern can handle thousands of connections with very little overhead.

You could easily loop and spawn thousands of clients for your one server. Let's try that!

> The code is in the `code/tcp_server_client2` directory.
