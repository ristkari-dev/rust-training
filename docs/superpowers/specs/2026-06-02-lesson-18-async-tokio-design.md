# Lesson 18 — Async/await with Tokio — design

The third and final lesson of Phase 4 (Concurrency). The async model: an
`async fn` returns a lazy `Future` that does nothing until `.await`ed,
and a runtime (Tokio) drives futures and schedules lightweight tasks.
Deliberately mirrors Lesson 16 (threads) so students see threads vs async
tasks side by side — the same spawn-and-collect pattern, with cheaper
tasks. This is the second lesson whose graded crate uses an external
dependency, and the first to use an async test harness. Closes Phase 4.

## Audience and prerequisites

- Has completed Lessons 01-17
- Comfortable with threads/`spawn`/`join` (L16), `Arc`/`Mutex` (L17),
  closures, `Vec`, and `Result`/`unwrap` (L14)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Write an `async fn` and explain that calling it returns a lazy
   `Future` — nothing runs until it is `.await`ed
2. Use `.await` to drive a future to completion (only inside an `async`
   function or block)
3. Recognize that futures need a runtime to run, and that `#[tokio::main]`
   / `#[tokio::test]` provide one
4. Spawn a concurrent task with `tokio::spawn` and get its result by
   `.await`ing the returned `JoinHandle`
5. Contrast async tasks (lightweight, cooperatively scheduled, for
   I/O-bound concurrency) with OS threads (heavier, for CPU-bound
   parallelism)

## Scope

In scope: `async fn` and the `Future` it returns (lazy until awaited);
the `.await` operator; the need for a runtime and `#[tokio::main]` /
`#[tokio::test]`; `tokio::spawn` and awaiting a `JoinHandle`; the
spawn-many-then-await-all concurrency pattern; the async-vs-threads
trade-off (conceptual). New infrastructure: a `tokio` workspace
dependency (`features = ["rt", "macros"]`) and async tests via
`#[tokio::test]`. The exercises drill `.await` on spawned tasks
(warm-up) and a spawn-per-value fan-out/collect (main) — the async twin
of Lesson 16's thread version.

Out of scope (deferred or skipped): writing manual `Future` impls,
`Poll`, `Pin`, wakers, and the executor internals (mentioned only as
"the runtime drives futures"); `tokio::join!`/`select!` macros (the
slides mention `join!` exists; `spawn` is the exercised primitive);
`async`-aware synchronization (`tokio::sync::Mutex`, channels) beyond a
pointer; `tokio::time` / real `sleep` (the exercises do pure async
computation to stay fast and deterministic); cancellation,
`spawn_blocking`, streams, and `async` traits; `#[tokio::main]` flavors
and multi-thread runtime tuning. Async is introduced as *`async`/`.await`
+ `tokio::spawn` on a runtime*; the futures-internals and the wider async
ecosystem are out of band.

## New dependency infrastructure

The second graded crate to depend on an external crate (after L14's
`thiserror`). Handling:

- Add `tokio = { version = "1", features = ["rt", "macros"] }` to the
  root `[workspace.dependencies]`. `rt` provides the runtime that
  `tokio::spawn` needs; `macros` provides `#[tokio::main]`/`#[tokio::test]`.
  (Verified sufficient during design — resolves to tokio 1.52.x.) The
  default `#[tokio::test]` runtime is current-thread, which runs spawned
  tasks fine.
- Both lesson crates declare `tokio = { workspace = true }` in their
  `[dependencies]` (inheriting the workspace feature set).
- `Cargo.lock` is gitignored (the established project pattern); CI
  fetches `tokio` and its dependency tree from crates.io. The cache key
  already hashes `**/Cargo.toml`, so the new dep refetches cleanly. The
  first CI run compiles tokio (a modest one-time cost).

## Slide arc (10 slides)

1. **Title — Async/await with Tokio.** Hook: *"Threads are great for
   CPU-bound work, but heavy when you need thousands of concurrent I/O
   operations. Async tasks are lightweight: an `async fn` is a lazy
   recipe, and a runtime runs many of them on a few threads."*
2. **Why async.** A thread costs an OS stack (~MBs) and a kernel
   context-switch. For I/O-bound concurrency — thousands of sockets, each
   mostly *waiting* — that's wasteful. Async **tasks** are cheap values
   the runtime juggles cooperatively on a small thread pool. Async is for
   *waiting on many things at once*.
