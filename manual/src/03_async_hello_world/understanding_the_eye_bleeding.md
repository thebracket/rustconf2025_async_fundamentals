# Understanding the Eye Bleeding

That's a *lot* of code just to say "hello" asynchronously. In context, though - it's not too bad. You're effectively bolting a cooperative multitasking system onto a threaded operating system and language. It's a good opportunity to understand just how much work is being done for you---and why some async pain points exist.

Let's walk through some parts of the code. Starting with the function:

```rust
fn tokio_main() -> impl Future<Output = ()> {
    TokioMainFuture {
        state: TokioMainState::Start,
        hello_future: None,
    }
}
```

So the `tokio_main` function is returning a future. It's defined as having a state, and possibly a link to another future (the `hello` function). The state machine starts in the `Start` state, and the `hello_future` is initially `None`.

```rust
enum TokioMainState {
    Start,
    AwaitingHello,
    Done,
}
```

The available states are `Start`, `AwaitingHello`, and `Done`. This is a simple state machine that will help us track where we are in the execution of the async function.

```rust
struct TokioMainFuture {
    state: TokioMainState,
    hello_future: Option<HelloFuture>,
}
```

The `TokioMainFuture` struct holds the current state and the future for the `hello` function. The `hello_future` is an `Option`, which allows it to be `None` until we start awaiting it. In other words, we have to build a state machine to track the progress of our async function.

And now it gets a bit messy. Let's walk through *polling*:

```rust
// Implement Future for TokioMainFuture
impl Future for TokioMainFuture {
    // We're not returning anything, so Output is ()
    type Output = ();
    
    // The poll function is where the magic happens
    //
    // We have a MUTABLE reference to self - so we can change the state.
    // But - to avoid the borrow checker declaring everything as borrowed, we have to pin the future.
    // Pinning essentially locks the future in place: it can't go away, be moved, or be invalidated.
    //
    // We also receive a reference to a `Context` - which is the async runtime's way of telling us how to wake up 
    // when we're ready to continue.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // We loop until we reach a final state
        loop {
            // Match on the current state
            match self.state {
                // We're just starting, so we create the hello future and set our state.
                TokioMainState::Start => {
                    println!("tokio_main: Creating hello() future");
                    // Create the hello() future
                    self.hello_future = Some(hello());
                    self.state = TokioMainState::AwaitingHello;
                }
                // We haven't awaited yet, so the next run-through will poll the hello future
                TokioMainState::AwaitingHello => {
                    println!("tokio_main: Polling hello()");
                    // Poll the hello() future
                    if let Some(ref mut future) = self.hello_future {
                        // IMPORTANT: We need to pin the future before polling
                        let pinned = unsafe { Pin::new_unchecked(future) };
                        // Now we call the polled future.
                        match pinned.poll(cx) {
                            // It will either indicate that it's ready - we can indicate that we're ready, too.
                            Poll::Ready(()) => {
                                println!("tokio_main: hello() completed!");
                                self.state = TokioMainState::Done;
                                return Poll::Ready(());
                            }
                            // Or it will indicate that it's not ready yet - we need to return Pending
                            Poll::Pending => {
                                println!("tokio_main: hello() returned Pending");
                                return Poll::Pending;
                            }
                        }
                    }
                }
                TokioMainState::Done => {
                    return Poll::Ready(());
                }
            }
        }
    }
}
```

So whenever you await something, you're updating the state machine to reflect that you're waiting for another task to complete. Your executor will be polling your futures, and when the future is ready, it will wake up the task and continue executing from where it left off.