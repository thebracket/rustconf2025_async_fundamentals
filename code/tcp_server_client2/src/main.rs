use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(server());
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.25)).await; // Give the server time to start

    let tasks = (0..500).map(|_| {
        tokio::spawn(async {
            client().await;
        })
    });
    futures::future::join_all(tasks).await;
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