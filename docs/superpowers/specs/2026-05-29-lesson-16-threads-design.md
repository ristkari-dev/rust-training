# Lesson 16 — Threads & channels — design

The first lesson of Phase 4 (Concurrency). Rust's "fearless
concurrency": spawn OS threads, move owned data into them, and
communicate by **message passing** (channels) rather than shared mutable
state. `Send`/`Sync` are the marker traits the compiler uses to make
this safe at compile time. Shared state via `Mutex`/`Arc` is the *next*
lesson — this one deliberately stays on message passing.

## Audience and prerequisites

- Has completed Lessons 01-15
- Comfortable with closures (used since L11), ownership/moves (L07),
  `Rc` (L10), iterators (L11), and `Result`/`unwrap` (L14)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Spawn a thread with `thread::spawn` and retrieve its result by
   calling `.join()` on the returned `JoinHandle`
2. Explain why a thread closure needs `move` — the thread may outlive
   the spawning function, so it must own its captures
3. Describe `Send` (safe to transfer ownership to another thread) and
   `Sync` (safe to share by reference), and that the compiler enforces
   them — most types are `Send`, but `Rc` is not (use `Arc`)
4. Use an `mpsc` channel to pass values between threads: `send` from
   workers, `recv`/iterate on the consumer
5. Collect results from many worker threads by cloning the sender into
   each and summing what arrives — message passing instead of shared
   mutable state

## Scope

