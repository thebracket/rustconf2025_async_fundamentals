# Shared State

Shared state is hard to avoid. Arguably *state* is the whole point of a program, and it's relatively rare to have a program that doesn't share state of some sort. It might be resources (database connections, file handles, network connections), or it might be data (caches, in-memory databases, etc). It might even be something as simple as usage counters or statistics.

Either way, sharing state between tasks is a common requirement. It's also a common source of problems!