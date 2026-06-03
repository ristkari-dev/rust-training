# Async/await with Tokio

> Threads are great for CPU-bound work, but heavy when you need thousands of concurrent I/O operations. Async tasks are lightweight: an `async fn` is a lazy recipe, and a runtime runs many of them on a few threads.

---

## Why async

A thread costs an OS stack (~MBs) and a kernel context-switch. For I/O-bound concurrency — thousands of sockets, each mostly *waiting* — that's wasteful.

Async **tasks** are cheap values the runtime juggles cooperatively on a small thread pool. Async is for *waiting on many things at once*.

---

## `async fn` & `Future`

```rust
async fn double(n: i32) -> i32 {
    n * 2
}

let fut = double(21);   // nothing has run yet — `fut` is a Future
```

An `async fn` doesn't run its body when called — it returns a `Future`, a lazy value describing the work. It's inert until driven.

---

## `.await`

```rust
let result = double(21).await;   // now it runs → 42
```

`.await` drives a future to completion and yields its output, handing control back to the runtime while the future is waiting. You can only `.await` inside an `async fn` or `async` block.

---

## The runtime — Tokio

A future does nothing on its own; it needs an *executor* to poll it. Tokio is the standard async runtime:

```rust
#[tokio::main]
async fn main() {
    println!("{}", double(21).await);
}
```

Tests use `#[tokio::test]`. Add `tokio` to `Cargo.toml` (`features = ["rt", "macros"]`).

---

## `tokio::spawn`

```rust
let handle = tokio::spawn(async move { 21 * 2 });
let result = handle.await.unwrap();   // 42
```

`tokio::spawn` schedules a future as an independent **task** that runs concurrently, returning a `JoinHandle`. `.await`ing the handle waits for the task and yields its result. It's the async analogue of `thread::spawn` + `join`.

---

## Concurrency pattern

```rust
let mut handles = Vec::new();
for v in values {
    handles.push(tokio::spawn(async move { v * v }));
}
let mut total = 0;
for handle in handles {
    total += handle.await.unwrap();
}
```

Spawn all tasks first (they run concurrently), then await them and combine — the same shape as Lesson 16's thread fan-out, but the tasks are far cheaper.

---

## Async vs threads

- **Threads** — CPU-bound parallelism; OS-scheduled; heavy (one stack each).
- **Async tasks** — I/O-bound concurrency; cooperatively scheduled; cheap (thousands on a few threads).

Reach for threads to use many cores on heavy computation; reach for async to wait on many I/O operations. (They combine: a runtime *is* threads running async tasks.)

---

## Putting it together

Today's exercises:

- **Warm-up** `sum_doubled` — spawn a task to double each argument, `.await` both
- **Main** `concurrent_sum_of_squares` — spawn one task per value, await every handle, sum

The compile-fail forgets a `.await` and uses a `Future` where an `i32` is expected.

---

## Wrap — Phase 4 complete

- An `async fn` returns a lazy `Future`
- `.await` runs it (only inside `async`)
- Futures need a runtime (`#[tokio::main]` / `#[tokio::test]`)
- `tokio::spawn` runs a concurrent task you `.await` via its `JoinHandle`
- Async suits I/O-bound concurrency; threads suit CPU-bound parallelism

Next: **Phase 5 — Lesson 19, Memory, layout, and `unsafe`**.
