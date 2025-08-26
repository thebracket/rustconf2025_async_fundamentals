# What's The Runtime Doing?

Let's go back to a simple example, and not use Tokio at all!

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

// --- async tasks ---
async fn hello() {
    println!("    [hello] Hello");
}

async fn goodbye() {
    futures::pending!(); // Simulate a pending operation
    println!("    [goodbye] Goodbye");
}

// --- root async function ---
async fn async_main() {
    println!("  [async_main] Starting");
    // We can await tasks sequentially
    println!("  [async_main] About to await hello()");
    hello().await;
    println!("  [async_main] hello() completed, about to await goodbye()");
    goodbye().await;
    println!("  [async_main] goodbye() completed");

    // We can also spawn tasks concurrently
    println!("  [async_main] Spawning hello() and goodbye() concurrently");
    let hello_future = hello();
    let goodbye_future = goodbye();
    // We can await both futures concurrently
    futures::join!(hello_future, goodbye_future);
    println!("  [async_main] hello() and goodbye() completed");
}

// --- minimal dummy waker ---
fn dummy_waker() -> Waker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
    fn dummy_raw_waker() -> RawWaker {
        RawWaker::new(std::ptr::null(), &RawWakerVTable::new(clone, no_op, no_op, no_op))
    }
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

// --- block_on executor ---
fn block_on<F: Future>(mut fut: F) -> F::Output {
    println!("[Runtime] Creating waker and context");
    let waker = dummy_waker();
    let mut ctx = Context::from_waker(&waker);
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

    let mut poll_count = 0;
    loop {
        poll_count += 1;
        println!("[Runtime] Poll #{}: Polling future", poll_count);
        match fut.as_mut().poll(&mut ctx) {
            Poll::Ready(val) => {
                println!("[Runtime] Future completed after {} poll(s)!", poll_count);
                return val;
            }
            Poll::Pending => {
                println!("[Runtime] Future returned Pending");
            }, // spin: trivial, no actual async
        }
    }
}

fn main() {
    println!("[Main] Starting block_on(async_main())");
    block_on(async_main());
    println!("[Main] Completed!");
}
```

The key here is that Rust requires a `Waker` to be able to wake up a task when it's ready to continue, and `block_on` just spins in a loop polling the future until it completes. This is a very simplified executor, but it illustrates the core concepts.

When you run this code, you'll see output that shows how the state machine progresses through the various states as it awaits different futures. Each `await` point effectively turns into a state transition in the state machine.

The output:

```
[Main] Starting block_on(async_main())
[Runtime] Creating waker and context
[Runtime] Poll #1: Polling future
  [async_main] Starting
  [async_main] About to await hello()
    [hello] Hello
  [async_main] hello() completed, about to await goodbye()
[Runtime] Future returned Pending
[Runtime] Poll #2: Polling future
    [goodbye] Goodbye
  [async_main] goodbye() completed
  [async_main] Spawning hello() and goodbye() concurrently
    [hello] Hello
[Runtime] Future returned Pending
[Runtime] Poll #3: Polling future
    [goodbye] Goodbye
  [async_main] hello() and goodbye() completed
[Runtime] Future completed after 3 poll(s)!
[Main] Completed!
```

> The upshot here is: "We aren't in Kansas anymore, Toto." This is *very* unlike normal Rust! A huge amount is being done for you under the hood, and it's quite low level. It isn't magic, but it looks like it! Once you get over the initial shock, it's an elegant solution to a very hard problem.