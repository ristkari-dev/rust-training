# Shared state

> Sometimes threads must share the same mutable data, not just pass messages. Sharing plus mutation is exactly what causes data races — so Rust makes you combine shared ownership (`Arc`) with a lock (`Mutex`) to do it safely.

---

## Recap — why shared state

Lesson 16 preferred message passing: move ownership between threads, nothing shared.

But some problems — a counter, a cache, an accumulator — genuinely need many threads touching one piece of mutable data. That's where locks come in.

---

## `Mutex<T>` — mutual exclusion

```rust
use std::sync::Mutex;

let m = Mutex::new(0);
{
    let mut guard = m.lock().unwrap();  // acquire the lock
    *guard += 1;                         // mutate through the guard
}                                        // guard dropped → lock released
```

A `Mutex<T>` wraps data so only one thread can access it at a time. `.lock()` blocks until the lock is free and returns a *guard*.

---

## The guard is RAII

`.lock()` returns a `MutexGuard<T>` that derefs to the inner `T`. The lock is held exactly as long as the guard lives, and released automatically when it goes out of scope — no manual unlock, no forgetting.

`.lock()` returns a `Result` because a thread that panics while holding the lock *poisons* it; `.unwrap()` is the usual response.

---

## `Arc<T>` — shared but immutable

From Lesson 16: `Arc<T>` lets many threads *own* the same value. But it only hands out *shared* (`&T`) access:

```rust
let a = Arc::new(0);
// *a += 1;  // ERROR: cannot assign through an Arc
```

Shared ownership alone can't mutate. You need a lock inside.

---

## `Arc<Mutex<T>>` — shared *and* mutable

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let clone = Arc::clone(&counter);   // another owner, same Mutex
*clone.lock().unwrap() += 1;
```

`Arc` shares ownership across threads; the `Mutex` inside makes the data safely mutable. This is the thread-safe analogue of Lesson 10's `Rc<RefCell<T>>`.

---

## The counter pattern

```rust
let counter = Arc::new(Mutex::new(0usize));
for _ in 0..threads {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        for _ in 0..per_thread {
            *counter.lock().unwrap() += 1;
        }
    }));
}
for h in handles { h.join().unwrap(); }
let total = *counter.lock().unwrap();   // exactly threads * per_thread
```

The Mutex serializes the increments, so the final count is exact — no lost updates.

---

## `RwLock<T>` — readers vs writers

When reads vastly outnumber writes, a `Mutex` (one accessor at a time, even for reads) is wasteful. `RwLock<T>` allows **many** concurrent readers **or** one writer:

```rust
let lock = RwLock::new(0);
let r = lock.read().unwrap();    // many readers allowed
let w = lock.write().unwrap();   // exclusive
```

Use `Arc<RwLock<T>>` for shared read-heavy state.

---

## Putting it together

Today's exercises:

- **Warm-up** `locked_increment` — lock a `Mutex<i32>`, add, return the new value
- **Main** `concurrent_counter` — share an `Arc<Mutex<usize>>` across N threads, each incrementing, then join and read the exact total

The compile-fail shares an `Arc<i32>` and tries to mutate it — the error that `Arc<Mutex<i32>>` fixes.

---

## Wrap — safe shared mutation

- `Mutex<T>` gives one thread at a time access via a guard
- The guard releases the lock on drop (RAII)
- `Arc` shares ownership but is immutable alone
- `Arc<Mutex<T>>` is shared mutable state (thread-safe `Rc<RefCell<T>>`)
- `RwLock` allows many readers or one writer

Next: **Lesson 18 — Async/await with Tokio**.
