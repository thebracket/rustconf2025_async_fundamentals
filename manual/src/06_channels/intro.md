# Channels

Go likes to take credit for Channels, even with their famous quote:

> “Do not communicate by sharing memory; instead, share memory by communicating.” - Rob Pike

Channels are actually *much* older than Go. Tony Hoare introduced the concept of Communicating Sequential Processes (CSP) in the 1970s, which laid the groundwork for channel-based communication in concurrent programming. Van Jacobson further developed these ideas in the 1980s, leading to the implementation of channels in languages like Occam and later in languages such as Erlang and Ada.

Rust's channels in "threaded/synchronous land" are really handy - but sometimes feel somewhat limited. Spawning a thread for every task that needs to receive messages is quite heavy-weight. Channels really come into their own when used with async code.

Channels provide a way to send messages from one task to another. Using other channels, you can even reply.