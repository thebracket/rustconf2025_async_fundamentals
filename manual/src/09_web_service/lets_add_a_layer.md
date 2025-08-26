# Let's Add A Layer

> The code is in `code/axum_layered`.

It's relatively unusual to have a web service with no state (including things like database connectors!). Axum includes a concept of "layers" that can be used to add state to your application.

Layers are *anything* that can be readily cloned. This includes things like database connection pools, caches, etc. In this case, we'll just wrap a string in an `Arc`.

So first, we create the data:

```rust
// Layers have to be readily cloneable, so we use an `Arc`.
let message = Arc::new(String::from("Hello Shared State!"));
```

Then we add the *layer* to our router:

```rust
let app = Router::new()
    .route("/", axum::routing::get(|| async { "Hello, World!" }))
    .route("/json", axum::routing::get(hello_json))
    .route("/json_post", axum::routing::post(receive_json))
    .layer(Extension(message)); // Add the layer here
```

And now we can adjust our handler to accept the layer:

```rust
async fn hello_json(
    Extension(message): Extension<Arc<HelloJson>>, // Extract the layer here
) -> axum::Json<HelloJson> {
    let reply = HelloJson {
        message: message.message.clone(),
    };
    axum::Json(reply)
}
```