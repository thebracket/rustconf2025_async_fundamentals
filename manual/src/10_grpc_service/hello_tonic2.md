# Hello Tonic 2

Now we need to write the Protobuf (I keep typing ProtoBug!) definition for our service. We want to add a directory next to `src` called `proto`. In there, we add `hello.proto`:

```proto
syntax = "proto3";
package hello;

service Greeter {
    rpc SayHello (HelloRequest) returns (HelloReply);
}

message HelloRequest {
   string name = 1;
}

message HelloReply {
    string message = 1;
}
```

Now we'll use a bit of Cargo magic to define a `build.rs` that writes a ridiculous amount of code for us. In the root of the project, create `build.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/hello.proto")?;
    Ok(())
}
```

> You can comment out the binaries and build now if you like. You'll find a huge amount of generated code in `target/debug/build/hello_tonic-*/out/`.