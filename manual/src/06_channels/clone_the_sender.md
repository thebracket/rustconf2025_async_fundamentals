# Clone the Sender

The "M" in "MPSC" stands for "multiple". *Multiple" senders, *Single* receiver. You can have as many senders as you want - and when they close/drop, the receiver will still work until all senders are gone. When they are all gone, the receiver will return `None` from `recv()`.

```rust
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create the channel
    //      Split the (transmitter, receiver)
    //      Buffer size of 32 messages
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn a task to send messages
    let mut tasks = Vec::new();
    for task_id in 0..2 {
        // We can clone the sender whenever we want to.
        let tx_clone = tx.clone();
        let task = tokio::spawn(async move {
            for i in 0..5 {
                let message = format!("Task {} - Message {}", task_id, i);
                if let Err(e) = tx_clone.send(message).await {
                    eprintln!("Failed to send message: {}", e);
                    return;
                }
            }
        });
        tasks.push(task);
    }

    // Really common bug alert!
    drop(tx); // Drop the original sender to avoid hanging the receiver.

    // Loop to receive messages
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
    println!("All senders have been dropped, receiver is done.");
}
```

> Go is notorious for making you check that the channel is closed, and the "defer" system can make it quite unclear when the channel actually went away. Rust makes it very explicit when the channel is closed - when all senders are dropped.