# Hello Tonic

gRPC is a bit more complicated because it uses Protocol Buffers (Protobuf) to define the service and message formats.

So let's start by making our project:

```bash
cargo new hello_tonic
cd hello_tonic
```

In our `Cargo.toml` we need some dependencies:

```toml
[dependencies]
tonic = "0.10"
prost = "0.12"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

[build-dependencies]
tonic-build = "0.10"
```

> I've pinned exact version numbers here to avoid surprises, and because it changes quite often!

We also want to build the server and the client in the same project, so let's use Cargo's feature that lets us have two binaries in the same project. Also in `Cargo.toml`:

```toml
[[bin]] # Bin to run the HelloWorld gRPC server
name = "helloworld-server"
path = "src/server.rs"

[[bin]] # Bin to run the HelloWorld gRPC client
name = "helloworld-client"
path = "src/client.rs"
```

---