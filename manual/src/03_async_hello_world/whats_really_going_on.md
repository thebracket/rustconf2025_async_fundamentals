# What's Really Going On?

There's a surprising amount of "magic" in your Hello World example. Let's break it down, piece by piece.

## The `#[tokio::main]` Attribute

This is a really handy time-saver sometimes, but it's hiding a lot of complexity (although you can add that back in with parameters in many cases).

Let's build the same program without it:

```rust
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(tokio_main());
}

async fn tokio_main() {
    hello().await;
}

async fn hello() {
    println!("Hello, async world!");
}
```

> It surprised me when Tokio defaulted to making a thread pool, one thread per CPU core. Each thread has its own task queue, and tasks can be moved between threads as needed with work-stealing. By default, it's a reinvention of Go's reinvention of Erlang (Unlike Go, it's not inserting await points automatically!)!

## Flexibility

This is a bit more verbose, but you can specify exactly how you want your runtime to behave. Let's just use a single thread:

```rust
fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(tokio_main());
}

async fn tokio_main() {
    hello().await;
}

async fn hello() {
    println!("Hello, async world!");
}
```

> You can get the same effect with `#[tokio::main(flavor = "current_thread")]`!

This is *really handy* if you just want to run a few async tasks in part of your program. You can even run more than one runtime in a single program!

