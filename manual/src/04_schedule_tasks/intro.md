# Let's Schedule Some Tasks

You've already done the most basic form of scheduling:

```rust
#[tokio::main]
async fn main() {
    hello().await; // This schedules the `hello` task and waits for it to complete.
}

async fn hello() {
    println!("Hello, world!");
}
```

## Following Along

Please add `futures` to your dependencies with `cargo add futures`.

---

We even used `join` to call two at once. Let's use the Tokio version this time:

```rust
#[tokio::main]
async fn main() {
    tokio::join!(hello(), world()); // This schedules both `hello` and `world` tasks to run concurrently.
}

async fn world() {
    println!("world!");
}
async fn hello() {
    println!("Hello!");
}
```

Tokio even has a `spawn` function that allows you to schedule tasks without waiting for them to complete, and *encourages* them to start on other threads:

```rust
#[tokio::main]
async fn main() {
    for n in 0..10 {
        tokio::spawn(hello(n)); // This schedules the `hello` task to run concurrently
    }
}

async fn hello(n: u8) {
    println!("Hello {n}");
}
```

> It's not guaranteed that the tasks will run in order, or even that they will all run before the program exits. If you want to ensure that all tasks complete, you can use `join_all`:

```rust
use futures::future::join_all;

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..10).map(|n| tokio::spawn(hello(n))).collect();
    join_all(tasks).await; // This waits for all tasks to complete.
}

async fn hello(n: u8) {
    println!("Hello {n}");
}
```

---

And now the cool part. You can do the same thing with `spawn` - entirely single threaded!

```rust
use futures::future::join_all;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let tasks: Vec<_> = (0..10).map(|n| tokio::spawn(hello(n))).collect();
    join_all(tasks).await; // This waits for all tasks to complete.
}

async fn hello(n: u8) {
    println!("Hello {n}");
}
```

> This will almost always run the tasks in order, but that's not a guarantee or promise!