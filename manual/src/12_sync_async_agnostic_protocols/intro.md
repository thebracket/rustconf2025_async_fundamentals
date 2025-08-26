# Sync/Async Agnostic Protocols

You've probably heard "no proto" and similar phrases. What does it mean? It means that the protocol you're using doesn't care whether the code is async or sync. This is important because it allows you to mix and match async and sync code without worrying about the underlying implementation details.

You'd implement this by building your protocol (usually as a state machine) with simple input/output methods that accept data (often byte slices) and return data (often byte slices). Then the caller has to write the I/O code.

The advantage: you can easily support whatever I/O model you want - sync, async, or even something else (like an embedded system with no OS).

The disadvantage: you have to write the I/O code yourself. But this is often a small price to pay for the flexibility you gain.

## The Other Option

You can also define the protocol, and provide both sync and async implementations. This is a LOT more work, but it's much easier on your poor users!