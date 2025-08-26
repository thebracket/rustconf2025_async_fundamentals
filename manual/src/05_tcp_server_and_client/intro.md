# A TCP Server and Client

We've covered the basics of async Rust - scheduling tasks, waiting for them to complete, and running them concurrently. There's obviously a lot more available, but nobody wants to sit and listen to me talk about the details of the `futures` crate for an hour. Instead, let's build something!

Our first project will be a simple TCP server and client that talk to one another. We'll keep them in the same file, and keep it simple.
