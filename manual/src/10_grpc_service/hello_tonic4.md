# Hello Tonic - The Client

```rust
// Once again - automatically generated code!
pub mod hello_world {
    tonic::include_proto!("hello");
}

// The client is written for us!
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

// So we mostly just have to call it!
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
```

So let's test it. First, run the server:

```bash
cargo run --bin helloworld-server
```

In a second terminal, run the client:

```bash
cargo run --bin helloworld-client
```

On the client, you should see:

```
RESPONSE=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Tue, 26 Aug 2025 20:36:41 GMT", "grpc-status": "0"} }, message: HelloReply { message: "Hello Tonic!" }, extensions: Extensions }
```