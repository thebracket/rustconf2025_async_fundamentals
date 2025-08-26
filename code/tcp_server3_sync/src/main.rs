use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(server());
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.25)).await; // Give the server time to start

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

                // Take the data and turn it into a string
                let command = String::from_utf8_lossy(&buf[..n]).trim().to_lowercase();
                match command.as_str() {
                    "calculate" => {
                        tokio::time::sleep(std::time::Duration::from_secs_f32(0.25)).await;
                        socket
                            .write_all(b"Calculation complete!\n")
                            .await
                            .unwrap();
                    }
                    "hello" => {
                        socket.write_all(b"Hello to you too!\n").await.unwrap();
                    }
                    _ => { /* No-op */ }
                }
            }
        });
    }
}

async fn client() {
    let mut socket = tokio::net::TcpStream::connect("127.0.0.1:3001").await.unwrap();
    socket.write_all(b"calculate").await.unwrap();
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await.unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

    socket.write_all(b"hello").await.unwrap();
    let n = socket.read(&mut buf).await.unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));    
}