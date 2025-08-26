# Let's Add A Layer

Data isn't setup as a layer/extension (Tower Middleware is - you can also use that with Axum). Instead, attach your shared state to your service struct.

```rust
use shared_state_actor::SharedStateCommand;
use tokio::sync::mpsc::Sender;
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("hello");
}

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};


#[derive(Debug)] // I removed default
pub struct MyGreeter {
    my_actor: Sender<SharedStateCommand>, // Add the layer to the service struct
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        // Use the layer
        shared_state_actor::increment_counter(&self.my_actor).await;
        let new_count = shared_state_actor::get_counter(&self.my_actor).await;

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", new_count).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_actor = shared_state_actor::start().await;
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter { my_actor };

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```
