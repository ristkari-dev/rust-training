# Lesson 18 — Async/await with Tokio

Threads are great for CPU-bound work, but heavy when you need thousands
of concurrent I/O operations. Async tasks are lightweight: an `async fn`
is a lazy recipe, and a runtime (Tokio) runs many of them on a few
threads. This lesson covers `async`/`.await`, the runtime, and
`tokio::spawn`. It closes Phase 4.

## Learning goals

- Write an `async fn` and explain that calling it returns a lazy
  `Future` — nothing runs until it is `.await`ed
- Use `.await` to drive a future to completion (only inside an `async`
  function or block)
- Recognize that futures need a runtime, provided by `#[tokio::main]` /
  `#[tokio::test]`
- Spawn a concurrent task with `tokio::spawn` and get its result by
  `.await`ing the `JoinHandle`
- Contrast async tasks (lightweight, I/O-bound concurrency) with OS
  threads (heavier, CPU-bound parallelism)

## Self-study notes

### `async fn` and `Future` — lazy work

An `async fn` doesn't run its body when called — it returns a `Future`,
a lazy value describing the work:

```rust
async fn double(n: i32) -> i32 {
    n * 2
}

let fut = double(21);   // nothing has run yet — `fut` is a Future
```

The future is inert until something drives it. This is different from a
normal function call, which runs immediately.

### `.await` — running a future

`.await` drives a future to completion and yields its output, handing
control back to the runtime while the future is waiting:

```rust
let result = double(21).await;   // now it runs → 42
```

You can only `.await` inside an `async fn` or `async` block — that's the
context that knows how to suspend and resume.

### The runtime — Tokio

A future does nothing on its own; it needs an *executor* to poll it.
Tokio is the standard async runtime. Start it with an attribute:

```rust
#[tokio::main]
async fn main() {
    println!("{}", double(21).await);
}
```

Tests use `#[tokio::test]`. You add `tokio` to `Cargo.toml` with the
features you need (here, `["rt", "macros"]`).

### `tokio::spawn` — concurrent tasks

`tokio::spawn` schedules a future as an independent **task** that runs
concurrently, returning a `JoinHandle`:

```rust
let handle = tokio::spawn(async move { 21 * 2 });
let result = handle.await.unwrap();   // 42
```

`.await`ing the handle waits for the task and yields its result (as a
`Result`). It's the async analogue of `thread::spawn` + `join`.

### Async vs threads — when to use which

- **Threads** — CPU-bound parallelism; OS-scheduled; heavy (a stack
  each). Use them to put many cores to work on heavy computation.
- **Async tasks** — I/O-bound concurrency; cooperatively scheduled;
  cheap (thousands on a few threads). Use them to wait on many I/O
  operations at once.

They combine: a runtime is really threads running async tasks.

## Exercises

### Warm-up: `sum_doubled`

Implement `async fn sum_doubled(a: i32, b: i32) -> i32` that spawns a
task to double each argument, awaits both, and returns their sum:

```rust
pub async fn sum_doubled(a: i32, b: i32) -> i32 {
    // let doubled_a = tokio::spawn(async move { a * 2 }).await.unwrap();
    // ...same for b...
    // doubled_a + doubled_b
    todo!()
}
```

The stub ships with `#[allow(clippy::unused_async)]` only because the
`todo!()` body has no `.await` yet — your implementation will, so you can
leave the allow or remove it once done.

### Main: `concurrent_sum_of_squares`

Implement `async fn concurrent_sum_of_squares(values: Vec<i32>) -> i32`.
Spawn one task per value to square it, await all the handles, and sum the
results:

```rust
pub async fn concurrent_sum_of_squares(values: Vec<i32>) -> i32 {
    // let mut handles = Vec::new();
    // for v in values { handles.push(tokio::spawn(async move { v * v })); }
    // sum each handle.await.unwrap()
    todo!()
}
```

This is the async twin of Lesson 16's thread version. The sum is
order-independent, so the result is the same no matter how the tasks
interleave.

### Compile-fail

`exercises/compile_fails/18-forgot-await.rs` calls an `async fn` and uses
its result as a plain value without `.await` — the compiler rejects it
(E0308: an `async fn` returns a `Future`, not the value). Fix it by
adding `.await`.

### Run

```bash
make verify LESSON=18-async-tokio
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