3. **`async fn` & `Future`.**
   ```rust
   async fn double(n: i32) -> i32 {
       n * 2
   }

   let fut = double(21);   // nothing has run yet — `fut` is a Future
   ```
   An `async fn` doesn't run its body when called — it returns a
   `Future`, a lazy value describing the work. It's inert until driven.
4. **`.await`.**
   ```rust
   let result = double(21).await;   // now it runs → 42
   ```
   `.await` drives a future to completion and yields its output, handing
   control back to the runtime while the future is waiting. You can only
   `.await` inside an `async fn` or `async` block.
5. **The runtime — Tokio.** A future does nothing on its own; it needs an
   *executor* to poll it. Tokio is the standard async runtime. You start
   it with an attribute:
   ```rust
   #[tokio::main]
   async fn main() {
       println!("{}", double(21).await);
   }
   ```
   Tests use `#[tokio::test]`. Add `tokio` to `Cargo.toml`
   (`features = ["rt", "macros"]`).
6. **`tokio::spawn`.**
   ```rust
   let handle = tokio::spawn(async move { 21 * 2 });
   let result = handle.await.unwrap();   // 42
   ```
   `tokio::spawn` schedules a future as an independent **task** that runs
   concurrently, returning a `JoinHandle`. `.await`ing the handle waits
   for the task and yields its result (as a `Result`). It's the async
   analogue of `thread::spawn` + `join`.
7. **Concurrency pattern.**
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
   Spawn all tasks first (they run concurrently), then await them and
   combine — the same shape as Lesson 16's thread fan-out, but the tasks
   are far cheaper.
8. **Async vs threads.**
   - **Threads** — CPU-bound parallelism; OS-scheduled; heavy (one stack
     each). Reach for these to use many cores on heavy computation.
   - **Async tasks** — I/O-bound concurrency; cooperatively scheduled;
     cheap (thousands on a few threads). Reach for these to wait on many
     I/O operations.

   (They combine: a runtime *is* threads running async tasks.)
9. **Putting it together.** Walk through the exercises: `sum_doubled`
   spawns a task to double each argument and `.await`s both (warm-up —
   `async`/`.await`/`spawn` mechanics); `concurrent_sum_of_squares` spawns
   one task per value, awaits every handle, and sums (main — the fan-out).
   The compile-fail forgets a `.await` and uses a `Future` where an `i32`
   is expected.
10. **Wrap — Phase 4 complete.** Five takeaways: an `async fn` returns a
    lazy `Future`; `.await` runs it (only inside `async`); futures need a
    runtime (`#[tokio::main]`/`#[tokio::test]`); `tokio::spawn` runs a
    concurrent task you `.await` via its `JoinHandle`; async suits
    I/O-bound concurrency, threads suit CPU-bound parallelism. Next:
    **Phase 5 — Lesson 19, Memory, layout, and `unsafe`**.

## Exercise spec

`lessons/18-async-tokio/` follows the standard four-part lesson shape,
plus a dependency in each crate's `Cargo.toml`:

```
18-async-tokio/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml          # adds tokio = { workspace = true }
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/18-forgot-await.rs
└── solutions/
    ├── Cargo.toml          # adds tokio = { workspace = true }
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `async-tokio-exercises` and `async-tokio-solutions`
(the lesson's "bare" name is `async-tokio`; the import idents are
`async_tokio_exercises` / `async_tokio_solutions`). This matches the
build-index master registry slug `async-tokio`, so the landing page links
it without any change.

### Cargo.toml dependency

Both `exercises/Cargo.toml` and `solutions/Cargo.toml` gain, after the
`[lints]` section:

```toml
[dependencies]
tokio = { workspace = true }
```

And the root `Cargo.toml` `[workspace.dependencies]` gains
`tokio = { version = "1", features = ["rt", "macros"] }`.

### Exercise stub (`exercises/src/lib.rs`)

Both functions are `pub async fn` with `todo!()` bodies. Each carries an
`#[allow(clippy::unused_async)]` with an explanatory comment: the function
*must* be `async` (the tests `.await` it), but an unimplemented `todo!()`
body contains no `.await`, which would otherwise trip clippy's
`unused_async` (a denied pedantic lint). The student's finished
implementation contains `.await`, so the allow matters only while the
body is a stub — the reference solution carries no such allow. The crate
and its tests compile; the async tests fail at runtime with the `todo!()`
panic.

