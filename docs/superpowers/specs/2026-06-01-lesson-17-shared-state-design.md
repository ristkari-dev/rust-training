# Lesson 17 — Shared state — design

The second lesson of Phase 4 (Concurrency). The payoff Lesson 16 pointed
to: when message passing isn't enough and you genuinely need *shared
mutable state* across threads, you combine two tools — `Arc`
(thread-safe shared ownership, from L16) and `Mutex` (synchronized
interior mutability). `Arc<Mutex<T>>` is the thread-safe analogue of
Lesson 10's `Rc<RefCell<T>>`. `RwLock` is covered conceptually.

## Audience and prerequisites

- Has completed Lessons 01-16
- Comfortable with threads, `move`, `join`, and `Arc`-is-`Send` (L16);
  `Rc<RefCell<T>>` interior mutability (L10); `Result`/`unwrap` (L14)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Use `Mutex<T>`: call `.lock()`, mutate the value through the returned
   guard, and rely on the guard's `Drop` to release the lock
2. Explain that the `MutexGuard` is an RAII handle — the lock is held
   exactly as long as the guard is in scope
3. Recognize that `Arc<T>` alone gives shared ownership but *immutable*
   access, so shared mutable state needs `Arc<Mutex<T>>`
4. Build the canonical shared-counter: clone an `Arc<Mutex<_>>` into many
   threads, lock-and-mutate in each, join, and read the exact final value
5. Describe `RwLock<T>` as the read/write variant (many readers *or* one
   writer) and when to prefer it

## Scope

In scope: `Mutex<T>` — `.lock()`, the `MutexGuard` and RAII unlock,
`.lock().unwrap()` (lock poisoning, briefly); `Arc<T>` recap as
thread-safe shared ownership and why it's immutable on its own;
`Arc<Mutex<T>>` for shared mutable state across threads, presented as the
thread-safe analogue of `Rc<RefCell<T>>`; the shared-counter pattern
(clone the `Arc` per thread, lock-increment, join, read); `RwLock<T>`
conceptually (read vs write guards, when reads dominate). The exercises
drill single-threaded `Mutex` (warm-up) and the `Arc<Mutex>` concurrent
counter (main).

Out of scope (deferred or skipped): `RwLock` in the exercises (slides/
README only — read/write interleaving makes a clean deterministic
assertion harder than the counter); deadlocks and lock ordering beyond a
mention; `Condvar`; atomics (`AtomicUsize` etc.) beyond a one-line "for a
plain counter you'd reach for an atomic" aside; `parking_lot`; `Mutex`
poisoning recovery (`.into_inner()`/`PoisonError`) beyond using
`.unwrap()`; `try_lock`; deliberately holding a guard across an `.await`
(an async concern — Lesson 18); scoped threads. Shared state is
introduced as *`Arc<Mutex<T>>` for safe shared mutation*; the lock-free
and async-aware variants are out of band.

## Slide arc (10 slides)

1. **Title — Shared state.** Hook: *"Sometimes threads must share the
   same mutable data, not just pass messages. Sharing plus mutation is
   exactly what causes data races — so Rust makes you combine shared
   ownership (`Arc`) with a lock (`Mutex`) to do it safely."*
2. **Recap — why shared state.** Lesson 16 preferred message passing:
   move ownership between threads, nothing shared. But some problems —
   a counter, a cache, an accumulator — genuinely need many threads
   touching one piece of mutable data. That's where locks come in.
3. **`Mutex<T>` — mutual exclusion.**
   ```rust
   use std::sync::Mutex;

   let m = Mutex::new(0);
   {
       let mut guard = m.lock().unwrap();  // acquire the lock
       *guard += 1;                         // mutate through the guard
   }                                        // guard dropped → lock released
   ```
   A `Mutex<T>` wraps data so only one thread can access it at a time.
   `.lock()` blocks until the lock is free and returns a *guard*.
4. **The guard is RAII.** `.lock()` returns a `MutexGuard<T>` that
   derefs to the inner `T`. The lock is held exactly as long as the
   guard lives, and released automatically when it goes out of scope —
   no manual unlock, no forgetting. (`.lock()` returns a `Result`
   because a thread that panics while holding the lock *poisons* it;
   `.unwrap()` is the usual response.)