In scope: `thread::spawn` and `JoinHandle::join`; the `move` closure and
why it's required; `Send`/`Sync` as marker traits the compiler checks
(conceptual, with `Rc`-isn't-`Send` as the concrete example);
`std::sync::mpsc` channels — `channel()`, `Sender::send`,
`Receiver::recv`/`iter`, cloning the sender for multiple producers, and
the rule that the receiver's iterator ends once all senders are dropped;
the message-passing philosophy ("share memory by communicating"). The
exercises drill spawn/join (warm-up) and an mpsc fan-out/collect
(main).

Out of scope (deferred or skipped): `Mutex`, `RwLock`, `Arc<Mutex<T>>`,
and shared mutable state generally (Lesson 17 — the very next lesson);
`Arc` beyond a one-line "the thread-safe `Rc`, use it here" pointer (it
gets real use in L17); scoped threads (`thread::scope`) beyond a
mention; `async`/`await` and Tokio (Lesson 18); thread pools and `rayon`;
`sync_channel`/bounded channels; atomics; deadlocks; `panic`
propagation across `join` beyond using `.unwrap()`. Concurrency is
introduced as *threads + message passing*; shared-state synchronization
is the next lesson, and async is the one after.

## Slide arc (10 slides)

1. **Title — Threads & channels.** Hook: *"Rust lets you run code on
   many threads without data races — the compiler rejects unsafe sharing
   before the program ever runs. The simplest safe pattern is to spawn
   threads and pass messages between them."*
2. **Phase 4 — fearless concurrency.** Concurrency means doing several
   things at once. The danger is *data races* (two threads touching the
   same memory, one writing). Rust's ownership system plus two marker
   traits (`Send`/`Sync`) catch races at *compile* time — "fearless
   concurrency".
3. **Spawning a thread.**
   ```rust
   use std::thread;

   let handle = thread::spawn(|| {
       println!("hello from a thread");
   });
   ```
   `thread::spawn` takes a closure and runs it on a new OS thread. It
   returns a `JoinHandle` — your handle to that thread.
4. **`move` and `join`.**
   ```rust
   let n = 21;
   let handle = thread::spawn(move || n * 2);  // move: the thread owns n
   let result = handle.join().unwrap();         // wait, get the value
   ```
   The thread may outlive the function that spawned it, so its closure
   must **own** what it captures — hence `move`. `join()` blocks until
   the thread finishes and returns whatever the closure returned (as a
   `Result`).
5. **`Send` & `Sync`.** Two marker traits the compiler checks
   automatically: a type is **`Send`** if it's safe to *move* to another
   thread, and **`Sync`** if it's safe to *share* (`&T`) across threads.
   Almost every type is `Send`. A key exception: `Rc<T>` is **not**
   `Send` (its non-atomic count would race) — its thread-safe sibling
   `Arc<T>` is. Try to send an `Rc` to a thread and the code won't
   compile.
6. **Channels — `mpsc`.**
   ```rust
   use std::sync::mpsc;

   let (tx, rx) = mpsc::channel();
   thread::spawn(move || {
       tx.send(42).unwrap();   // producer sends
   });
   let value = rx.recv().unwrap();  // consumer receives
   ```
   `mpsc` = *multiple producer, single consumer*. The sender (`tx`) and
   receiver (`rx`) are two ends of a queue. `send` hands a value to the
   other end; `recv` waits for one.
7. **Multiple producers.**
   ```rust
   for v in values {
       let tx = tx.clone();                 // each thread gets its own sender
       thread::spawn(move || tx.send(v * v).unwrap());
   }
   drop(tx);                                 // drop the original
   let total: i32 = rx.iter().sum();         // ends when all senders are gone
   ```
   Clone the sender to give each worker its own. `rx.iter()` yields
   values until *every* sender has been dropped — so you drop the
   original `tx` and let the workers' clones drop as they finish.
8. **Message passing vs shared state.** A Go-borrowed slogan Rust likes:
   *"Don't communicate by sharing memory; share memory by
   communicating."* Channels move ownership of data between threads, so
   there's nothing shared to race over. When you *do* need shared mutable
   state, you reach for `Mutex`/`Arc` — that's **Lesson 17**.
9. **Putting it together.** Walk through the exercises:
   `double_in_thread` spawns a thread, moves `n` in, and joins for the
   result (warm-up); `parallel_sum_of_squares` spawns one thread per
   value, each sending its square down an `mpsc` channel, and sums what
   arrives (main). The compile-fail moves an `Rc` into a thread and gets
   the `Send` error.
10. **Wrap — Phase 4 begins.** Five takeaways: `thread::spawn` runs a
    closure on a new thread; `move` gives it ownership, `join` returns
    its result; `Send`/`Sync` let the compiler reject data races (`Rc`
    isn't `Send`, `Arc` is); `mpsc` channels pass messages between
    threads; prefer message passing over shared memory. Next:
    **Lesson 17 — Shared state** (`Mutex`, `RwLock`, `Arc<Mutex<T>>`).

## Exercise spec

`lessons/16-threads/` follows the standard four-part lesson shape:

```
16-threads/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/16-rc-not-send.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `threads-exercises` and `threads-solutions` (the
lesson's "bare" name is `threads`; the import idents are
`threads_exercises` / `threads_solutions`). This matches the build-index
master registry slug `threads`, so the landing page links it without any
change. No external dependencies — only `std::thread` and
`std::sync::mpsc`.

### Exercise stub (`exercises/src/lib.rs`)

The stub ships the two function signatures with `todo!()` bodies and
**no `use` statements**. The `use std::thread;` / `use std::sync::mpsc;`
imports are deliberately omitted: with `todo!()` bodies they would be
unused, and the workspace denies `unused`. Students add the imports as
they implement — which is itself part of learning the APIs. The crate
and its tests still compile (the signatures are complete); the tests
fail at runtime with the `todo!()` panic, like every prior lesson.

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

### Warm-up: `double_in_thread`

Reference solution (with the imports the solution file carries):

```rust
use std::thread;

#[must_use]
pub fn double_in_thread(n: i32) -> i32 {
    let handle = thread::spawn(move || n * 2);
    handle.join().unwrap()
}
```

Pedagogical packing: the spawn → `move` → `join` trio in three lines.
`move` transfers `n` (a `Copy` `i32`, but `move` is still required to
satisfy the `'static` bound on the closure) into the thread; `join()`
blocks and returns the closure's value via a `Result`, unwrapped.
Returns `i32`, so `#[must_use]` is appropriate (not a `Result`, so no
`double_must_use`).

Four tests:

```rust
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
```

### Main: `parallel_sum_of_squares`

Reference solution:

```rust
use std::sync::mpsc;
use std::thread;

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

Pedagogical packing: the canonical mpsc fan-out/collect. One thread per
value, each with its own cloned sender, sends the value's square. The
original `tx` is dropped so that `rx.iter()` terminates once all the
workers' sender clones drop (i.e. when every thread has finished). The
sum is **commutative**, so the result is deterministic regardless of
thread-completion order — verified stable across repeated runs during
design. Reuses iterators (L11) via `rx.iter().sum()`.

Four tests (the `vec![]` case yields 0 — no threads spawned, all senders
dropped immediately):

```rust
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

**Eight tests total** (four warm-up + four main). Test arithmetic:
`main_sum_many` → `1 + 4 + 9 = 14`; `main_sum_negatives` →
`4 + 16 = 20`. The tests are deterministic despite the concurrency
because every assertion checks a commutative sum, never an order.

### Compile-fail: `16-rc-not-send.rs`

Path: `exercises/compile_fails/16-rc-not-send.rs`. A self-contained file
that moves an `Rc<i32>` into a spawned thread. Ships broken; the student
switches `Rc` to `Arc` until it compiles.

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

Pass condition: the student replaces `Rc` with `Arc` (`use std::sync::
Arc;` + `Arc::new`). rustc reports E0277 "`Rc<i32>` cannot be sent
between threads safely" — verified during design. After the swap the
file compiles.

This is the lesson's centerpiece for `Send`: the marker trait isn't
abstract bureaucracy — it's the compiler proving, before the program
runs, that you aren't about to corrupt a reference count from two
threads.

## README structure

`lessons/16-threads/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - Spawning threads — `spawn`, `move`, `join`
  - `Send` & `Sync` — the marker traits
  - Channels — `mpsc`
  - Multiple producers — cloning the sender
  - Message passing vs shared state
- **Exercises** — four subsections: Warm-up (`double_in_thread`), Main
  (`parallel_sum_of_squares`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"Channels" and "`Send` & `Sync`" sections are the heaviest — they carry
the lesson's core.

## Lint expectations

Lesson 16's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design:

- Both functions return `i32`, so each carries `#[must_use]` (not a
  `Result`, so no `double_must_use`).
- `double_in_thread` uses `handle.join().unwrap()` — fine under
  pedantic.
- `parallel_sum_of_squares` clones the sender per worker, `drop(tx)`s
  the original (a real `Drop` type — no `dropping_copy_types` lint), and
  `rx.iter().sum()` collects. No lint fires.
- `values: Vec<i32>` by value: `clippy::needless_pass_by_value` is
  *allowed* in the workspace anyway, and the value is consumed by the
  `for` loop regardless.
- The **exercise stub** deliberately omits `use` statements so its
  `todo!()` bodies don't leave unused imports (which the `unused` deny
  group would reject); the stub lints clean (verified).

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/16-threads/` exists with the four-part structure
- Cargo manifests use the correct package names (`threads-exercises`,
  `threads-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `double_in_thread` / `parallel_sum_of_squares` signatures; the exercise
  ships `todo!()` bodies (and no `use` statements), the solution ships
  real bodies (with the needed `use` statements)
- `cargo test --package threads-solutions` → 8 tests pass (deterministic)
- `cargo test --manifest-path lessons/16-threads/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/16-threads`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/16-threads`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/16-threads/slides/index.html`
- `dist/index.html` lists lesson 16 as a clickable link (registry slug
  `threads` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/16-threads/slides/` returns 200

## Open questions

None.
