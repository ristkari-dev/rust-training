# Lesson 16 — Threads & channels — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the first lesson of Phase 4 of the Rust training course: threads & channels. Rust's "fearless concurrency" via message passing. Warm-up: `double_in_thread` (spawn/move/join). Main: `parallel_sum_of_squares` (one thread per value, mpsc channel, deterministic sum). Compile-fail: moving an `Rc` into a thread (E0277 — not `Send`), fixed with `Arc`.

**Architecture:** Use the existing `make new-lesson` scaffolder. The exercise stub ships the two function signatures with `todo!()` bodies and NO `use` statements (unused imports with `todo!()` bodies would trip the `unused` deny lint — students add the imports as they implement). The solution carries the needed `use` statements. Tests assert commutative sums, so they are deterministic despite the concurrency. All reference code in this plan was empirically verified clippy-pedantic-clean and deterministic across repeated runs during design.

**Tech Stack:** Rust 2024 edition, `std::thread` + `std::sync::mpsc` (no external dependencies), existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-29-lesson-16-threads-design.md`](../specs/2026-05-29-lesson-16-threads-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

**Commit convention:** Plain commit messages only — no `Co-Authored-By` trailer or any AI attribution.

---

## Task 1: Scaffold lessons/16-threads

**Files (all created by the scaffolder):**
- `lessons/16-threads/README.md` (placeholder, replaced in Task 4)
- `lessons/16-threads/slides/index.html` (final — no edit needed)
- `lessons/16-threads/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/16-threads/exercises/Cargo.toml` (final — no edit needed)
- `lessons/16-threads/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/16-threads/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/16-threads/solutions/Cargo.toml` (final — no edit needed)
- `lessons/16-threads/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/16-threads/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=16-threads
```

Expected: `scaffolded ./lessons/16-threads`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/16-threads/
ls lessons/16-threads/slides/ lessons/16-threads/exercises/ lessons/16-threads/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/16-threads/exercises/Cargo.toml lessons/16-threads/solutions/Cargo.toml
```

Expected:
```
lessons/16-threads/exercises/Cargo.toml:name = "threads-exercises"
lessons/16-threads/solutions/Cargo.toml:name = "threads-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"threads-[^"]*"' | sort -u
```

Expected output:
```
"name":"threads-exercises"
"name":"threads-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/16-threads
git commit -m "chore: scaffold lessons/16-threads"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/16-threads/exercises/src/lib.rs`
- Overwrite: `lessons/16-threads/exercises/tests/exercise.rs`
- Create: `lessons/16-threads/exercises/compile_fails/16-rc-not-send.rs`

- [ ] **Step 1: Overwrite `lessons/16-threads/exercises/src/lib.rs`**

Note: this stub deliberately has NO `use` statements — with `todo!()` bodies, unused imports would fail the workspace's `unused` deny lint. Students add `use std::thread;` / `use std::sync::mpsc;` as they implement.

```rust
//! Lesson 16 — exercises.
//!
//! Implement `double_in_thread` (warm-up) and `parallel_sum_of_squares`
//! (main) so that `cargo test --manifest-path
//! lessons/16-threads/exercises/Cargo.toml` passes. You'll need to add
//! `use` statements (e.g. `std::thread`, `std::sync::mpsc`) as you go.
//! The tests live in `tests/exercise.rs`.

#[must_use]
pub fn double_in_thread(_n: i32) -> i32 {
    todo!("spawn a thread that doubles n, then join it and return the result")
}

#[must_use]
pub fn parallel_sum_of_squares(_values: Vec<i32>) -> i32 {
    todo!("spawn a thread per value to square it, send results over an mpsc channel, and sum them")
}
```

- [ ] **Step 2: Overwrite `lessons/16-threads/exercises/tests/exercise.rs`**

```rust
use threads_exercises::{double_in_thread, parallel_sum_of_squares};

// Warm-up: double_in_thread (spawn / move / join)

#[test]
fn warmup_double_positive() {
    assert_eq!(double_in_thread(5), 10);
}

#[test]
fn warmup_double_zero() {
    assert_eq!(double_in_thread(0), 0);
}

#[test]
fn warmup_double_negative() {
    assert_eq!(double_in_thread(-3), -6);
}

#[test]
fn warmup_double_large() {
    assert_eq!(double_in_thread(21), 42);
}

