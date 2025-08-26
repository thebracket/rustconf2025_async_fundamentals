# What's Really Going On With The Async Keyword?

There's another part of the program that is quietly doing magic without telling you:

```rust
async fn hello()
```

The `async` keyword is doing a lot of work behind the scenes. You can get a clue by looking at the type of the function:

```rust
#[tokio::main]
async fn main() {
    let h = hello();
}

async fn hello() {
    println!("Hello, async world!");
}
```

If you hover over `h` in an IDE, you'll see that it's of type `impl Future<Output = ()>`. If you use a function to print the type, you'll see that its a closure!

The async keyword is doing a *lot* of magic, on every single async function! It's transforming the function into a state machine that implements the `Future` trait (and occasionally gets better as the compiler improves).

In the name of understanding, here's the world's most overcomplicated `async hello world`:

```rust
fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(tokio_main());
}

async fn tokio_main() {
    println!("Hello, async world!");
}
```

And here's what that `async fn tokio_main()` actually becomes (simplified):

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

// This is what async fn tokio_main() becomes:
fn tokio_main() -> impl Future<Output = ()> {
    // Return a struct that implements Future
    TokioMainFuture { done: false }
}

struct TokioMainFuture {
    done: bool,
}

impl Future for TokioMainFuture {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.done {
            println!("Hello, async world!");
            self.done = true;
            Poll::Ready(())
        } else {
            Poll::Ready(())
        }
    }
}
```

The `async` keyword creates a state machine (the `TokioMainFuture` struct) that tracks where we are in the function's execution. For this simple example with no await points, it just runs once and completes!

> I've never had to do this in real code. The `async` keyword really makes life easier! But it's also hiding a lot of complexity. Note that this isn't *exactly* what the compiler generates---the compiler is super optimized. This is the "readable" version!