5. **`Arc<T>` recap — shared but immutable.** From Lesson 16: `Arc<T>`
   is the thread-safe reference-counted pointer — it lets many threads
   *own* the same value. But `Arc` only hands out *shared* (`&T`)
   access:
   ```rust
   let a = Arc::new(0);
   // *a += 1;  // ERROR: cannot assign through an Arc
   ```
   Shared ownership alone can't mutate. You need a lock inside.
6. **`Arc<Mutex<T>>` — shared *and* mutable.**
   ```rust
   use std::sync::{Arc, Mutex};

   let counter = Arc::new(Mutex::new(0));
   let clone = Arc::clone(&counter);   // another owner, same Mutex
   *clone.lock().unwrap() += 1;
   ```
   `Arc` shares ownership across threads; the `Mutex` inside makes the
   data safely mutable. This is the thread-safe analogue of Lesson 10's
   `Rc<RefCell<T>>`.
7. **The counter pattern.**
   ```rust
   let counter = Arc::new(Mutex::new(0usize));
   let mut handles = Vec::new();
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
   The Mutex serializes the increments, so the final count is *exact* no
   matter how the threads interleave — no lost updates.
8. **`RwLock<T>` — readers vs writers.** When reads vastly outnumber
   writes, a `Mutex` (one accessor at a time, even for reads) is
   wasteful. `RwLock<T>` allows *many* concurrent readers **or** one
   writer:
   ```rust
   let lock = RwLock::new(0);
   let r = lock.read().unwrap();    // many readers allowed
   let w = lock.write().unwrap();   // exclusive
   ```
   Use `Arc<RwLock<T>>` for shared read-heavy state.
9. **Putting it together.** Walk through the exercises: `locked_increment`
   locks a `Mutex<i32>`, adds, and returns the new value (warm-up —
   single-threaded lock mechanics); `concurrent_counter` shares an
   `Arc<Mutex<usize>>` across N threads, each incrementing, then joins
   and reads the exact total (main). The compile-fail shares an `Arc<i32>`
   and tries to mutate it — the error that `Arc<Mutex<i32>>` fixes.
10. **Wrap — safe shared mutation.** Five takeaways: `Mutex<T>` gives one
    thread at a time access via a guard; the guard releases the lock on
    drop (RAII); `Arc` shares ownership but is immutable alone;
    `Arc<Mutex<T>>` is shared mutable state (thread-safe
    `Rc<RefCell<T>>`); `RwLock` allows many readers or one writer. Next:
    **Lesson 18 — Async/await with Tokio**.

## Exercise spec

`lessons/17-shared-state/` follows the standard four-part lesson shape:

```
17-shared-state/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/17-arc-no-mutate.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `shared-state-exercises` and
`shared-state-solutions` (the lesson's "bare" name is `shared-state`; the
import idents are `shared_state_exercises` / `shared_state_solutions`).
This matches the build-index master registry slug `shared-state`, so the
landing page links it without any change. No external dependencies — only
`std::sync` and `std::thread`.

### Exercise stub (`exercises/src/lib.rs`)

The stub ships the two signatures with `todo!()` bodies. It keeps
`use std::sync::Mutex;` because the warm-up's signature references
`Mutex` (so the import is *used* — no unused-import lint). The `Arc` and
`thread` imports are not in any signature, so the stub omits them;
students add them when implementing `concurrent_counter`. The crate and
its tests compile; the tests fail at runtime with the `todo!()` panic.

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

### Warm-up: `locked_increment`

Reference solution:

```rust
#[must_use]
pub fn locked_increment(m: &Mutex<i32>, by: i32) -> i32 {
    let mut guard = m.lock().unwrap();
    *guard += by;
    *guard
}
```

Pedagogical packing: the single-threaded `Mutex` lifecycle in three
lines. `.lock().unwrap()` acquires the guard; `*guard += by` mutates the
inner `i32` through the guard's `DerefMut`; the final `*guard` reads the
new value (a `Copy` `i32`); the guard drops at the end of the function,
releasing the lock. Returns `i32`, so `#[must_use]` is appropriate (not a
`Result`, so no `double_must_use`).

