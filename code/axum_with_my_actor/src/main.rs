use axum::{http::StatusCode, Extension, Json, Router};
use shared_state_actor::SharedStateCommand;
use tokio::sync::mpsc::Sender;

#[tokio::main]
async fn main() {
    // Start my actor here and get its handle
    let my_actor = shared_state_actor::start().await;

    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/json", axum::routing::get(hello_json))
        .route("/json_post", axum::routing::post(receive_json))
        .layer(Extension(my_actor)); // Add the actor here

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
    Extension(my_actor): Extension<Sender<SharedStateCommand>>, // Extract the layer here
) -> axum::Json<HelloJson> {
    shared_state_actor::increment_counter(&my_actor).await;
    let new_total = shared_state_actor::get_counter(&my_actor).await;

    let reply = HelloJson {
        message: format!("Counter: {}", new_total),
    };
    axum::Json(reply)
}

async fn receive_json(
    Json(payload): Json<HelloJson>,
) -> StatusCode {
    println!("Received payload: {:?}", payload);
    StatusCode::OK
}