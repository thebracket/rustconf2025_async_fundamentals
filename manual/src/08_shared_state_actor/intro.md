# Let's Make a Shared State Actor

Using what we've covered, make a new "library" crate called `shared_state_actor`. It has:

* An initialization function that starts an actor and returns a channel sender.
* A function to increment the actor's state.
* A function to get the actor's current state.

You can use `#[tokio::test]` to write tests for your library.

---

My version is in `code/shared_state_actor`. I added lots of unit tests!