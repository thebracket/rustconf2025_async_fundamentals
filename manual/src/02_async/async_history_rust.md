# Rust

Rust didn't ship with async/await support until version 1.39 in 2019. Rust's async story is still evolving, but it's already pretty good.

Rust has the same constraints as C++: it has to support everything from your toaster to a supercomputer. It doesn't have a garbage collector---but at least memory management is easier, and you have to try quite hard to blow yourself up.

Rust also doesn't ship with batteries included---the standard library includes enough to build your own runtime, but doesn't ship with one.

|Runtime|Comments|
|-------|--------|
|Futures|A minimal async runtime, missing a lot of features including a "reactor" (IO loop). Great for learning or slipping a little async into a sync program.|
|Embassy|An async runtime for embedded systems. Cooperatively multitask on your watch|
|Smol|A minimal async runtime, great for learning or small programs|
|Tokio|The most popular async runtime. Full-featured, with a multi-threaded scheduler, timers, channels. Also has single-threaded and current-thread runtimes.|
|async-std|An async runtime that mimics the standard library. Full-featured, with a multi-threaded scheduler, timers, channels. Not as widely used.|

And there are many more, some focused on specific use cases (such as `io_uring` on Linux). Even the Bevy game engine has its own async runtime!