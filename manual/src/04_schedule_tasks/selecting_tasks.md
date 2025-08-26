# Selecting Tasks

Tokio includes a really handy `select!` macro (there are others, including in `futures`), which allows you to wait for multiple tasks to complete, and select the first one that does.

```rust
#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        "Task 1 completed"
    });

    let task2 = tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        "Task 2 completed"
    });

    tokio::select! {
        result = task1 => println!("{}", result.unwrap()),
        result = task2 => println!("{}", result.unwrap()),
    }
}
```

Task 2 will complete first (it waits the shortest time). But you can use this to combine multiple inputs in a single asynchronous function. More on that in a moment.