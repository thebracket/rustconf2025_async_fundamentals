use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc;

// ==========================
// SERVER
// ==========================
async fn calculator_task(tx: mpsc::Sender<String>) {
    // Simulate a long calculation
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    let _ = tx.send("Calculation complete!\n".to_string()).await;
}

async fn server() {
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("Server listening on 127.0.0.1:3001");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(socket: TcpStream) {
    // Split socket into read/write halves
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    // Channel for sending messages to the write half
    let (reply_tx, mut reply_rx) = mpsc::channel::<String>(32);

    // Spawn a task to write messages from the channel
    let write_task = tokio::spawn(async move {
        while let Some(msg) = reply_rx.recv().await {
            println!("Server sending: {}", msg.trim());
            if let Err(e) = writer.write_all(msg.as_bytes()).await {
                eprintln!("Write error: {e}");
                break;
            }
        }
    });

    // Read loop
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line).await.unwrap();
        if n == 0 {
            break; // connection closed
        }

        let command = line.trim().to_lowercase();
        match command.as_str() {
            "calculate" => {
                println!("Server received: calculate");
                let tx_clone = reply_tx.clone();
                tokio::spawn(calculator_task(tx_clone));
            }
            "hello" => {
                println!("Server received: hello");
                let _ = reply_tx.send("Hello to you too!\n".to_string()).await;
            }
            _ => {
                println!("Server received unknown command: {}", command);
            }
        }
    }

    // Close writer task
    drop(reply_tx);
    write_task.await.unwrap();
}

// ==========================
// CLIENT
// ==========================
async fn client() {
    let socket = TcpStream::connect("127.0.0.1:3001").await.unwrap();
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    // Task to read from the server continuously
    let read_task = tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => print!("Client received: {}", line),
                Err(e) => {
                    eprintln!("Client read error: {e}");
                    break;
                }
            }
        }
    });

    // Task to send messages without blocking
    let write_task = tokio::spawn(async move {
        writer.write_all(b"calculate\n").await.unwrap();
        println!("Client sent: calculate");

        // Do something else while the calculation is in progress
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        writer.write_all(b"hello\n").await.unwrap();
        println!("Client sent: hello");
    });

    let _ = tokio::join!(read_task, write_task);
}

// ==========================
// MAIN
// ==========================
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Start server
    tokio::spawn(server());

    // Wait a bit for server to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Run client
    client().await;
}
