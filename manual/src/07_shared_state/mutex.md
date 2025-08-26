# Mutex

A great way to shoot yourself in the foot in async Rust is to use a regular Mutex. A regular Mutex will block the entire thread when it is locked. This means that if you lock a Mutex in an async task, you will block the entire thread, and all other tasks on that thread, until the Mutex is unlocked.

This is a common mistake, and it can lead to deadlocks and other problems. Instead, you should use an async-aware Mutex, such as `tokio::sync::Mutex` or `async_lock::Mutex`. These Mutexes will only block the task that is trying to lock the Mutex, and will allow other tasks to run on the same thread.

> You should still avoid ever passing a Mutex guard across an `.await` point. This can lead to deadlocks and other problems. Instead, you should lock the Mutex, do your work, and then unlock the Mutex before the `.await` point. Just don't. It hurts.

Let's look at the `code/mutex` example. This is a simple program that spawns a bunch of tasks that increment a counter. The counter is protected by a `tokio::sync::Mutex`.

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            for _ in 0..1000 {
                let mut num = counter.lock().await;
                *num += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    let num = counter.lock().await;
    println!("Final counter value: {}", *num);
}
```

It's *just* like a threaded version, but the Mutex is async-aware. This *might* work with a regular Mutex - or it might not. You might get lucky, or you might deadlock. Don't take the chance.