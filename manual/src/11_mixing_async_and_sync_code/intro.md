# Mixing Async and Sync Code

We talked about *function coloring* - or how `async` tends to be viral, because regular functions can't call `async` functions without being `async` themselves. We also talked about how `block_on` doesn't have to make your whole program sync - you can use it in a thread to run async code from sync code.

So how do we mix async and sync code in a server? The easy answer is with a channel.

```rust
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    std::thread::spawn(move || {
        // This is the sync part
        for i in 0..10 {
            tx.blocking_send(format!("Message {}", i)).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    });

    runtime.block_on(async move {
        // This is the async part
        while let Some(msg) = rx.recv().await {
            println!("Got: {}", msg);
        }
    });
}
```

Notice that we've kept the async part *very small*. It can be as small as you need - while the rest of your program focuses on sync code. This is a *really* common pattern when you have a synchronous program focused on number crunching, and just need a little bit of async code to handle I/O.

> You could also use `ureq` or a synchronous TCP server!