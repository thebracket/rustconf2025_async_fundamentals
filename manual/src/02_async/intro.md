# Async

Async has often been controversial! From "Async Rust is a Bad Language" to various grumblings---it's a hot topic.

We're going to be tackling some of the controversies throughout. There are two to really keep in mind:

* Almost every Async implementation has a garbage collector (GC). Rust does not - and you often pay for this with `Arc` as a reference counted pointer.
* Async is *viral*, coloring your functions. Once you start using it, it tends to spread. That may be fine for your project, or it may not. Understanding the implications is *really* important to your project.