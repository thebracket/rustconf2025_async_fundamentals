# Let's Add Some Timing

You're already writing async code, one slide into the "let's stop scaring everyone" section!

```rust
async fn looper(n: u8) {
    for i in 0..3 {
        println!("[{n}]: Iteration {i}");
        std::thread::sleep(std::time::Duration::from_millis(i));
    }
}

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..3).map(|n| looper(n)).collect();
    futures::future::join_all(tasks).await; // This waits for all tasks to complete
}
```

The output is:

```
[0]: Iteration 0
[0]: Iteration 1
[0]: Iteration 2
[1]: Iteration 0
[1]: Iteration 1
[1]: Iteration 2
[2]: Iteration 0
[2]: Iteration 1
[2]: Iteration 2
```

Probably not what you had in mind! Anyone want to guess why?

---

(GAP GRAPHIC)

Calling `std::thread::sleep` is a really bad idea in async code. It blocks the *entire thread* for the duration of the sleep, preventing any other tasks from running. That includes the executor itself, which is unable to schedule other tasks while the current one is sleeping. This is an example of *blocking the executor* - and is just like running a badly behaved game on Windows 3!

Tokio - in their continuing effort to build the entire standard library in async code - has a `sleep` function that does not block the executor:

```rust
async fn looper(n: u8) {
    for i in 0..3 {
        println!("[{n}]: Iteration {i}");
        tokio::time::sleep(std::time::Duration::from_millis(i)).await;
    }
}

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..3).map(|n| looper(n)).collect();
    futures::future::join_all(tasks).await; // This waits for all tasks to complete
}
```

The output is now:

```
[0]: Iteration 0
[1]: Iteration 0
[2]: Iteration 0
[0]: Iteration 1
[1]: Iteration 1
[2]: Iteration 1
[0]: Iteration 2
[1]: Iteration 2
[2]: Iteration 2
```

The sleep function isn't blocking, and all of the tasks are able to run concurrently. Whether you use a threaded or single-threaded runtime, the tasks will run concurrently, interleaving their output:

```rust
async fn looper(n: u8) {
    for i in 0..3 {
        println!("[{n}]: Iteration {i}");
        tokio::time::sleep(std::time::Duration::from_millis(i)).await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let tasks: Vec<_> = (0..3).map(|n| looper(n)).collect();
    futures::future::join_all(tasks).await; // This waits for all tasks to complete
}
```

That's pretty cool. You've built a surprising amount of a Xerox Star (and if this were a Xerox Star, you ran out of RAM)!