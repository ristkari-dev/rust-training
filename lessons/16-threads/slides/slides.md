# Threads & channels

> Rust lets you run code on many threads without data races — the compiler rejects unsafe sharing before the program ever runs. The simplest safe pattern is to spawn threads and pass messages between them.

---

## Phase 4 — fearless concurrency

Concurrency means doing several things at once. The danger is a **data race**: two threads touching the same memory, at least one writing.

Rust's ownership system plus two marker traits (`Send`/`Sync`) catch races at **compile** time — "fearless concurrency".

---

## Spawning a thread

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("hello from a thread");
});
```

`thread::spawn` takes a closure and runs it on a new OS thread. It returns a `JoinHandle` — your handle to that thread.

---

## `move` and `join`

```rust
let n = 21;
let handle = thread::spawn(move || n * 2);  // move: the thread owns n
let result = handle.join().unwrap();          // wait, get the value
```

The thread may outlive the function that spawned it, so its closure must **own** its captures — hence `move`. `join()` blocks until the thread finishes and returns the closure's value (as a `Result`).

---

## `Send` & `Sync`

Two marker traits the compiler checks automatically:

- **`Send`** — safe to *move* to another thread
- **`Sync`** — safe to *share* (`&T`) across threads

Almost every type is `Send`. A key exception: `Rc<T>` is **not** `Send` (its non-atomic count would race) — its thread-safe sibling `Arc<T>` is.

---

## Channels — `mpsc`

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send(42).unwrap();   // producer sends
});
let value = rx.recv().unwrap();  // consumer receives
```

`mpsc` = *multiple producer, single consumer*. `send` hands a value to the other end; `recv` waits for one.

---

## Multiple producers

```rust
for v in values {
    let tx = tx.clone();                 // each thread gets its own sender
    thread::spawn(move || tx.send(v * v).unwrap());
}
drop(tx);                                 // drop the original
let total: i32 = rx.iter().sum();         // ends when all senders are gone
```

Clone the sender per worker. `rx.iter()` yields values until *every* sender has dropped — so drop the original `tx` and let the clones drop as workers finish.

---

## Message passing vs shared state

A slogan Rust borrows from Go:

> Don't communicate by sharing memory; share memory by communicating.

Channels move ownership of data between threads, so there's nothing shared to race over. When you *do* need shared mutable state, reach for `Mutex`/`Arc` — that's **Lesson 17**.

---

## Putting it together

Today's exercises:

- **Warm-up** `double_in_thread` — spawn, `move`, `join`
- **Main** `parallel_sum_of_squares` — one thread per value, each sends its square down an `mpsc` channel, main sums them

The compile-fail moves an `Rc` into a thread and hits the `Send` error.

---

## Wrap — Phase 4 begins

- `thread::spawn` runs a closure on a new thread
- `move` gives it ownership; `join` returns its result
- `Send`/`Sync` let the compiler reject data races (`Rc` isn't `Send`, `Arc` is)
- `mpsc` channels pass messages between threads
- Prefer message passing over shared memory

Next: **Lesson 17 — Shared state** (`Mutex`, `RwLock`, `Arc<Mutex<T>>`).
