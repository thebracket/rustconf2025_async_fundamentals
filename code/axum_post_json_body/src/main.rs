use axum::{http::StatusCode, Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/json", axum::routing::get(hello_json))
        .route("/json_post", axum::routing::post(receive_json));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct HelloJson {
    message: String,
}

async fn hello_json() -> axum::Json<HelloJson> {
    let response = HelloJson {
        message: "Hello, World!".to_string(),
    };
    axum::Json(response)
}

async fn receive_json(
    Json(payload): Json<HelloJson>,
) -> StatusCode {
    println!("Received payload: {:?}", payload);
    StatusCode::OK
}