// Main: parallel_sum_of_squares (mpsc fan-out / collect)

#[test]
fn main_sum_empty() {
    assert_eq!(parallel_sum_of_squares(vec![]), 0);
}

#[test]
fn main_sum_one() {
    assert_eq!(parallel_sum_of_squares(vec![3]), 9);
}

#[test]
fn main_sum_many() {
    assert_eq!(parallel_sum_of_squares(vec![1, 2, 3]), 14);
}

#[test]
fn main_sum_negatives() {
    assert_eq!(parallel_sum_of_squares(vec![-2, 4]), 20);
}
```

- [ ] **Step 3: Create `lessons/16-threads/exercises/compile_fails/16-rc-not-send.rs`**

The `compile_fails/` directory does not exist yet — create it. This file is self-contained and std-only. Write this file:

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// A thread might run at any time, on any core, so anything you move into
// it must be safe to send across threads — the `Send` marker trait.
// `Rc<T>` is deliberately NOT `Send`: it counts references with a plain
// (non-atomic) integer, so two threads cloning/dropping it at once would
// corrupt the count. The compiler rejects sending an `Rc` to a thread.
//
// rustc reports E0277: "`Rc<i32>` cannot be sent between threads
// safely", and explains the closure isn't `Send` because it captures an
// `Rc`.
//
// The fix: use `Arc<T>` ("atomic Rc"), the thread-safe reference-counted
// pointer. Its count uses atomic operations, so it IS `Send`. We'll use
// `Arc` properly in Lesson 17.
//
// Hint: change `use std::rc::Rc;` to `use std::sync::Arc;` and `Rc::new`
// to `Arc::new`.

use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(42);
    let handle = thread::spawn(move || {
        println!("value is {}", *data);
    });
    handle.join().unwrap();
}
```

- [ ] **Step 4: Verify exercise tests compile and fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/16-threads/exercises/Cargo.toml
```

Expected: the crate COMPILES, then all 8 tests FAIL with `not yet implemented` panic message. (Compilation succeeding while tests panic is the correct undone state — the signatures are complete; only the bodies are `todo!()`.)

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package threads-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/16-threads
```

Expected: prints `ok: lessons/16-threads/exercises/compile_fails/16-rc-not-send.rs` and exits 0. (The tool printing the rustc E0277 error text is expected — what matters is the final `ok:` line and exit 0.)

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/16-threads
```

Expected: non-zero exit with a `FAIL: file did not compile, but was expected to: lessons/16-threads/...` message. (This is correct — the file ships broken on purpose.)

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package threads-exercises --all-targets -- -D warnings
cargo fmt --check --package threads-exercises
```

Expected: both exit 0. (The `todo!()` bodies with unused `_n`/`_values` params and NO `use` statements lint clean — verified during design.)

- [ ] **Step 9: Commit**

```bash
git add lessons/16-threads/exercises
git commit -m "feat(lesson-16): add exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/16-threads/solutions/src/lib.rs`
- Overwrite: `lessons/16-threads/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/16-threads/solutions/src/lib.rs`**

```rust
//! Lesson 16 — reference solutions.

use std::sync::mpsc;
use std::thread;

#[must_use]
pub fn double_in_thread(n: i32) -> i32 {
    let handle = thread::spawn(move || n * 2);
    handle.join().unwrap()
}

#[must_use]
pub fn parallel_sum_of_squares(values: Vec<i32>) -> i32 {
    let (tx, rx) = mpsc::channel();
    for v in values {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(v * v).unwrap();
        });
    }
    drop(tx);
    rx.iter().sum()
}
```

