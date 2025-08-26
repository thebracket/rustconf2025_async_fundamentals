# RustConf 2025 - Async Fundamentals

![Ardan Labs Logo](manual/src/ardanlabs-logo.png)

These are the slides and code samples for Herbert Wolverson's RustConf 2025 workshop on Async Foundations. This is a comprehensive 5.5-hour hands-on workshop designed for developers with some Rust proficiency who want to master async programming in Rust.

## About the Workshop

This workshop covers async Rust from the ground up, including:
- Understanding async/await and the runtime
- Building TCP servers and clients
- Working with channels and shared state
- Creating actor-based architectures
- Building web services with Axum
- gRPC services with Tonic
- Best practices for mixing sync and async code

## About Herbert Wolverson

Herbert Wolverson is the Rust Lead at [Ardan Labs](https://www.ardanlabs.com/) and author of *Hands-on Rust* and *Rust Brain Teasers* (Pragmatic Programmers), with *Advanced Hands-on Rust* coming soon.

## About Ardan Labs

[Ardan Labs](https://www.ardanlabs.com/) provides corporate training and consulting for Go, Rust, Kubernetes, and other cloud-native technologies. We help teams and individuals master modern software development through hands-on training and real-world experience.

## Running the Workshop Materials

### Slides

You can run the interactive slides locally with:

```bash
cargo install mdbook
cd manual
mdbook serve
```

Then open your browser to `http://localhost:3000`.

### Code Examples

All code examples are in the `code/` directory as a Cargo workspace. To ensure offline availability during the workshop:

```bash
cd code
cargo vendor
```

Each example can be run individually:

```bash
cd code/tcp_server_client
cargo run
```