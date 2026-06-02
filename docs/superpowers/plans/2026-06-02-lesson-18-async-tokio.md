# Lesson 18 — Async/await with Tokio — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the third lesson of Phase 4 of the Rust training course: async/await with Tokio. An `async fn` returns a lazy `Future`; `.await` runs it; Tokio is the runtime. Warm-up: `sum_doubled` (awaits two spawned tasks). Main: `concurrent_sum_of_squares` (spawn-per-value fan-out, the async twin of L16). Compile-fail: forgetting `.await` (E0308 — got a `Future`, not the value). This is the second lesson whose graded crate uses an external dependency (`tokio`) and the first to use an async test harness (`#[tokio::test]`).

**Architecture:** Use the existing `make new-lesson` scaffolder, then wire the `tokio` workspace dependency as a dedicated task before writing the exercise code. The exercise stub ships both `pub async fn` signatures with `todo!()` bodies, each carrying a documented `#[allow(clippy::unused_async)]` (the body has no `.await` yet; the solution awaits and needs no allow). Tests are `#[tokio::test]` async functions asserting commutative sums, so they are deterministic. All reference code in this plan was empirically verified clippy-pedantic-clean and deterministic across repeated runs during design.

**Tech Stack:** Rust 2024 edition, `tokio` 1.x with `features = ["rt", "macros"]` (new workspace dependency), `#[tokio::test]`, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-06-02-lesson-18-async-tokio-design.md`](../specs/2026-06-02-lesson-18-async-tokio-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

**Commit convention:** Plain commit messages only — no `Co-Authored-By` trailer or any AI attribution. If a commit fails with a GPG/pinentry error, simply retry the same `git commit` command once or twice.

---

## Task 1: Scaffold lessons/18-async-tokio

**Files (all created by the scaffolder):**
- `lessons/18-async-tokio/README.md` (placeholder, replaced in Task 5)
- `lessons/18-async-tokio/slides/index.html` (final — no edit needed)
- `lessons/18-async-tokio/slides/slides.md` (placeholder, replaced in Task 6)
- `lessons/18-async-tokio/exercises/Cargo.toml` (dependency added in Task 2)
- `lessons/18-async-tokio/exercises/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/18-async-tokio/exercises/tests/exercise.rs` (placeholder, replaced in Task 3)
- `lessons/18-async-tokio/solutions/Cargo.toml` (dependency added in Task 2)
- `lessons/18-async-tokio/solutions/src/lib.rs` (placeholder, replaced in Task 4)
- `lessons/18-async-tokio/solutions/tests/exercise.rs` (placeholder, replaced in Task 4)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=18-async-tokio
```

