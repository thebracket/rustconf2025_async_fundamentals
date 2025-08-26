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
