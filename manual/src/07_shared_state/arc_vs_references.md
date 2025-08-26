# Arc vs References

One of the common criticisms of async Rust is that sharing regular references between tasks is either impossible or difficult---getting into nasty "pinning" and other hard-to-reason-about areas of Rust.

This turns into a second criticism: you tend to have `Arc` everywhere!

Take a step back to the history of async programming. Notice that all of the modern async systems (other than Rust and C++) have garbage collection? Garbage collection makes sharing state relatively easy: the shared data sits in a managed heap, and is cleaned up when you're done with it.

Rust doesn't have a big, heavy garbage collector: but it does have `Arc`. `Arc` is a pointer to some data on the heap, that also has a reference count. When you `clone` it, you get *the same pointer*, but the reference count is incremented. When you drop an `Arc`, the reference count is decremented. When the reference count hits zero, the data is cleaned up.

You can see this in the following small example:

```rust
use std::sync::Arc;

struct MyData(u32);
impl Drop for MyData {
    fn drop(&mut self) {
        println!("Dropping MyData({})", self.0);
    }
}

fn main() {
    let data = Arc::new(MyData(42));
    let data_clone = Arc::clone(&data);
    drop(data);
    drop(data_clone);
}
```

The arc is only dropped once - it's cloned repeatedly.

This makes `Arc` and "arc-like" types (things *designed* to be cloned) very popular in async Rust---and hard to avoid. Shared resources and state winds up wrapped in an Arc, and passed around. Cloning it is cheap, and you don't have to worry about lifetimes or pinning. You can add *interior mutability* to allow mutation of the shared state and retain Rust's safety guarantees.

So "Arc everywhere" *is* a common pattern. It's a valid criticism that you'll get pretty sick of typing `Arc<...>` everywhere, but it's a tradeoff. You get safety, and you get to avoid lifetimes and pinning. You even have a garbage collector of sorts (reference counting), without the non-deterministic pauses and loss of `Drop` semantics.

> Note that `Drop` *can* get really messy with task cancellation. That gets into really advanced Rust pain!