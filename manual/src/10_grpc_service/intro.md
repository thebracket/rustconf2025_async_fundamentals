# A gRPC Service

JSON-HTTP is popular, and so is gRPC. gRPC is a Remote Procedure Call (RPC) framework that uses HTTP/2 as its transport protocol and Protocol Buffers (protobuf) as its interface definition language. It was originally developed by Google and has become widely adopted for building efficient and scalable APIs.

> Author grumpiness: it's not really that efficient when you compare it to a custom protocol! All the headers and metadata add up. I've seen small payloads have headers massively outweigh the payload!

So let's make a gRPC service!