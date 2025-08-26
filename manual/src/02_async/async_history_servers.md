# Async Servers

> Those who don't remember Erlang are condemned to reimplement it, over and over again.

## Erlang

In 1986, Ericsson started developing Erlang, a programming language designed for building highly concurrent, distributed, and fault-tolerant systems. It was used to build telecom switches that could handle thousands of simultaneous calls. (It was open-sourced in 1998.) Erlang introduced:

* **Actors**: Lightweight processes that communicate via message passing.
* **Fault Tolerance**: "Let it crash" philosophy, where processes can fail without bringing down the entire system.
* **Hot Code Swapping**: Ability to update code without stopping the system.
* **Await/Async**: Built-in support for asynchronous programming.

Erlang is a lot like Kubernetes, on a smaller scale. Individual modules (actors) can be updated, restarted, or replaced without affecting the whole system. It's designed to run on distributed systems, making it ideal for telecom applications.

Most importantly, Erlang is built around **green threads** (lightweight threads managed by the Erlang runtime, not the OS).

## Early Windows

In the late 1990s, Microsoft Internet Information Services (IIS) quietly added a configuration flag labelled "Async I/O". It wasn't really documented, and nobody seemed to know what to do with it - but suddenly IIS systems could go faster. Or crash. Some documentation might have helped.

Later releases (IIS 5 and later) actually documented the feature, introducing "IO Completion" (events fired when I/O completed) and "Overlapped I/O" (issuing multiple I/O requests without waiting for each to complete).

Windows also introduced "fibers" - lightweight threads that could be scheduled by the application. It's almost green threads...

## Node.js

In 2009, Ryan Dahl created Node.js, a JavaScript runtime built on Chrome's V8 engine. Node.js was based on `libev`, which provides an event loop and asynchronous I/O capabilities. Suddenly you could serve thousands of connections with a single-threaded server. And JavaScript. You may like JS more than me!

## Go

In 2009, Google released Go. Go was the poster-child for async programming at the time. Go even advertised itself as having green threads (goroutines) and channels for communication.

Go creates a task manager per CPU core (by default), each of which can run "goroutines" (async tasks). The Go runtime schedules goroutines onto available OS threads, and cheats a little: *every* function call quietly "yields" to the scheduler, allowing it to switch between async tasks. Go also manages blocking calls (like I/O) by using a pool of OS threads to handle them, so the main goroutine scheduler isn't blocked.

## Async .NET

In 2012, Microsoft introduced `async` and `await` keywords in C# 5.0 and .NET Framework 4.5. This made asynchronous programming much more accessible to .NET developers. The .NET async model is based on the Task Parallel Library (TPL) - which maintains a pool of threads, each of which runs async tasks. In 2019, it gained channels.

## Async Python

Python gained async support in 2015. Python 3.4 introduced the `asyncio` library, which provides an event loop, coroutines, and tasks. Python 3.5 added the `async` and `await` keywords, making it easier to write asynchronous code.

> You should have noticed a pattern here: async systems are gradually converging on a model of lightweight tasks (green threads) managed by a runtime, with an event loop to handle I/O and scheduling.