```rust
//! Lesson 18 — exercises.
//!
//! Implement `sum_doubled` (warm-up) and `concurrent_sum_of_squares`
//! (main) so that `cargo test --manifest-path
//! lessons/18-async-tokio/exercises/Cargo.toml` passes. Both are async;
//! the tests `.await` them. The tests live in `tests/exercise.rs`.

// `#[allow(clippy::unused_async)]`: these functions MUST be `async` — the
// tests `.await` them — but the unfinished `todo!()` bodies contain no
// `.await` yet, which would otherwise trip clippy's `unused_async`. Your
// finished implementation will contain `.await`, so the allow is only
// needed while the body is a stub.

#[allow(clippy::unused_async)]
pub async fn sum_doubled(_a: i32, _b: i32) -> i32 {
    todo!("spawn a task to double each argument, await both, and return their sum")
}

#[allow(clippy::unused_async)]
pub async fn concurrent_sum_of_squares(_values: Vec<i32>) -> i32 {
    todo!("spawn a task per value to square it, await all handles, and sum the results")
}
```

### Warm-up: `sum_doubled`

Reference solution (no `#[allow]` — it awaits):

```rust
pub async fn sum_doubled(a: i32, b: i32) -> i32 {
    let doubled_a = tokio::spawn(async move { a * 2 }).await.unwrap();
    let doubled_b = tokio::spawn(async move { b * 2 }).await.unwrap();
    doubled_a + doubled_b
}
```

