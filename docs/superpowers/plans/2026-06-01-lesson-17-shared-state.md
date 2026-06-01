# Lesson 17 — Shared state — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the second lesson of Phase 4 of the Rust training course: shared state. Shared mutable state across threads via `Arc<Mutex<T>>`. Warm-up: `locked_increment` (single-threaded Mutex lock/mutate). Main: `concurrent_counter` (the canonical Arc<Mutex> counter, deterministic exact total). Compile-fail: mutating through a bare `Arc<i32>` (E0594), fixed with `Arc<Mutex<i32>>`.

**Architecture:** Use the existing `make new-lesson` scaffolder. The exercise stub ships the two signatures with `todo!()` bodies; it keeps `use std::sync::Mutex;` (the warm-up signature references it) and omits `Arc`/`thread` (students add those for the main). Tests assert exact equalities that hold regardless of thread order, so they are deterministic. All reference code in this plan was empirically verified clippy-pedantic-clean and deterministic across repeated runs during design.

**Tech Stack:** Rust 2024 edition, `std::sync` (`Mutex`, `Arc`) + `std::thread` (no external dependencies), existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-06-01-lesson-17-shared-state-design.md`](../specs/2026-06-01-lesson-17-shared-state-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

**Commit convention:** Plain commit messages only — no `Co-Authored-By` trailer or any AI attribution. If a commit fails with a GPG/pinentry error, simply retry the same `git commit` command once or twice.

---

## Task 1: Scaffold lessons/17-shared-state

**Files (all created by the scaffolder):**
- `lessons/17-shared-state/README.md` (placeholder, replaced in Task 4)
- `lessons/17-shared-state/slides/index.html` (final — no edit needed)
- `lessons/17-shared-state/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/17-shared-state/exercises/Cargo.toml` (final — no edit needed)
- `lessons/17-shared-state/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/17-shared-state/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/17-shared-state/solutions/Cargo.toml` (final — no edit needed)
- `lessons/17-shared-state/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/17-shared-state/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=17-shared-state
```

Expected: `scaffolded ./lessons/17-shared-state`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/17-shared-state/
ls lessons/17-shared-state/slides/ lessons/17-shared-state/exercises/ lessons/17-shared-state/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/17-shared-state/exercises/Cargo.toml lessons/17-shared-state/solutions/Cargo.toml
```

Expected:
```
lessons/17-shared-state/exercises/Cargo.toml:name = "shared-state-exercises"
lessons/17-shared-state/solutions/Cargo.toml:name = "shared-state-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"shared-state-[^"]*"' | sort -u
```

Expected output:
```
"name":"shared-state-exercises"
"name":"shared-state-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/17-shared-state
git commit -m "chore: scaffold lessons/17-shared-state"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/17-shared-state/exercises/src/lib.rs`
- Overwrite: `lessons/17-shared-state/exercises/tests/exercise.rs`
- Create: `lessons/17-shared-state/exercises/compile_fails/17-arc-no-mutate.rs`

- [ ] **Step 1: Overwrite `lessons/17-shared-state/exercises/src/lib.rs`**

Note: this stub keeps `use std::sync::Mutex;` (the warm-up signature references `Mutex`, so the import is used — no unused-import lint) but omits `Arc`/`thread` (not referenced until the bodies are implemented). Do NOT add other imports.

```rust
//! Lesson 17 — exercises.
//!
//! Implement `locked_increment` (warm-up) and `concurrent_counter`
//! (main) so that `cargo test --manifest-path
//! lessons/17-shared-state/exercises/Cargo.toml` passes. You'll need to
//! add `use` statements (e.g. `std::sync::Arc`, `std::thread`) for the
//! main exercise. The tests live in `tests/exercise.rs`.

use std::sync::Mutex;

#[must_use]
pub fn locked_increment(_m: &Mutex<i32>, _by: i32) -> i32 {
    todo!("lock the mutex, add `by` to the value, and return the new value")
}

#[must_use]
pub fn concurrent_counter(_threads: usize, _per_thread: usize) -> usize {
    todo!("share an Arc<Mutex<usize>> across `threads` threads; each increments `per_thread` times; join all and return the final count")
}
```

- [ ] **Step 2: Overwrite `lessons/17-shared-state/exercises/tests/exercise.rs`**

```rust
use shared_state_exercises::{concurrent_counter, locked_increment};
use std::sync::Mutex;

// Warm-up: locked_increment (single-threaded Mutex)

#[test]
fn warmup_increment_basic() {
    let m = Mutex::new(10);
    assert_eq!(locked_increment(&m, 5), 15);
}

#[test]
fn warmup_increment_zero() {
    let m = Mutex::new(0);
    assert_eq!(locked_increment(&m, 0), 0);
}

