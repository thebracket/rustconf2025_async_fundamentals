# Axum with POST-JSON body

> The code is in `code/axum_post_json_body` and `code/axum_poster`.

With a couple of minor tweaks, we can extend our Axum service to accept a POST request with a JSON body.

First, the handler function:

```rust
async fn receive_json(
    Json(payload): Json<HelloJson>,
) -> StatusCode {
    println!("Received payload: {:?}", payload);
    StatusCode::OK
}
```

And then the route:

```rust
.route("/json_post", axum::routing::post(receive_json));
```

Axum is really nice in terms of taking care of the conversion for you!

Let's write a quick client to send a POST request with a JSON body. Create a new binary crate with `cargo new axum_poster`. Add the dependencies:

```bash
cargo add reqwest -F json
cargo add tokio -F full
cargo add serde -F derive
```

And the client:

```rust
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct HelloJson {
    message: String,
}

#[tokio::main]
async fn main() {
    let message = HelloJson {
        message: "Hello, World!".to_string(),
    };
    reqwest::Client::new()
        .post("http://localhost:3001/json_post")
        .json(&message)
        .send()
        .await
        .unwrap();
}
```

> Other HTTP verbs work similarly. For HTTP+JSON services, Axum is as easy as Go!