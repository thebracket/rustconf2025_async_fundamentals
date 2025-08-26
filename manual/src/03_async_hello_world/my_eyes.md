# My Eyes! My Eyes!

Here's a slightly more complicated example:

```rust
fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(tokio_main());
}

async fn tokio_main() {
    hello().await;
}

async fn hello() {
    println!("Hello, async world!");
}
```

Let's desugar that:

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(tokio_main());
}

// tokio_main() becomes a function returning a Future
fn tokio_main() -> impl Future<Output = ()> {
    TokioMainFuture {
        state: TokioMainState::Start,
        hello_future: None,
    }
}

enum TokioMainState {
    Start,
    AwaitingHello,
    Done,
}

struct TokioMainFuture {
    state: TokioMainState,
    hello_future: Option<HelloFuture>,
}

impl Future for TokioMainFuture {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.state {
                TokioMainState::Start => {
                    println!("tokio_main: Creating hello() future");
                    // Create the hello() future
                    self.hello_future = Some(hello());
                    self.state = TokioMainState::AwaitingHello;
                }
                TokioMainState::AwaitingHello => {
                    println!("tokio_main: Polling hello()");
                    // Poll the hello() future
                    if let Some(ref mut future) = self.hello_future {
                        // IMPORTANT: We need to pin the future before polling
                        let pinned = unsafe { Pin::new_unchecked(future) };
                        match pinned.poll(cx) {
                            Poll::Ready(()) => {
                                println!("tokio_main: hello() completed!");
                                self.state = TokioMainState::Done;
                                return Poll::Ready(());
                            }
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

// hello() also becomes a Future - but let's make it more interesting
fn hello() -> HelloFuture {
    HelloFuture { 
        state: HelloState::Start,
        print_async_future: None,
    }
}

enum HelloState {
    Start,
    AwaitingPrint,
    Done,
}

struct HelloFuture {
    state: HelloState,
    print_async_future: Option<PrintAsyncFuture>,
}

impl Future for HelloFuture {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.state {
                HelloState::Start => {
                    println!("  hello: Creating print_async() future");
                    self.print_async_future = Some(print_async());
                    self.state = HelloState::AwaitingPrint;
                }
                HelloState::AwaitingPrint => {
                    println!("  hello: Polling print_async()");
                    if let Some(ref mut future) = self.print_async_future {
                        let pinned = unsafe { Pin::new_unchecked(future) };
                        match pinned.poll(cx) {
                            Poll::Ready(()) => {
                                println!("  hello: print_async() completed!");
                                self.state = HelloState::Done;
                                return Poll::Ready(());
                            }
                            Poll::Pending => {
                                println!("  hello: print_async() returned Pending");
                                return Poll::Pending;
                            }
                        }
                    }
                }
                HelloState::Done => {
                    return Poll::Ready(());
                }
            }
        }
    }
}

// Another async function that hello() awaits
fn print_async() -> PrintAsyncFuture {
    PrintAsyncFuture { done: false }
}

struct PrintAsyncFuture {
    done: bool,
}

impl Future for PrintAsyncFuture {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.done {
            println!("    print_async: Hello, async world!");
            self.done = true;
            Poll::Ready(())
        } else {
            Poll::Ready(())
        }
    }
}
```

See how much more complex it gets with just one `await`? The state machine now has to:
1. Track which state it's in (Start, AwaitingHello, Done)
2. Store the future it's waiting on
3. Poll that future and handle both Ready and Pending cases
4. Properly propagate the waker context

And this is still simplified! The real compiler-generated code is even more optimized and complex.

> Writing async by hand is a road to madness. It's also a great way to understand just what's actually happening. There's a *lot* of code being generated for each await point. Now think about some webservers that often have twenty or more await points just to handle a single request!

The amazing part is that despite all of this: it's really, really fast. State machines are efficient, and a truly remarkable number of computer science problems boil down to a state machine or a finite automaton!