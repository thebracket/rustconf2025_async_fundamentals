# Program Skeleton

> The code is available in the `code/tcp_server_client` directory of the repository.

Let's start by creating a program skeleton.

1. Create your new Rust project.
2. Add `tokio` with `cargo add tokio -F full`.

Let's create a basic skeleton structure:

```rust
#[tokio::main]
async fn main() {
    tokio::spawn(server());
    // Give the server time to start. You don't actually need this!
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.25)).await;
    client().await;
}

async fn server() {
    // Server code will go here
}

async fn client() {
    // Client code will go here
}
```

Obviously, this doesn't do anything yet!