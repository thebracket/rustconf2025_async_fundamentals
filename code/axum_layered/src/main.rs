use std::sync::Arc; // I told you Arc was everywhere!
use axum::{http::StatusCode, Extension, Json, Router};

#[tokio::main]
async fn main() {
    // Layers have to be readily cloneable, so we use an `Arc`.
    let message = Arc::new(String::from("Hello Shared State!"));

    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/json", axum::routing::get(hello_json))
        .route("/json_post", axum::routing::post(receive_json))
        .layer(Extension(message)); // Add the layer here

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct HelloJson {
    message: String,
}

async fn hello_json(
    Extension(message): Extension<Arc<HelloJson>>, // Extract the layer here
) -> axum::Json<HelloJson> {
    let reply = HelloJson {
        message: message.message.clone(),
    };
    axum::Json(reply)
}

async fn receive_json(
    Json(payload): Json<HelloJson>,
) -> StatusCode {
    println!("Received payload: {:?}", payload);
    StatusCode::OK
}