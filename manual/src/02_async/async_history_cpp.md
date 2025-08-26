# And then there's C++

> Ever noticed how *every* history says "and then there's C++"?

All of the async system we've talked about so far are *opinionated*. The language and framework designers have made specific choices about how async should work, and you better like it!

C++ has to support *everything* (as does Rust). C++ has to support low-level programming (embedded systems, OS kernels), high-performance computing (games, simulations), and high-level programming (web servers, GUIs).

C++ also does not have a garbage collector. Notice that .NET, Python, Go, JavaScript, and even Erlang all have garbage collection. In a massively concurrent system, memory management is *hard*. C++ leaves that minor implementation detail to you.

So C++20 added `co_await`, `co_yield`, and `co_return` keywords to support coroutines. It doesn't make any assumptions about how you schedule or run them. It doesn't ship with an async runtime---just enough building blocks to let you implement your own.

And that's *great*.

Or it would be, but writing C++ coroutines is *hard*. Boost provides a few library solutions, but it's a mess. It's still a really powerful tool, but it needs more tooling!

> I actually tried to use Coroutines for asset loading in a game engine once. It worked perfectly, so long as you were running Microsoft Visual Studio. It exploded messily on everything else!