#[test]
fn warmup_increment_negative() {
    let m = Mutex::new(3);
    assert_eq!(locked_increment(&m, -8), -5);
}

#[test]
fn warmup_increment_twice() {
    let m = Mutex::new(0);
    assert_eq!(locked_increment(&m, 2), 2);
    assert_eq!(locked_increment(&m, 3), 5);
}

// Main: concurrent_counter (Arc<Mutex> shared counter)

#[test]
fn main_counter_zero_threads() {
    assert_eq!(concurrent_counter(0, 100), 0);
}

#[test]
fn main_counter_single_thread() {
    assert_eq!(concurrent_counter(1, 50), 50);
}

#[test]
fn main_counter_many() {
    assert_eq!(concurrent_counter(8, 1000), 8000);
}

#[test]
fn main_counter_no_increments() {
    assert_eq!(concurrent_counter(10, 0), 0);
}
```

- [ ] **Step 3: Create `lessons/17-shared-state/exercises/compile_fails/17-arc-no-mutate.rs`**

The `compile_fails/` directory does not exist yet — create it. This file is self-contained and std-only. Write this file:

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// `Arc<T>` gives you SHARED ownership — many handles to the same value —
// but only shared (&T) access. You cannot get a `&mut` through an `Arc`,
// because other threads might be reading the value at the same time.
// So `*counter += 1` is rejected: there's no way to mutate the `i32`
// behind an `Arc<i32>`.
//
// rustc reports E0594: "cannot assign to data in an `Arc<i32>`".
//
// The fix: put a `Mutex` inside the `Arc`. `Arc<Mutex<i32>>` shares
// ownership (the `Arc`) AND allows safe mutation (the `Mutex`): lock the
// mutex to get exclusive access, then mutate through the guard.
//
// Hint: make `counter` an `Arc<Mutex<i32>>` (`Arc::new(Mutex::new(0))`)
// and change `*counter += 1;` to `*counter.lock().unwrap() += 1;`.

use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(0);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            *counter += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("final: {}", *counter);
}
```

- [ ] **Step 4: Verify exercise tests compile and fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/17-shared-state/exercises/Cargo.toml
```

Expected: the crate COMPILES, then all 8 tests FAIL with `not yet implemented` panic message. (Compilation succeeding while tests panic is the correct undone state — the signatures are complete; only the bodies are `todo!()`.)

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package shared-state-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/17-shared-state
```

Expected: prints `ok: lessons/17-shared-state/exercises/compile_fails/17-arc-no-mutate.rs` and exits 0. (The tool printing the rustc E0594 error text is expected — what matters is the final `ok:` line and exit 0.)

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/17-shared-state
```

Expected: non-zero exit with a `FAIL: file did not compile, but was expected to: lessons/17-shared-state/...` message. (This is correct — the file ships broken on purpose.)

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package shared-state-exercises --all-targets -- -D warnings
cargo fmt --check --package shared-state-exercises
```

Expected: both exit 0. (The `todo!()` bodies, the kept `Mutex` import used by the warm-up signature, and the tests all lint clean — verified during design.)

- [ ] **Step 9: Commit**

```bash
git add lessons/17-shared-state/exercises
git commit -m "feat(lesson-17): add exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/17-shared-state/solutions/src/lib.rs`
- Overwrite: `lessons/17-shared-state/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/17-shared-state/solutions/src/lib.rs`**

```rust
//! Lesson 17 — reference solutions.

use std::sync::{Arc, Mutex};
use std::thread;

#[must_use]
pub fn locked_increment(m: &Mutex<i32>, by: i32) -> i32 {
    let mut guard = m.lock().unwrap();
    *guard += by;
    *guard
}

#[must_use]
pub fn concurrent_counter(threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();
    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..per_thread {
                let mut count = counter.lock().unwrap();
                *count += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    *counter.lock().unwrap()
}
```

> Pedagogical notes:
> - `locked_increment`: `.lock().unwrap()` acquires the guard; `*guard += by` mutates the inner `i32` through `DerefMut`; the final `*guard` reads the new value (a `Copy` `i32`); the guard drops at the end of the function, releasing the lock.
> - `concurrent_counter`: each of `threads` worker threads gets its own `Arc::clone` of the SAME counter, locks it, and increments `per_thread` times. After joining every handle, `*counter.lock().unwrap()` reads the total. The result is exactly `threads * per_thread` regardless of interleaving — the `Mutex` serializes increments, so no updates are lost (verified stable across repeated runs).
> - Both functions return a plain integer, so each carries `#[must_use]` — not a `Result`, so no `double_must_use`; and omitting it would trip `clippy::must_use_candidate`.
> - `concurrent_counter` returns `*counter.lock().unwrap()` directly (NOT `let total = ...; total`) to avoid `clippy::let_and_return`. `Arc::clone(&counter)` is the explicit idiomatic clone. Do NOT add `#[allow]` attributes. If clippy fires unexpectedly, fix the code rather than adding an allow, and report the deviation.

