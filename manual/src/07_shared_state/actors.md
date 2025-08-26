# Actors

Mutating shared state often gets quite complicated. Even with accessor methods rather than public data, you can still wind up with a mess. Actors are one way to manage shared state, and avoid some of the common pitfalls. You find them in a *lot* of async Rust.

Usually, you'll find actors implemented with channels and a command pattern. For example:

```rust
use tokio::sync::mpsc;

// This part is usually in its own module/file.

pub enum ActorCommand {
    Increment,
    GetValue(tokio::sync::oneshot::Sender<u32>),
}

pub async fn start_actor() -> mpsc::Sender<ActorCommand> {
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        let mut state = 0;
        while let Some(cmd) = rx.recv().await {
            match cmd {
                ActorCommand::Increment => state += 1,
                ActorCommand::GetValue(sender) => {
                    let _ = sender.send(state);
                }
            }
        }
    });
    tx
}

pub async fn increment_actor(actor: &mpsc::Sender<ActorCommand>) {
    actor.send(ActorCommand::Increment).await.unwrap();
}

pub async fn get_actor_value(actor: &mpsc::Sender<ActorCommand>) -> u32 {
    let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
    actor.send(ActorCommand::GetValue(resp_tx)).await.unwrap();
    resp_rx.await.unwrap()
}

// Use the actor

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // You clone this to all the parts that need it.
    let actor = start_actor().await;

    // Calling the actor is a normal async function call.
    increment_actor(&actor).await;
    increment_actor(&actor).await;
    let value = get_actor_value(&actor).await;
    println!("Actor value: {}", value);
}
```

You gain a few benefits from this pattern:

* There's no mutex! The actor is the only code that mutates the state, so you don't have to worry about locking or deadlocks.
* The actor can be sent to multiple tasks, and they can all share the same state.
* You can change to an MPMC channel and have multiple actors if you need to scale up (for actually shared state, now you DO need to worry about synchronization between actors).
* You have backpressure: if the actor can't keep up, the channel will fill up, and the callers will be backpressured.

There's a few pitfalls, too:
* The actor is a single point of failure. If it crashes, the whole system might be affected.
* The actor can become a bottleneck if it can't keep up with the load.
* There's a bit more code to write. Not a lot, but it's boilerplate - and it takes a bit of discipline to not just start mutating state directly.

There are whole frameworks such as `ractor` built around the actor model. As we said in history: eventually, we end up reinventing Erlang!