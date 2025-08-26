# A Side of Silliness

You won't do this very often (or you might! It's actually a common way to reduce latency!), but you can run lots of Tokio's at once!

```rust
fn main() {
    let t1 = std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            hello_from_thread(1).await;
        });
    });
    let t2 = std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            hello_from_thread(2).await;
        });
    });
    t1.join().unwrap();
    t2.join().unwrap();
}

async fn hello_from_thread(id: u8) {
    println!("Hello from thread {}!", id);
}
```

In this case, it's not exactly *useful* - but each could be running webservers, network servers, network clients, etc. independently of one another. Once we get to channels, it starts to be useful!