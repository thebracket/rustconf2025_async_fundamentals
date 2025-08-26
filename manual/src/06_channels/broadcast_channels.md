# Broadcast Channels

Sometimes, you want to send the same message to multiple tasks. How often have you written something like this?

```rust
pub static QUITTING: AtomicBool = AtomicBool::new(false);

fn main() {
    let task1 = tokio::spawn(async {
        while !QUITTING.load(Ordering::Relaxed) {
            // Do work
        }
    });

    let task2 = tokio::spawn(async {
        while !QUITTING.load(Ordering::Relaxed) {
            // Do work
        }
    });

    // Later, when you want to quit:
    QUITTING.store(true, Ordering::Relaxed);
}
```

This is a really common pattern, and it works pretty well. Suddenly, though, you have a global variable EVERYWHERE. Instead, you can use a broadcast channel.

```rust
use tokio::sync::broadcast;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Buffer size of 1 - we're not going to quit very often!
    let (tx, mut rx1) = broadcast::channel(1);

    // Make some tasks that listen for the quit signal
    for task_id in 0..2 {
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            loop {
                // Select really comes into its own for channels!
                tokio::select! {
                    _ = rx.recv() => {
                        println!("Task {} received quit signal, exiting.", task_id);
                        break;
                    }
                    // Simulate doing some work
                    _ = tokio::time::sleep(tokio::time::Duration::from_millis(50)) => {
                        println!("Task {} is working...", task_id);
                    }
                }
            }
        });
    }

    // Simulate doing some work in main
    tokio::time::sleep(tokio::time::Duration::from_secs_f32(0.5)).await;

    // Send the quit signal
    let _ = tx.send(());

    // Give tasks a moment to finish
    tokio::time::sleep(tokio::time::Duration::from_secs_f32(0.1)).await;
    println!("Main task exiting.");
}
```
