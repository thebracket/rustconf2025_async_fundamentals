# Async Hello World

You've probably written something like this:

```rust
#[tokio::main]
async fn main() {
    hello().await;
}

async fn hello() {
    println!("Hello, async world!");
}
```

Let's take a moment to get this running on your machine.

> We're just making sure you're setup to run async code in Rust. So this is the sneaky "help you out up front if it explodes" practical!

Steps:

1. Open your terminal.
2. Create a new Rust project: `cargo new async_hello_world`
3. Change into the project directory: `cd async_hello_world`
4. Open `Cargo.toml` and add the dependencies below.
5. Open `src/main.rs` and replace its contents with the code above.
6. Run the project: `cargo run`

Dependencies to add in `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```
