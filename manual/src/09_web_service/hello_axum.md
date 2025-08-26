# Hello Axum - Add Some JSON

Axum includes JSON processing via the `axum::Json` extractor and response type. It's remarkably automatic!

## Setup

1. In your `axum_hello` directory, we need to add one more dependency: `cargo add serde --features derive`.

Now we can modify our `main.rs` to handle JSON:

```rust
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/json", axum::routing::get(hello_json));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize)]
struct HelloJson {
    message: String,
}

async fn hello_json() -> axum::Json<HelloJson> {
    let response = HelloJson {
        message: "Hello, World!".to_string(),
    };
    axum::Json(response)
}
```

All we've done is add a new route `/json` that responds with a JSON object. We also setup the data - a regular struct, deriving Serde's `Serialize` trait. The handler function `hello_json` constructs an instance of that struct and returns it wrapped in `axum::Json`.

Run it, and you can go to `http://localhost:3001/json` to see the JSON response:

```json
{
  "message": "Hello, World!"
}
```

The great news is that if Serde can serialize it (it supports just about everything!), Axum can return it as JSON with minimal effort.