Four tests:

```rust
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
```

The `warmup_increment_twice` test asserts *both* call results (rather
than discarding the first) — discarding a `#[must_use]` return would
trip `unused_must_use`, and asserting both also demonstrates that the
mutation persists in the shared `Mutex` across calls.

### Main: `concurrent_counter`

Reference solution:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

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

Pedagogical packing: the canonical `Arc<Mutex>` shared counter. Each of
`threads` worker threads gets its own `Arc::clone` of the *same* counter,
locks it, and increments `per_thread` times. After joining every handle,
the final `*counter.lock().unwrap()` reads the total. The result is
*exactly* `threads * per_thread` no matter how the threads interleave,
because the `Mutex` serializes every increment — there are no lost
updates. (Returning `*counter.lock().unwrap()` directly avoids a
`let`-and-return; the temporary guard drops after the `usize` is copied
out.) Reuses threads/`join` from L16.

Four tests:

```rust
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

**Eight tests total** (four warm-up + four main). `main_counter_many`
(8 threads × 1000 increments = 8000) is the real contention test:
without the `Mutex` it would lose updates under a data race; with it, the
result is exactly 8000 on every run (verified stable across repeated runs
during design). All assertions are exact equalities that hold regardless
of thread order, so the tests are deterministic.

### Compile-fail: `17-arc-no-mutate.rs`

Path: `exercises/compile_fails/17-arc-no-mutate.rs`. A self-contained
file that shares an `Arc<i32>` across threads and tries to mutate the
inner value directly. Ships broken; the student wraps the value in a
`Mutex` (i.e. `Arc<Mutex<i32>>`) and locks it.

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

Pass condition: the student switches `counter` to `Arc<Mutex<i32>>`
(`use std::sync::{Arc, Mutex};`, `Arc::new(Mutex::new(0))`) and mutates
via `*counter.lock().unwrap() += 1`. rustc reports E0594 "cannot assign
to data in an `Arc`" — verified during design. After the fix the file
compiles.

This is the lesson's centerpiece: it isolates *why* `Arc<Mutex<T>>`
exists — `Arc` alone shares but can't mutate; the `Mutex` supplies the
safe mutation.

## README structure

`lessons/17-shared-state/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - `Mutex<T>` and the guard
  - `Arc<T>` — shared but immutable
  - `Arc<Mutex<T>>` — shared mutable state
  - The counter pattern
  - `RwLock<T>` — readers vs writers
- **Exercises** — four subsections: Warm-up (`locked_increment`), Main
  (`concurrent_counter`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`Arc<Mutex<T>>`" and "The counter pattern" sections are the heaviest —
they carry the lesson's core.

## Lint expectations

Lesson 17's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design:

- Both functions return a plain integer (`i32`/`usize`), so each carries
  `#[must_use]` (not a `Result`, so no `double_must_use`; and omitting it
  would trip `must_use_candidate`).
- Because `locked_increment` is `#[must_use]`, the `warmup_increment_twice`
  test asserts both results rather than discarding the first (a discard
  would trip `unused_must_use`).
- `concurrent_counter` returns `*counter.lock().unwrap()` directly (no
  `let`-and-return, which would trip `clippy::let_and_return`).
- `Arc::clone(&counter)` is the explicit, idiomatic clone form.
- The **exercise stub** keeps `use std::sync::Mutex;` (used by the
  warm-up signature) and omits `Arc`/`thread` (not referenced until the
  bodies are implemented); it lints clean (verified).

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/17-shared-state/` exists with the four-part structure
- Cargo manifests use the correct package names (`shared-state-exercises`,
  `shared-state-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `locked_increment` / `concurrent_counter` signatures; the exercise
  ships `todo!()` bodies, the solution ships real bodies
- `cargo test --package shared-state-solutions` → 8 tests pass
  (deterministic across repeated runs)
- `cargo test --manifest-path lessons/17-shared-state/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/17-shared-state`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/17-shared-state`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/17-shared-state/slides/index.html`
- `dist/index.html` lists lesson 17 as a clickable link (registry slug
  `shared-state` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/17-shared-state/slides/`
  returns 200

## Open questions

None.