Expected: `scaffolded ./lessons/18-async-tokio`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/18-async-tokio/
ls lessons/18-async-tokio/slides/ lessons/18-async-tokio/exercises/ lessons/18-async-tokio/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/18-async-tokio/exercises/Cargo.toml lessons/18-async-tokio/solutions/Cargo.toml
```

Expected:
```
lessons/18-async-tokio/exercises/Cargo.toml:name = "async-tokio-exercises"
lessons/18-async-tokio/solutions/Cargo.toml:name = "async-tokio-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"async-tokio-[^"]*"' | sort -u
```

Expected output:
```
"name":"async-tokio-exercises"
"name":"async-tokio-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/18-async-tokio
git commit -m "chore: scaffold lessons/18-async-tokio"
```

---

## Task 2: Wire the `tokio` workspace dependency

**Files:**
- Modify: `Cargo.toml` (root — add to `[workspace.dependencies]`)
- Modify: `lessons/18-async-tokio/exercises/Cargo.toml` (add `[dependencies]`)
- Modify: `lessons/18-async-tokio/solutions/Cargo.toml` (add `[dependencies]`)

- [ ] **Step 1: Add `tokio` to the root `[workspace.dependencies]`**

In the root `Cargo.toml`, the `[workspace.dependencies]` section currently reads:

```toml
[workspace.dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
thiserror = "2"
toml_edit = "0.22"
walkdir = "2"
tempfile = "3"
tiny_http = "0.12"
```

Add a `tokio` line so it becomes:

```toml
[workspace.dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
thiserror = "2"
tokio = { version = "1", features = ["rt", "macros"] }
toml_edit = "0.22"
walkdir = "2"
tempfile = "3"
tiny_http = "0.12"
```

- [ ] **Step 2: Add the dependency to `lessons/18-async-tokio/exercises/Cargo.toml`**

The scaffolded file ends with a `[lints]` section. Append a `[dependencies]` section after it so the whole file reads:

```toml
[package]
name = "async-tokio-exercises"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true

[dependencies]
tokio = { workspace = true }
```

- [ ] **Step 3: Add the dependency to `lessons/18-async-tokio/solutions/Cargo.toml`**

Same change for the solutions crate — append `[dependencies]` after `[lints]`:

```toml
[package]
name = "async-tokio-solutions"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true

[dependencies]
tokio = { workspace = true }
```

- [ ] **Step 4: Build to resolve `tokio`**

```bash
cargo build --workspace
```

Expected: cargo downloads `tokio` and its dependency tree (if not already cached) and the workspace builds warning-free. (The two lesson crates compile even though their `src/lib.rs` is still the scaffolded placeholder — they just don't use `tokio` yet, which is fine.)

- [ ] **Step 5: Verify `tokio` is now available to the lesson crates**

```bash
cargo tree --package async-tokio-solutions --depth 1 | grep tokio
```

Expected: shows a `tokio v1.x.y` dependency line.

- [ ] **Step 6: Commit**

Note: `Cargo.lock` is gitignored in this repo (CI regenerates it), so do NOT attempt to add it. Commit only the three manifests.

```bash
git add Cargo.toml lessons/18-async-tokio/exercises/Cargo.toml lessons/18-async-tokio/solutions/Cargo.toml
git commit -m "build(lesson-18): add tokio workspace dependency"
```

---

## Task 3: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/18-async-tokio/exercises/src/lib.rs`
- Overwrite: `lessons/18-async-tokio/exercises/tests/exercise.rs`
- Create: `lessons/18-async-tokio/exercises/compile_fails/18-forgot-await.rs`

- [ ] **Step 1: Overwrite `lessons/18-async-tokio/exercises/src/lib.rs`**

The `#[allow(clippy::unused_async)]` attributes are REQUIRED here: both functions must be `async` (the tests `.await` them), but the `todo!()` bodies contain no `.await`, which would otherwise trip clippy's `unused_async`. Write the file exactly as shown, including the comment:

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

- [ ] **Step 2: Overwrite `lessons/18-async-tokio/exercises/tests/exercise.rs`**

```rust
use async_tokio_exercises::{concurrent_sum_of_squares, sum_doubled};

// Warm-up: sum_doubled (async fn + spawn + .await)

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

// Main: concurrent_sum_of_squares (spawn-per-value fan-out)

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

- [ ] **Step 3: Create `lessons/18-async-tokio/exercises/compile_fails/18-forgot-await.rs`**

The `compile_fails/` directory does not exist yet — create it. This file is self-contained and does NOT use `tokio` (rustc compiles `async fn` natively, and the futures are never run). Write this file:

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

- [ ] **Step 4: Verify exercise tests compile and fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/18-async-tokio/exercises/Cargo.toml
```

Expected: the crate COMPILES, then all 8 tests FAIL with `not yet implemented` panic message. (Compilation succeeding while tests panic is the correct undone state — the signatures are complete; only the bodies are `todo!()`.)

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package async-tokio-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/18-async-tokio
```

Expected: prints `ok: lessons/18-async-tokio/exercises/compile_fails/18-forgot-await.rs` and exits 0. (The tool printing the rustc E0308 error text is expected — what matters is the final `ok:` line and exit 0.)

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/18-async-tokio
```

Expected: non-zero exit with a `FAIL: file did not compile, but was expected to: lessons/18-async-tokio/...` message. (This is correct — the file ships broken on purpose.)

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package async-tokio-exercises --all-targets -- -D warnings
cargo fmt --check --package async-tokio-exercises
```

Expected: both exit 0. (The `#[allow(clippy::unused_async)]` attributes keep the `todo!()` async stubs clean; the tests lint clean — verified during design.)

- [ ] **Step 9: Commit**

```bash
git add lessons/18-async-tokio/exercises
git commit -m "feat(lesson-18): add async exercise stubs, tests, and compile-fail"
```

---

## Task 4: Reference solutions

**Files:**
- Overwrite: `lessons/18-async-tokio/solutions/src/lib.rs`
- Overwrite: `lessons/18-async-tokio/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/18-async-tokio/solutions/src/lib.rs`**

The solution functions await, so they need NO `#[allow]` and NO `#[must_use]` (an `async fn` already returns a `#[must_use]` `Future`). Write exactly as shown:

```rust
//! Lesson 18 — reference solutions.

pub async fn sum_doubled(a: i32, b: i32) -> i32 {
    let doubled_a = tokio::spawn(async move { a * 2 }).await.unwrap();
    let doubled_b = tokio::spawn(async move { b * 2 }).await.unwrap();
    doubled_a + doubled_b
}

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

> Pedagogical notes:
> - `sum_doubled`: spawns two concurrent tasks (each doubling an argument) and `.await`s each `JoinHandle` (which yields a `Result`, unwrapped) to get the value. Spawning rather than calling a trivially-`async` helper is deliberate — a bare `async fn double(n) { n * 2 }` with no `.await` would trip `clippy::unused_async`.
> - `concurrent_sum_of_squares`: spawns one task per value (squaring it) — they run concurrently — collecting `JoinHandle`s first, then `.await`s each and accumulates. The sum is commutative, so the result is deterministic regardless of task-completion order (verified stable across repeated runs). The explicit `for` loops are required because `.await` can't appear in a standard iterator closure.
> - NO `#[must_use]` and NO `#[allow]` attributes: an `async fn` already returns a `#[must_use]` `Future`, and these bodies await (so `unused_async` does not fire). Do not add either. If clippy fires unexpectedly, fix the code rather than adding an allow, and report the deviation.

- [ ] **Step 2: Overwrite `lessons/18-async-tokio/solutions/tests/exercise.rs`**

```rust
use async_tokio_solutions::{concurrent_sum_of_squares, sum_doubled};

// Warm-up: sum_doubled (async fn + spawn + .await)

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

// Main: concurrent_sum_of_squares (spawn-per-value fan-out)

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

- [ ] **Step 2.5: (note) the solutions and exercises tests are identical except the crate name in the `use` line** (`async_tokio_solutions` vs `async_tokio_exercises`).

- [ ] **Step 3: Verify solution tests pass (run TWICE to confirm determinism)**

```bash
cargo test --package async-tokio-solutions
cargo test --package async-tokio-solutions
```

Expected: 8 tests pass, BOTH times. (The tasks run concurrently, but every assertion is an exact, order-independent sum.)

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package async-tokio-solutions --all-targets -- -D warnings
cargo fmt --check --package async-tokio-solutions
```

Expected: both exit 0. No `#[allow]` and no `#[must_use]` attributes. The code above is exactly correct as written — do NOT modify it (keep the spawn-based warm-up and the explicit accumulation loops). If clippy fires on anything, do NOT add an `#[allow]` and do NOT change the code; instead STOP and report the exact clippy output.

- [ ] **Step 5: Commit**

```bash
git add lessons/18-async-tokio/solutions
git commit -m "feat(lesson-18): add reference solutions"
```

---

## Task 5: Lesson README

**Files:**
- Overwrite: `lessons/18-async-tokio/README.md`

- [ ] **Step 1: Overwrite `lessons/18-async-tokio/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 18` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/18-async-tokio/README.md
grep -c '^### ' lessons/18-async-tokio/README.md
grep -c '^```' lessons/18-async-tokio/README.md
```

Expected:
- First line: `# Lesson 18 — Async/await with Tokio`
- `grep -c '^### '` returns `9` (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns `14` (7 code blocks × 2 fence lines — the "Async vs threads" self-study subsection and the "Compile-fail" exercise subsection are prose only)

If either count is wrong, the file content is off — re-check it against the content above and fix before committing.

- [ ] **Step 3: Commit**

```bash
git add lessons/18-async-tokio/README.md
git commit -m "docs(lesson-18): write self-study notes"
```

---

## Task 6: Slide deck

**Files:**
- Overwrite: `lessons/18-async-tokio/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/18-async-tokio/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Async/await with Tokio` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 18**

```bash
make slides-build
test -f dist/lessons/18-async-tokio/slides/slides.md
test -f dist/lessons/18-async-tokio/slides/index.html
grep -c "18-async-tokio" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "18-async-tokio"` returns at least 1. (The build-index master registry already has lesson 18 registered with slug `async-tokio`, matching this directory, so it renders as a clickable link.)

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/18-async-tokio/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/18-async-tokio/slides/slides.md
git commit -m "feat(lesson-18): write slide deck"
```

---

## Task 7: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds (now including `tokio`), default-members tests pass (now includes the 8 new async tests in `async-tokio-solutions`), compile-fail `--expect broken` passes for lesson 18.

- [ ] **Step 2: `make verify LESSON=18-async-tokio` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=18-async-tokio || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "18-async-tokio" dist/index.html
```

Expected: `dist/lessons/` contains all eighteen lessons. `grep -c "18-async-tokio"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 18 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/18-async-tokio/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/18-async-tokio/` exists with all four parts
- Root `Cargo.toml` `[workspace.dependencies]` includes
  `tokio = { version = "1", features = ["rt", "macros"] }`; both lesson
  `Cargo.toml`s declare `tokio = { workspace = true }`
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `sum_doubled` / `concurrent_sum_of_squares` async signatures (exercise
  ships `todo!()` bodies with the documented `unused_async` allow;
  solution ships real bodies with no allow)
- `cargo test --package async-tokio-solutions` → 8 passing tests
  (deterministic across repeated runs)
- `cargo test --manifest-path lessons/18-async-tokio/exercises/Cargo.toml`
  → compiles, 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/18-async-tokio` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/18-async-tokio` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/18-async-tokio/slides/index.html`
- `dist/index.html` lists lesson 18 as a clickable link
- All changes committed and pushed (plain commit messages, no co-author trailer)
- Deployed site returns HTTP 200 for `/` and `/lessons/18-async-tokio/slides/`