- [ ] **Step 2: Overwrite `lessons/17-shared-state/solutions/tests/exercise.rs`**

```rust
use shared_state_solutions::{concurrent_counter, locked_increment};
use std::sync::Mutex;

// Warm-up: locked_increment (single-threaded Mutex)

#[test]
fn warmup_increment_basic() {
    let m = Mutex::new(10);
    assert_eq!(locked_increment(&m, 5), 15);
}

#[test]
fn warmup_increment_zero() {
    let m = Mutex::new(0);
    assert_eq!(locked_increment(&m, 0), 0);
}

#[test]
fn warmup_increment_negative() {
    let m = Mutex::new(3);
    assert_eq!(locked_increment(&m, -8), -5);
}

#[test]
fn warmup_increment_twice() {
    let m = Mutex::new(0);
    assert_eq!(locked_increment(&m, 2), 2);
    assert_eq!(locked_increment(&m, 3), 5);
}

// Main: concurrent_counter (Arc<Mutex> shared counter)

#[test]
fn main_counter_zero_threads() {
    assert_eq!(concurrent_counter(0, 100), 0);
}

#[test]
fn main_counter_single_thread() {
    assert_eq!(concurrent_counter(1, 50), 50);
}

#[test]
fn main_counter_many() {
    assert_eq!(concurrent_counter(8, 1000), 8000);
}

#[test]
fn main_counter_no_increments() {
    assert_eq!(concurrent_counter(10, 0), 0);
}
```

- [ ] **Step 3: Verify solution tests pass (run TWICE to confirm determinism)**

```bash
cargo test --package shared-state-solutions
cargo test --package shared-state-solutions
```

Expected: 8 tests pass, BOTH times. (The concurrency is real, but every assertion is an exact equality that holds regardless of thread order — the Mutex serializes the increments.)

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package shared-state-solutions --all-targets -- -D warnings
cargo fmt --check --package shared-state-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. The code above is exactly correct as written — do NOT modify it (in particular, keep the direct `*counter.lock().unwrap()` return and the per-thread `let counter = Arc::clone(&counter);`). If clippy fires on anything, do NOT add an `#[allow]` and do NOT change the code; instead STOP and report the exact clippy output.

- [ ] **Step 5: Commit**

```bash
git add lessons/17-shared-state/solutions
git commit -m "feat(lesson-17): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/17-shared-state/README.md`

- [ ] **Step 1: Overwrite `lessons/17-shared-state/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 17` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/17-shared-state/README.md
grep -c '^### ' lessons/17-shared-state/README.md
grep -c '^```' lessons/17-shared-state/README.md
```

Expected:
- First line: `# Lesson 17 — Shared state`
- `grep -c '^### '` returns `9` (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns `16` (8 code blocks × 2 fence lines — the "Compile-fail" exercise subsection is prose only)

If either count is wrong, the file content is off — re-check it against the content above and fix before committing.

- [ ] **Step 3: Commit**

```bash
git add lessons/17-shared-state/README.md
git commit -m "docs(lesson-17): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/17-shared-state/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/17-shared-state/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Shared state` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 17**

```bash
make slides-build
test -f dist/lessons/17-shared-state/slides/slides.md
test -f dist/lessons/17-shared-state/slides/index.html
grep -c "17-shared-state" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "17-shared-state"` returns at least 1. (The build-index master registry already has lesson 17 registered with slug `shared-state`, matching this directory, so it renders as a clickable link.)

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/17-shared-state/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/17-shared-state/slides/slides.md
git commit -m "feat(lesson-17): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `shared-state-solutions`), compile-fail `--expect broken` passes for lesson 17.

- [ ] **Step 2: `make verify LESSON=17-shared-state` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=17-shared-state || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "17-shared-state" dist/index.html
```

Expected: `dist/lessons/` contains all seventeen lessons. `grep -c "17-shared-state"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 17 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/17-shared-state/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/17-shared-state/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same `locked_increment` / `concurrent_counter` signatures (exercise ships `todo!()` bodies and keeps only the `Mutex` import; solution ships real bodies with `Arc`/`Mutex`/`thread`)
- `cargo test --package shared-state-solutions` → 8 passing tests (deterministic across repeated runs)
- `cargo test --manifest-path lessons/17-shared-state/exercises/Cargo.toml` → compiles, 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/17-shared-state` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/17-shared-state` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/17-shared-state/slides/index.html`
- `dist/index.html` lists lesson 17 as a clickable link
- All changes committed and pushed (plain commit messages, no co-author trailer)
- Deployed site returns HTTP 200 for `/` and `/lessons/17-shared-state/slides/`
