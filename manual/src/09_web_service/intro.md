# A Web Service

There are quite a few web frameworks in Rust. The one we will be using is [Axum](https://docs.rs/axum/latest/axum/). Axum is built on top of [Hyper](https://hyper.rs/) and [Tokio](https://tokio.rs/). It is a very ergonomic framework that makes it easy to build web services. It's also quite popular---and is updated regularly.

Follow along, and let's build a simple web service that responds with "Hello, World!" to any request.

## Initial Setup

1. Create a new project `cargo new axum_hello`.
2. Change into the new directory `cd axum_hello`.
3. Add dependencies: `cargo add tokio -F full` and `cargo add axum`.

## Hello Axum

Axum feels a lot like a basic NodeJs Express app. You define routes and handlers, and a lot of magic takes place behind the scenes. Here's "Hello Axum":

```rust
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Not bad for 9 lines of code!