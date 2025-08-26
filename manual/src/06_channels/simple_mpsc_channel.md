# Simple MPSC Channel

Let's go ahead and build a simple MPSC channel. Rather than using the standard library's channel, we'll use Tokio's built-in version - which is very similar.

This won't be the most useful channel ever, it's designed to illustrate the basic concepts.

```rust
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create the channel
    //      Split the (transmitter, receiver)
    //      Buffer size of 32 messages
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn a task to send messages
    let sender_task = tokio::spawn(async move {
        for i in 0..10 {
            let message = format!("Message {}", i);
            if let Err(e) = tx.send(message).await {
                eprintln!("Failed to send message: {}", e);
                return;
            }
        }
    });

    // Loop to receive messages
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}
```

This illustrates the basic idea of a channel:
1. Create a channel with a buffer size (in this case, 32 messages).
2. Split the channel into a transmitter (`tx`) and a receiver (`rx`).
3. Spawn a task to send messages using the transmitter.
4. Use a loop to receive messages using the receiver.

