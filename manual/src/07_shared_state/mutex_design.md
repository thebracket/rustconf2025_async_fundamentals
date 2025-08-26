# Mutex: Design

Please don't do this outside of toy workshop examples:

```rust
pub static MY_SHARED_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
```

You are *trusting* every single consumer of the shared cache to do the right thing, not deadlock, and not hold the lock for too long. This is a recipe for disaster.

Instead, encapsulate the shared state in a type, and provide methods that do the right thing. This way, you can control how the Mutex is used, and ensure that it's used correctly.

```rust
use std::collections::HashMap;
use std::sync::Mutex;

struct SharedCache {
    cache: Mutex<HashMap<String, String>>,
}

impl SharedCache {
    // Public New, access methods, etc.
}
```

Now you only have to get it right in one place. You have a single place to review, to add logging, metrics, etc. You can even change the implementation later (to a more complex locking scheme, or a different data structure) without changing the public API. You could even change the functions to call a completely different microservice, without changing the public API.