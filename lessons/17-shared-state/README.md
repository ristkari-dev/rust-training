# Lesson 17 — Shared state

Sometimes threads must share the same mutable data, not just pass
messages. Sharing plus mutation is exactly what causes data races — so
Rust makes you combine shared ownership (`Arc`) with a lock (`Mutex`) to
do it safely. This lesson covers `Mutex`, the `Arc<Mutex<T>>` pattern,
and `RwLock`.

## Learning goals

- Use `Mutex<T>`: `.lock()`, mutate through the guard, and rely on the
  guard's `Drop` to release the lock
- Explain that the `MutexGuard` is RAII — the lock is held exactly as
  long as the guard is in scope
- Recognize that `Arc<T>` alone is shared but immutable, so shared
  mutable state needs `Arc<Mutex<T>>`
- Build the shared-counter: clone an `Arc<Mutex<_>>` into many threads,
  lock-and-mutate, join, read the exact total
- Describe `RwLock<T>` (many readers or one writer) and when to prefer it

## Self-study notes

### `Mutex<T>` and the guard

A `Mutex<T>` ("mutual exclusion") wraps data so only one thread can touch
it at a time:

```rust
use std::sync::Mutex;

let m = Mutex::new(0);
{
    let mut guard = m.lock().unwrap();  // acquire the lock
    *guard += 1;                         // mutate through the guard
}                                        // guard dropped → lock released
```

`.lock()` blocks until the lock is free and returns a `MutexGuard` that
derefs to the inner value. The lock is held exactly as long as the guard
lives — it's released automatically on drop (RAII), no manual unlock.
`.lock()` returns a `Result` because a thread panicking while holding the
lock *poisons* it; `.unwrap()` is the usual response.

### `Arc<T>` — shared but immutable

From Lesson 16, `Arc<T>` is the thread-safe reference-counted pointer:
many threads can *own* the same value. But it only gives shared (`&T`)
access:

```rust
let a = Arc::new(0);
// *a += 1;  // ERROR: cannot assign through an Arc
```

Shared ownership alone can't mutate — you need a lock inside.

### `Arc<Mutex<T>>` — shared mutable state

Stack them: `Arc` shares ownership across threads, the `Mutex` makes the
inner data safely mutable:

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let clone = Arc::clone(&counter);   // another owner, same Mutex
*clone.lock().unwrap() += 1;
```

This is the thread-safe analogue of Lesson 10's `Rc<RefCell<T>>`.

### The counter pattern

Give each thread its own `Arc` clone, lock-and-increment, then join and
read:

```rust
let counter = Arc::new(Mutex::new(0usize));
for _ in 0..threads {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        *counter.lock().unwrap() += 1;
    }));
}
for h in handles { h.join().unwrap(); }
let total = *counter.lock().unwrap();   // exact, no lost updates
```

The Mutex serializes the increments, so the total is exact no matter how
the threads interleave.

### `RwLock<T>` — readers vs writers

When reads vastly outnumber writes, a `Mutex` (one accessor at a time,
even for reads) is wasteful. `RwLock<T>` allows *many* concurrent readers
**or** one writer:

```rust
use std::sync::RwLock;

let lock = RwLock::new(0);
let r = lock.read().unwrap();    // many readers allowed at once
let w = lock.write().unwrap();   // exclusive
```

Use `Arc<RwLock<T>>` for shared read-heavy state.

## Exercises

### Warm-up: `locked_increment`

Implement `locked_increment(m: &Mutex<i32>, by: i32) -> i32` that locks
the mutex, adds `by` to the value, and returns the new value:

```rust
pub fn locked_increment(m: &Mutex<i32>, by: i32) -> i32 {
    // let mut guard = m.lock().unwrap();
    // *guard += by;
    // *guard
    todo!()
}
```

### Main: `concurrent_counter`

Implement `concurrent_counter(threads: usize, per_thread: usize) -> usize`.
Share an `Arc<Mutex<usize>>` across `threads` threads; each increments it
`per_thread` times; join them all and return the final count:

```rust
pub fn concurrent_counter(threads: usize, per_thread: usize) -> usize {
    // let counter = Arc::new(Mutex::new(0usize));
    // ...clone the Arc into each thread, lock + increment...
    // join all, then: *counter.lock().unwrap()
    todo!()
}
```

You'll add `use std::sync::Arc;` and `use std::thread;` yourself. The
result is exactly `threads * per_thread` — the `Mutex` makes sure no
increments are lost.

### Compile-fail

`exercises/compile_fails/17-arc-no-mutate.rs` shares an `Arc<i32>` across
threads and tries to mutate it directly, which the compiler rejects
(E0594 — you can't mutate through an `Arc`). Fix it by wrapping the value
in a `Mutex` (`Arc<Mutex<i32>>`) and locking it.

### Run

```bash
make verify LESSON=17-shared-state
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