Pedagogical packing: spawns two concurrent tasks, each doubling an
argument, and `.await`s each `JoinHandle` (which returns a `Result`,
unwrapped) to get the value. Teaches `async fn`, `tokio::spawn`, and
`.await` on a handle in the simplest form. Spawning a task (rather than
calling a trivially-`async` helper) is deliberate: a bare
`async fn double(n) { n * 2 }` with no `.await` inside would trip
`clippy::unused_async`. No `#[must_use]` is used: an `async fn` already
returns a `#[must_use]` `Future`, so adding it would be redundant
(verified — the lesson's functions carry no `#[must_use]`).

Four tests (note `#[tokio::test]` and `.await` in each):

```rust
#[tokio::test]
async fn warmup_sum_doubled_basic() {
    assert_eq!(sum_doubled(3, 4).await, 14);
}

#[tokio::test]
async fn warmup_sum_doubled_zero() {
    assert_eq!(sum_doubled(0, 0).await, 0);
}

#[tokio::test]
async fn warmup_sum_doubled_negative() {
    assert_eq!(sum_doubled(-2, 5).await, 6);
}

#[tokio::test]
async fn warmup_sum_doubled_large() {
    assert_eq!(sum_doubled(10, 10).await, 40);
}
```

### Main: `concurrent_sum_of_squares`

Reference solution:

```rust
pub async fn concurrent_sum_of_squares(values: Vec<i32>) -> i32 {
    let mut handles = Vec::new();
    for v in values {
        handles.push(tokio::spawn(async move { v * v }));
    }
    let mut total = 0;
    for handle in handles {
        total += handle.await.unwrap();
    }
    total
}
```

Pedagogical packing: the async fan-out/collect, mirroring Lesson 16's
`parallel_sum_of_squares` thread version. Spawn one task per value
(squaring it) — they run concurrently — collecting the `JoinHandle`s
first, then `.await` each and accumulate. The sum is commutative, so the
result is deterministic regardless of task-completion order (verified
stable across repeated runs during design). The explicit accumulation
loop is used because `.await` can't appear in a standard iterator
closure.

Four tests:

```rust
#[tokio::test]
async fn main_sum_empty() {
    assert_eq!(concurrent_sum_of_squares(vec![]).await, 0);
}

#[tokio::test]
async fn main_sum_one() {
    assert_eq!(concurrent_sum_of_squares(vec![3]).await, 9);
}

#[tokio::test]
async fn main_sum_many() {
    assert_eq!(concurrent_sum_of_squares(vec![1, 2, 3]).await, 14);
}

#[tokio::test]
async fn main_sum_negatives() {
    assert_eq!(concurrent_sum_of_squares(vec![-2, 4]).await, 20);
}
```

**Eight tests total** (four warm-up + four main), all `#[tokio::test]`.
Test arithmetic: `sum_doubled(3, 4)` → `6 + 8 = 14`; `sum_doubled(-2, 5)`
→ `-4 + 10 = 6`; `concurrent_sum_of_squares([1,2,3])` → `1 + 4 + 9 = 14`;
`([-2,4])` → `4 + 16 = 20`. All assertions are exact, order-independent
sums, so the tests are deterministic.

### Compile-fail: `18-forgot-await.rs`

Path: `exercises/compile_fails/18-forgot-await.rs`. A self-contained file
(no `tokio` — rustc compiles `async fn` natively, and the tasks are never
run) whose `async fn` uses an async call's result without `.await`-ing it.
Ships broken; the student adds `.await`.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Calling an `async fn` does NOT run it — it returns a `Future`, a lazy
// value that produces the result only once you `.await` it. So
// `double(21)` is a `Future`, not an `i32`. Binding it to `let doubled:
// i32` is a type mismatch: you asked for an `i32` but got a future.
//
// rustc reports E0308 ("mismatched types: expected `i32`, found future")
// and even suggests the fix: `.await` the future.
//
// The fix: `.await` the call so the future runs and yields its `i32`.
// (`.await` is allowed here because `run` is itself `async`.)
//
// Hint: change `double(21)` to `double(21).await`.

async fn double(n: i32) -> i32 {
    n * 2
}

async fn run() -> i32 {
    let doubled: i32 = double(21);
    doubled
}

fn main() {
    // `run()` returns a future; the type error inside `run`'s body is a
    // compile error whether or not we ever run it.
    let _ = run();
}
```

Pass condition: the student changes `double(21)` to `double(21).await`.
rustc reports E0308 "mismatched types: expected `i32`, found future" with
a "consider `await`ing on the `Future`" suggestion — verified during
design. After the fix the file compiles.

This is the lesson's centerpiece: it isolates the single most common
async beginner mistake and the core mental model — an `async fn` returns
a lazy `Future`, and you must `.await` it to get the value.

## README structure

`lessons/18-async-tokio/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - `async fn` and `Future` — lazy work
  - `.await` — running a future
  - The runtime — Tokio
  - `tokio::spawn` — concurrent tasks
  - Async vs threads — when to use which
- **Exercises** — four subsections: Warm-up (`sum_doubled`), Main
  (`concurrent_sum_of_squares`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`tokio::spawn`" and "Async vs threads" sections are the heaviest — they
carry the lesson's payoff and the key decision.

## Lint expectations

Lesson 18's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design:

- The reference functions carry **no `#[must_use]`**: an `async fn`
  already returns a `#[must_use]` `Future`, so adding the attribute is
  redundant (and the lesson follows that).
- The warm-up spawns tasks rather than calling a trivially-`async`
  helper, so `clippy::unused_async` does not fire on the solution.
- `concurrent_sum_of_squares` uses explicit `for` loops (spawn, then
  accumulate) because `.await` cannot appear in a standard iterator
  closure — no clippy lint prefers an iterator here.
- The **exercise stub** carries `#[allow(clippy::unused_async)]` on each
  function (the only allow in the lesson), because the `todo!()` bodies
  have no `.await`; this is documented inline. The solution has no allow
  (verified clippy-clean).
- Tests use `#[tokio::test]` and are `async fn`s that `.await` the
  exercise functions.

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/18-async-tokio/` exists with the four-part structure
- Root `Cargo.toml` `[workspace.dependencies]` includes
  `tokio = { version = "1", features = ["rt", "macros"] }`; both lesson
  `Cargo.toml`s declare `tokio = { workspace = true }`
- Cargo manifests use the correct package names (`async-tokio-exercises`,
  `async-tokio-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `sum_doubled` / `concurrent_sum_of_squares` async signatures; the
  exercise ships `todo!()` bodies (with the documented `unused_async`
  allow), the solution ships real bodies (no allow)
- `cargo test --package async-tokio-solutions` → 8 tests pass
  (deterministic across repeated runs)
- `cargo test --manifest-path lessons/18-async-tokio/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/18-async-tokio`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/18-async-tokio`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/18-async-tokio/slides/index.html`
- `dist/index.html` lists lesson 18 as a clickable link (registry slug
  `async-tokio` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/18-async-tokio/slides/`
  returns 200

## Open questions

None.
