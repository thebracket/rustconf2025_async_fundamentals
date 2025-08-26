# Channels with Split RX/TX in Servers

Channels make it easy to turn a TCP server into a multi-tasking server, and separate concerns.

Let's take a look at the example code `code/tcp_server3_sync`.

1. The server listens for incoming connections.
2. The client connects and sends a "calculate" command.
3. The server performs the calculation and sends the result back.
4. The client receives the result and prints it.
5. The client sends the "hello" command.
6. The server responds with a greeting.
7. The client receives the greeting and prints it.

This is great for many patterns: the server is performing the requested steps, exactly in order.

---

But what if you have more of a "scatter gather" type requirement? You request lots of different pieces---say data for your dashboard. You don't want each request to block the next one. Or you are building a chat server, and you don't want clients to only process messages after the previous one sends a message - you want to handle messages as they arrive.

Let's look at `code/tcp_server4_async`. This is a more complex example:

* We're using `split` to break the TCP stream into a read half and a write half (in synchronous Rust, you can just clone the stream).
* We build a channel to send messages from the main server task to the per-client task.
* Calling `calculate` is now done in a separate task, so it doesn't block the main server loop.

So now you have a server that can handle multiple clients, and each client can send multiple requests without blocking each other.

The client does a similar trick.

> And best of all - we're only using one thread and a miniscule amount of memory per client. This is the power of async programming.