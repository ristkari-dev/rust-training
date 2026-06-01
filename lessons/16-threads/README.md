# Lesson 16 — Threads & channels

Rust lets you run code on many threads without data races — the compiler
rejects unsafe sharing before the program ever runs ("fearless
concurrency"). The simplest safe pattern is to spawn threads and pass
messages between them with channels. This lesson opens Phase 4.

## Learning goals

- Spawn a thread with `thread::spawn` and get its result with `.join()`
- Explain why a thread closure needs `move` — the thread may outlive the
  spawning function, so it must own its captures
- Describe `Send` (safe to move to another thread) and `Sync` (safe to
  share by reference); `Rc` is not `Send`, `Arc` is
- Use an `mpsc` channel to pass values between threads
- Collect results from many worker threads by cloning the sender and
  summing what arrives

## Self-study notes

### Spawning threads — `spawn`, `move`, `join`

`thread::spawn` runs a closure on a new OS thread and returns a
`JoinHandle`:

```rust
use std::thread;

let n = 21;
let handle = thread::spawn(move || n * 2);  // move: the thread owns n
let result = handle.join().unwrap();          // wait, get the value
```

The thread may outlive the function that spawned it, so its closure must
**own** what it captures — hence `move`. `join()` blocks until the
thread finishes and returns whatever the closure returned (wrapped in a
`Result`).

### `Send` & `Sync` — the marker traits

Two traits the compiler checks automatically: a type is **`Send`** if
it's safe to *move* to another thread, and **`Sync`** if it's safe to
*share* (`&T`) across threads. Almost everything is `Send`. The notable
exception is `Rc<T>` — its non-atomic reference count would race — so
`Rc` is **not** `Send`. Its thread-safe sibling `Arc<T>` is:

```rust
use std::sync::Arc;
let shared = Arc::new(42);          // Arc is Send + Sync; Rc is neither
```

Send an `Rc` to a thread and the code simply won't compile.

### Channels — `mpsc`

A channel is a one-way queue between threads. `mpsc` means *multiple
producer, single consumer*:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send(42).unwrap();    // producer sends
});
let value = rx.recv().unwrap();   // consumer receives (blocks)
```

`send` hands a value to the other end; `recv` waits for one.

### Multiple producers — cloning the sender

Clone the sender to give each worker thread its own:

```rust
for v in values {
    let tx = tx.clone();
    thread::spawn(move || tx.send(v * v).unwrap());
}
drop(tx);                          // drop the original
let total: i32 = rx.iter().sum();  // ends when ALL senders are gone
```

`rx.iter()` yields values until *every* sender has dropped — so you drop
the original `tx` and let the workers' clones drop as they finish.

### Message passing vs shared state

A slogan Rust borrows from Go: *"Don't communicate by sharing memory;
share memory by communicating."* Channels move ownership of data between
threads, so there's nothing shared to race over. When you genuinely need
shared mutable state, you reach for `Mutex`/`Arc` — that's **Lesson
17**.

## Exercises

### Warm-up: `double_in_thread`

Implement `double_in_thread(n: i32) -> i32` that spawns a thread to
double `n`, then joins it and returns the result:

```rust
pub fn double_in_thread(n: i32) -> i32 {
    // let handle = thread::spawn(move || n * 2);
    // handle.join().unwrap()
    todo!()
}
```

You'll add `use std::thread;` yourself — the stub ships without imports.

### Main: `parallel_sum_of_squares`

Implement `parallel_sum_of_squares(values: Vec<i32>) -> i32`. Spawn one
thread per value to compute its square, send each square down an `mpsc`
channel, and sum everything the receiver collects:

```rust
pub fn parallel_sum_of_squares(values: Vec<i32>) -> i32 {
    // let (tx, rx) = mpsc::channel();
    // ...clone tx into each thread, send v * v...
    // drop(tx); rx.iter().sum()
    todo!()
}
```

The sum is order-independent, so the result is the same no matter how
the threads interleave.

### Compile-fail

`exercises/compile_fails/16-rc-not-send.rs` moves an `Rc` into a spawned
thread, which the compiler rejects (E0277 — `Rc` is not `Send`). Fix it
by switching to `Arc` (`use std::sync::Arc;` and `Arc::new`).

### Run

```bash
make verify LESSON=16-threads
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