> Pedagogical notes:
> - `double_in_thread`: the spawn → `move` → `join` trio. `move` transfers `n` into the thread (required to satisfy the closure's `'static` bound even though `i32` is `Copy`); `join()` blocks and returns the closure's value via a `Result`, unwrapped.
> - `parallel_sum_of_squares`: the canonical mpsc fan-out/collect. One thread per value, each with its own cloned sender, sends the value's square. The original `tx` is dropped so `rx.iter()` terminates once every worker's cloned sender has dropped. The sum is commutative, so the result is deterministic regardless of thread order (verified stable across repeated runs).
> - Both functions return `i32`, so each carries `#[must_use]` — this does NOT trip `clippy::double_must_use` (that only fires on already-`#[must_use]` return types like `Result`).
> - `drop(tx)` drops a real `Drop` type (`Sender`), so `clippy::dropping_copy_types` does not fire. No `#[allow]` attributes should be needed. If clippy fires unexpectedly, fix the code rather than adding an allow, and report the deviation.

- [ ] **Step 2: Overwrite `lessons/16-threads/solutions/tests/exercise.rs`**

```rust
use threads_solutions::{double_in_thread, parallel_sum_of_squares};

// Warm-up: double_in_thread (spawn / move / join)

#[test]
fn warmup_double_positive() {
    assert_eq!(double_in_thread(5), 10);
}

#[test]
fn warmup_double_zero() {
    assert_eq!(double_in_thread(0), 0);
}

#[test]
fn warmup_double_negative() {
    assert_eq!(double_in_thread(-3), -6);
}

#[test]
fn warmup_double_large() {
    assert_eq!(double_in_thread(21), 42);
}

// Main: parallel_sum_of_squares (mpsc fan-out / collect)

#[test]
fn main_sum_empty() {
    assert_eq!(parallel_sum_of_squares(vec![]), 0);
}

#[test]
fn main_sum_one() {
    assert_eq!(parallel_sum_of_squares(vec![3]), 9);
}

#[test]
fn main_sum_many() {
    assert_eq!(parallel_sum_of_squares(vec![1, 2, 3]), 14);
}

#[test]
fn main_sum_negatives() {
    assert_eq!(parallel_sum_of_squares(vec![-2, 4]), 20);
}
```

- [ ] **Step 3: Verify solution tests pass (run twice to confirm determinism)**

```bash
cargo test --package threads-solutions
cargo test --package threads-solutions
```

Expected: 8 tests pass, both times. (The concurrency is real, but every assertion checks a commutative sum, so the result never depends on thread order.)

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package threads-solutions --all-targets -- -D warnings
cargo fmt --check --package threads-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires on anything, fix the code (not with an allow) and report it.

- [ ] **Step 5: Commit**

```bash
git add lessons/16-threads/solutions
git commit -m "feat(lesson-16): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/16-threads/README.md`

- [ ] **Step 1: Overwrite `lessons/16-threads/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 16` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/16-threads/README.md
grep -c '^### ' lessons/16-threads/README.md
grep -c '^```' lessons/16-threads/README.md
```

Expected:
- First line: `# Lesson 16 — Threads & channels`
- `grep -c '^### '` returns `9` (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns `14` (7 code blocks × 2 fence lines — the "Message passing vs shared state" self-study subsection and the "Compile-fail" exercise subsection are prose only)

If either count is wrong, the file content is off — re-check it against the content above and fix before committing.

- [ ] **Step 3: Commit**

```bash
git add lessons/16-threads/README.md
git commit -m "docs(lesson-16): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/16-threads/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/16-threads/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Threads & channels` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 16**

```bash
make slides-build
test -f dist/lessons/16-threads/slides/slides.md
test -f dist/lessons/16-threads/slides/index.html
grep -c "16-threads" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "16-threads"` returns at least 1. (The build-index master registry already has lesson 16 registered with slug `threads`, matching this directory, so it renders as a clickable link.)

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/16-threads/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/16-threads/slides/slides.md
git commit -m "feat(lesson-16): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `threads-solutions`), compile-fail `--expect broken` passes for lesson 16.

- [ ] **Step 2: `make verify LESSON=16-threads` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=16-threads || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "16-threads" dist/index.html
```

Expected: `dist/lessons/` contains all sixteen lessons. `grep -c "16-threads"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 16 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/16-threads/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/16-threads/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same `double_in_thread` / `parallel_sum_of_squares` signatures (exercise ships `todo!()` bodies and no `use` statements; solution ships real bodies with the needed `use` statements)
- `cargo test --package threads-solutions` → 8 passing tests (deterministic across repeated runs)
- `cargo test --manifest-path lessons/16-threads/exercises/Cargo.toml` → compiles, 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/16-threads` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/16-threads` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/16-threads/slides/index.html`
- `dist/index.html` lists lesson 16 as a clickable link
- All changes committed and pushed (plain commit messages, no co-author trailer)
- Deployed site returns HTTP 200 for `/` and `/lessons/16-threads/slides/`
