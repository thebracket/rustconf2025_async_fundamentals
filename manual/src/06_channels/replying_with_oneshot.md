# Replying with OneShot

Sometimes, sending a message to another task isn't enough - you want a reply. OneShot channels are perfect for this.

OneShot channels are designed to send a single value from one task to another.

```rust
use tokio::sync::{mpsc, oneshot};

enum Command {
    GetData(oneshot::Sender<String>),
    // You can add more commands here
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let server_task = tokio::spawn(async move {
        while let Some(command) = rx.recv().await {
            match command {
                Command::GetData(reply_tx) => {
                    // Simulate some data fetching
                    let data = "Here is your data".to_string();
                    // Send the data back through the oneshot channel
                    let _ = reply_tx.send(data);
                }
            }
        }
    });

    // Client task
    let client_task = tokio::spawn(async move {
        let (reply_tx, reply_rx) = oneshot::channel();
        if let Err(e) = tx.send(Command::GetData(reply_tx)).await {
            eprintln!("Failed to send command: {}", e);
            return;
        }
        match reply_rx.await {
            Ok(data) => println!("Received data: {}", data),
            Err(e) => eprintln!("Failed to receive data: {}", e),
        }
    });
    let _ = tokio::join!(server_task, client_task);
}
```

> Congratulations! You just built the simplest form of the actor model. We'll talk more about this later.