# Lesson 19 — Memory, layout, and `unsafe` — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the first lesson of Phase 5 of the Rust training course: memory, layout, and `unsafe`. `unsafe` unlocks a few operations the compiler can't verify (chiefly raw-pointer deref); the discipline is keeping it small, sound, and documented. Warm-up: `read_doubled` (a safe fn with a contained, sound `unsafe` deref). Main: `unsafe fn sum_raw(ptr, len)` (an unsafe fn with a `# Safety` contract + pointer arithmetic). Compile-fail: dereferencing a raw pointer outside an `unsafe` block (E0133).

**Architecture:** Use the existing `make new-lesson` scaffolder. All exercise code is SOUND (no undefined behavior). The stub ships both signatures with `todo!()` bodies; the `unsafe fn` keeps its `# Safety` doc even as a stub (clippy's `missing_safety_doc` requires it). All reference code in this plan was empirically verified clippy-pedantic-clean during design — including the specific forms chosen to satisfy `borrow_as_ptr` (use `&raw const n`), `missing_safety_doc`, the 2024 `unsafe_op_in_unsafe_fn` lint, and `useless_vec` (tests use arrays, not `vec!`).

**Tech Stack:** Rust 2024 edition, `std` only (no external dependencies), existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-06-03-lesson-19-unsafe-memory-design.md`](../specs/2026-06-03-lesson-19-unsafe-memory-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

**Commit convention:** Plain commit messages only — no `Co-Authored-By` trailer or any AI attribution. If a commit fails with a GPG/pinentry error, simply retry the same `git commit` command once or twice.

---

## Task 1: Scaffold lessons/19-unsafe-memory

**Files (all created by the scaffolder):**
- `lessons/19-unsafe-memory/README.md` (placeholder, replaced in Task 4)
- `lessons/19-unsafe-memory/slides/index.html` (final — no edit needed)
- `lessons/19-unsafe-memory/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/19-unsafe-memory/exercises/Cargo.toml` (final — no edit needed)
- `lessons/19-unsafe-memory/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/19-unsafe-memory/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/19-unsafe-memory/solutions/Cargo.toml` (final — no edit needed)
- `lessons/19-unsafe-memory/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/19-unsafe-memory/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=19-unsafe-memory
```

Expected: `scaffolded ./lessons/19-unsafe-memory`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/19-unsafe-memory/
ls lessons/19-unsafe-memory/slides/ lessons/19-unsafe-memory/exercises/ lessons/19-unsafe-memory/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/19-unsafe-memory/exercises/Cargo.toml lessons/19-unsafe-memory/solutions/Cargo.toml
```

Expected:
```
lessons/19-unsafe-memory/exercises/Cargo.toml:name = "unsafe-memory-exercises"
lessons/19-unsafe-memory/solutions/Cargo.toml:name = "unsafe-memory-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"unsafe-memory-[^"]*"' | sort -u
```

Expected output:
```
"name":"unsafe-memory-exercises"
"name":"unsafe-memory-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/19-unsafe-memory
git commit -m "chore: scaffold lessons/19-unsafe-memory"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/19-unsafe-memory/exercises/src/lib.rs`
- Overwrite: `lessons/19-unsafe-memory/exercises/tests/exercise.rs`
- Create: `lessons/19-unsafe-memory/exercises/compile_fails/19-deref-outside-unsafe.rs`

- [ ] **Step 1: Overwrite `lessons/19-unsafe-memory/exercises/src/lib.rs`**

The `# Safety` doc on `sum_raw` is REQUIRED even in the stub — clippy's `missing_safety_doc` (a denied pedantic lint) fires on any public `unsafe fn` without it. Write exactly as shown:

```rust
//! Lesson 19 — exercises.
//!
//! Implement `read_doubled` (warm-up) and `sum_raw` (main) so that
//! `cargo test --manifest-path
//! lessons/19-unsafe-memory/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn read_doubled(_n: i32) -> i32 {
    todo!("make a raw `*const i32` to `n` with `&raw const n`, dereference it in an `unsafe` block, and double the value")
}

/// Sum `len` consecutive `i32`s starting at `ptr`.
///
/// # Safety
///
/// `ptr` must be valid for reads of `len` consecutive `i32` values, and
/// that memory must stay valid for the duration of the call.
#[must_use]
pub unsafe fn sum_raw(_ptr: *const i32, _len: usize) -> i32 {
    todo!("sum `len` values starting at `ptr` using pointer arithmetic (`*ptr.add(i)`) in an `unsafe` block")
}
```

- [ ] **Step 2: Overwrite `lessons/19-unsafe-memory/exercises/tests/exercise.rs`**

```rust
use unsafe_memory_exercises::{read_doubled, sum_raw};

// Warm-up: read_doubled (raw pointer deref in a safe fn)

#[test]
fn warmup_basic() {
    assert_eq!(read_doubled(21), 42);
}

#[test]
fn warmup_zero() {
    assert_eq!(read_doubled(0), 0);
}

#[test]
fn warmup_negative() {
    assert_eq!(read_doubled(-5), -10);
}

#[test]
fn warmup_large() {
    assert_eq!(read_doubled(1000), 2000);
}

// Main: sum_raw (unsafe fn + pointer arithmetic)

#[test]
fn main_sum_empty() {
    let v: [i32; 0] = [];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` (0) reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 0);
}

#[test]
fn main_sum_one() {
    let v = [7];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 7);
}

#[test]
fn main_sum_many() {
    let v = [1, 2, 3, 4];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 10);
}

#[test]
fn main_sum_negatives() {
    let v = [-2, 5, 10];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 13);
}
```

- [ ] **Step 3: Create `lessons/19-unsafe-memory/exercises/compile_fails/19-deref-outside-unsafe.rs`**

The `compile_fails/` directory does not exist yet — create it. This file is self-contained and std-only. Write this file:

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Creating a raw pointer is safe — `&raw const n` just makes a `*const
// i32`. But DEREFERENCING a raw pointer is one of the operations the
// compiler can't verify (the pointer might be null, dangling, or
// unaligned), so it is only allowed inside an `unsafe` block. Here the
// deref `*ptr` is in ordinary safe code, so the compiler rejects it.
//
// rustc reports E0133: "dereference of raw pointer is unsafe and
// requires unsafe function or block".
//
// The fix: wrap the dereference in an `unsafe { }` block (and, in real
// code, add a `// SAFETY:` comment explaining why the pointer is valid).
//
// Hint: change `let value = *ptr;` to `let value = unsafe { *ptr };`.

fn main() {
    let n = 42;
    let ptr = &raw const n;
    let value = *ptr;
    println!("{value}");
}
```

- [ ] **Step 4: Verify exercise tests compile and fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/19-unsafe-memory/exercises/Cargo.toml
```

Expected: the crate COMPILES, then all 8 tests FAIL with `not yet implemented` panic message. (Compilation succeeding while tests panic is the correct undone state — the signatures are complete; only the bodies are `todo!()`.)

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package unsafe-memory-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/19-unsafe-memory
```

Expected: prints `ok: lessons/19-unsafe-memory/exercises/compile_fails/19-deref-outside-unsafe.rs` and exits 0. (The tool printing the rustc E0133 error text is expected — what matters is the final `ok:` line and exit 0.)

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/19-unsafe-memory
```

Expected: non-zero exit with a `FAIL: file did not compile, but was expected to: lessons/19-unsafe-memory/...` message. (This is correct — the file ships broken on purpose.)

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package unsafe-memory-exercises --all-targets -- -D warnings
cargo fmt --check --package unsafe-memory-exercises
```

Expected: both exit 0. (The `todo!()` bodies, the `# Safety` doc on the `unsafe fn` stub, and the array-based tests all lint clean — verified during design.)

- [ ] **Step 9: Commit**

```bash
git add lessons/19-unsafe-memory/exercises
git commit -m "feat(lesson-19): add exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/19-unsafe-memory/solutions/src/lib.rs`
- Overwrite: `lessons/19-unsafe-memory/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/19-unsafe-memory/solutions/src/lib.rs`**

Write exactly as shown. Do NOT change `&raw const n` to `&n as *const i32` (clippy `borrow_as_ptr` rejects the cast form); keep the `# Safety` doc and the inner `unsafe {}` block inside `sum_raw`.

```rust
//! Lesson 19 — reference solutions.

#[must_use]
pub fn read_doubled(n: i32) -> i32 {
    let ptr = &raw const n;
    // SAFETY: `ptr` was just created from the live local `n`, so it is
    // non-null, aligned, and points to an initialized `i32`.
    let value = unsafe { *ptr };
    value * 2
}

/// Sum `len` consecutive `i32`s starting at `ptr`.
///
/// # Safety
///
/// `ptr` must be valid for reads of `len` consecutive `i32` values, and
/// that memory must stay valid for the duration of the call.
#[must_use]
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    let mut total = 0;
    for i in 0..len {
        // SAFETY: the caller guarantees `ptr` is valid for `len` reads,
        // so `ptr.add(i)` for `i < len` is in bounds and readable.
        total += unsafe { *ptr.add(i) };
    }
    total
}
```

> Pedagogical notes:
> - `read_doubled` is a SAFE function containing one sound `unsafe` operation. `&raw const n` creates a `*const i32` directly (the modern 2024 raw-borrow operator); creating the pointer is safe, dereferencing it (`*ptr`) needs the `unsafe {}` block, and the `// SAFETY:` comment justifies why it's sound.
> - `sum_raw` is an `unsafe fn` — its soundness depends on a caller-upheld invariant stated in the `# Safety` doc (required by clippy's `missing_safety_doc`). It walks `len` elements with pointer arithmetic (`ptr.add(i)`), dereferencing each inside an `unsafe {}` block (required even inside an `unsafe fn` under the 2024 edition's `unsafe_op_in_unsafe_fn`).
> - The explicit `for i in 0..len` loop is used because the deref can't go in a plain iterator closure; `clippy::needless_range_loop` does not fire (no slice is being indexed).
> - Both functions return `i32`, so each carries `#[must_use]`. Do NOT add `#[allow]` attributes. If clippy fires unexpectedly, fix the code rather than adding an allow, and report the deviation.

- [ ] **Step 2: Overwrite `lessons/19-unsafe-memory/solutions/tests/exercise.rs`**

```rust
use unsafe_memory_solutions::{read_doubled, sum_raw};

// Warm-up: read_doubled (raw pointer deref in a safe fn)

#[test]
fn warmup_basic() {
    assert_eq!(read_doubled(21), 42);
}

#[test]
fn warmup_zero() {
    assert_eq!(read_doubled(0), 0);
}

#[test]
fn warmup_negative() {
    assert_eq!(read_doubled(-5), -10);
}

#[test]
fn warmup_large() {
    assert_eq!(read_doubled(1000), 2000);
}

// Main: sum_raw (unsafe fn + pointer arithmetic)

#[test]
fn main_sum_empty() {
    let v: [i32; 0] = [];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` (0) reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 0);
}

#[test]
fn main_sum_one() {
    let v = [7];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 7);
}

#[test]
fn main_sum_many() {
    let v = [1, 2, 3, 4];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 10);
}

#[test]
fn main_sum_negatives() {
    let v = [-2, 5, 10];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 13);
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package unsafe-memory-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package unsafe-memory-solutions --all-targets -- -D warnings
cargo fmt --check --package unsafe-memory-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. The code above is exactly correct as written — do NOT modify it. If clippy fires on anything, do NOT add an `#[allow]` and do NOT change the code; instead STOP and report the exact clippy output.

- [ ] **Step 5: Commit**

```bash
git add lessons/19-unsafe-memory/solutions
git commit -m "feat(lesson-19): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/19-unsafe-memory/README.md`

- [ ] **Step 1: Overwrite `lessons/19-unsafe-memory/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 19` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
# Lesson 19 — Memory, layout, and `unsafe`

Rust guarantees memory safety — but a few low-level operations can't be
checked by the compiler. `unsafe` is the escape hatch: it unlocks those
operations (chiefly dereferencing a raw pointer) and asks you to uphold
the rules by hand. It does *not* turn off the borrow checker. This lesson
opens Phase 5.

## Learning goals

- Explain that `unsafe` does not disable the borrow checker — it unlocks
  a few operations the compiler can't verify
- Create a raw pointer (`&raw const x`, `*const T`) — safe — and
  dereference it inside an `unsafe {}` block — not safe
- Write a `// SAFETY:` comment justifying why an `unsafe` operation is
  sound
- Define an `unsafe fn` with a `# Safety` doc and call it inside an
  `unsafe {}` block
- Recognize memory-layout tools (`size_of`, `align_of`, `#[repr(C)]`)
  and the rule that unsafe code must never cause undefined behavior

## Self-study notes

### What `unsafe` is (and isn't)

`unsafe` does **not** switch off Rust's safety checks. The borrow
checker, type checker, and lifetimes all still apply. It unlocks a small
set of operations the compiler can't verify — most importantly,
dereferencing a raw pointer and calling an `unsafe fn`:

```rust
let n = 42;
let ptr: *const i32 = &raw const n;
let value = unsafe { *ptr };   // deref needs unsafe
```

Most Rust code never uses `unsafe` at all. It's for the rare low-level
building block.

### Raw pointers

`*const T` and `*mut T` are raw pointers. Unlike references, they can be
null, dangling, or unaligned, and the compiler tracks none of that.
*Creating* one is safe; *dereferencing* one is not:

```rust
let n = 42;
let p = &raw const n;   // *const i32 — safe to make
let q = &raw mut some_mut;  // *mut T from a mutable place
```

`&raw const x` / `&raw mut x` are the modern way to take a raw pointer
without first creating a reference.

### The `unsafe {}` block and `// SAFETY:`

Unsafe operations go inside an `unsafe {}` block, and a `// SAFETY:`
comment records *why* the operation is sound:

```rust
let ptr = &raw const n;
// SAFETY: `ptr` came from the live local `n`, so it is non-null,
// aligned, and points to an initialized `i32`.
let value = unsafe { *ptr };
```

`unsafe` is a promise *you* make to the compiler. The comment is you
showing your work — keep the block small and the justification honest.

### `unsafe fn` and `# Safety` contracts

When a function's correctness depends on invariants the **caller** must
uphold, mark it `unsafe fn` and document them under `# Safety`:

```rust
/// # Safety
/// `ptr` must be valid for reads of `len` consecutive `i32`s.
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    // ...read *ptr.add(i) for i in 0..len...
}
```

Calling it requires an `unsafe {}` block — the caller is accepting the
contract.

### Memory layout — `size_of`, `align_of`, `#[repr(C)]`

Safe tools let you inspect how types are laid out:

```rust
assert_eq!(std::mem::size_of::<i32>(), 4);
assert_eq!(std::mem::align_of::<i32>(), 4);
```

By default Rust may reorder a struct's fields for tighter packing.
`#[repr(C)]` forces a predictable, C-compatible layout — essential when
sharing data with C (Lesson 20). The soundness rule above always holds:
unsafe code must never cause undefined behavior.

## Exercises

### Warm-up: `read_doubled`

Implement `read_doubled(n: i32) -> i32`. Make a raw `*const i32` pointing
at `n` with `&raw const n`, dereference it inside an `unsafe` block, and
return the value doubled:

```rust
pub fn read_doubled(n: i32) -> i32 {
    // let ptr = &raw const n;
    // let value = unsafe { *ptr };
    // value * 2
    todo!()
}
```

Add a `// SAFETY:` comment explaining why the deref is sound (the pointer
comes from the live local `n`).

### Main: `sum_raw`

Implement the `unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32`
(its `# Safety` contract is given). Sum `len` consecutive `i32`s starting
at `ptr`, using pointer arithmetic:

```rust
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    // for i in 0..len { total += unsafe { *ptr.add(i) }; }
    todo!()
}
```

The tests call it as `unsafe { sum_raw(v.as_ptr(), v.len()) }`, upholding
the contract by passing a valid array pointer and its length.

### Compile-fail

`exercises/compile_fails/19-deref-outside-unsafe.rs` dereferences a raw
pointer in ordinary safe code, which the compiler rejects (E0133 — a raw
deref requires an `unsafe` block). Fix it by wrapping the deref in
`unsafe { }`.

### Run

```bash
make verify LESSON=19-unsafe-memory
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/19-unsafe-memory/README.md
grep -c '^### ' lessons/19-unsafe-memory/README.md
grep -c '^```' lessons/19-unsafe-memory/README.md
```

Expected:
- First line: `# Lesson 19 — Memory, layout, and `unsafe``
- `grep -c '^### '` returns `9` (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns `16` (8 code blocks × 2 fence lines — the "Compile-fail" exercise subsection is prose only)

If either count is wrong, the file content is off — re-check it against the content above and fix before committing.

- [ ] **Step 3: Commit**

```bash
git add lessons/19-unsafe-memory/README.md
git commit -m "docs(lesson-19): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/19-unsafe-memory/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/19-unsafe-memory/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Memory, layout, and unsafe` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
# Memory, layout, and `unsafe`

> Rust guarantees memory safety — but a few low-level operations can't be checked by the compiler. `unsafe` is the escape hatch: it unlocks those operations and asks you to uphold the rules by hand.

---

## Phase 5 — safe by default

Everything so far has been *safe* Rust: the compiler proves no use-after-free, no data races, no out-of-bounds access.

`unsafe` doesn't switch that off — it unlocks a small set of extra operations the compiler can't verify, and trusts you to keep them sound.

---

## The unsafe "superpowers"

Inside `unsafe`, you may:

- dereference a raw pointer
- call an `unsafe fn`
- access a mutable `static`
- implement an `unsafe trait`
- access a `union` field

That's *all* `unsafe` adds. The borrow checker and type checker still apply. Most Rust never needs any of this.

---

## Raw pointers

```rust
let n = 42;
let ptr: *const i32 = &raw const n;   // creating a raw pointer is SAFE
// let v = *ptr;                       // dereferencing is NOT
```

`*const T` and `*mut T` can be null, dangling, or unaligned — the compiler tracks none of that. Making one is safe; *using* one is where the danger is.

---

## The `unsafe {}` block

```rust
let ptr = &raw const n;
// SAFETY: `ptr` came from the live local `n`, so it is non-null,
// aligned, and points to an initialized `i32`.
let value = unsafe { *ptr };
```

The dereference goes inside an `unsafe {}` block. The `// SAFETY:` comment records *why* it's sound — `unsafe` is a promise you make, and the comment is you showing your work.

---

## `unsafe fn` & safety contracts

```rust
/// # Safety
/// `ptr` must be valid for reads of `len` consecutive `i32`s.
unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 { /* ... */ }

let v = [1, 2, 3];
let total = unsafe { sum_raw(v.as_ptr(), v.len()) };
```

When a function's correctness depends on invariants the *caller* must uphold, mark it `unsafe fn` and document them under `# Safety`. Calling it requires an `unsafe {}` block.

---

## Memory layout

```rust
assert_eq!(std::mem::size_of::<i32>(), 4);
assert_eq!(std::mem::align_of::<i32>(), 4);
```

By default Rust may reorder a struct's fields for packing. `#[repr(C)]` forces a predictable, C-compatible layout — essential when you share data with C (Lesson 20).

---

## Soundness — the one rule

Unsafe code must **never** cause undefined behavior (UB) — no matter how it's called.

- keep `unsafe` blocks tiny
- wrap them in safe APIs that uphold the invariants
- write a `// SAFETY:` for every one

The `miri` interpreter (Lesson 27) can catch many UB bugs in tests.

---

## Putting it together

Today's exercises:

- **Warm-up** `read_doubled` — make a raw pointer with `&raw const`, deref it in an `unsafe` block, double the value
- **Main** `sum_raw` — an `unsafe fn` that sums `len` values via `*ptr.add(i)` under a `# Safety` contract

The compile-fail dereferences a raw pointer with no `unsafe` block.

---

## Wrap — Phase 5 begins

- `unsafe` unlocks a few operations, not a borrow-checker bypass
- creating a raw pointer is safe; dereferencing it isn't
- `// SAFETY:` documents why each use is sound
- an `unsafe fn` states a `# Safety` contract the caller upholds
- unsafe code must never cause UB

Next: **Lesson 20 — FFI** (calling C from Rust and back).
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 19**

```bash
make slides-build
test -f dist/lessons/19-unsafe-memory/slides/slides.md
test -f dist/lessons/19-unsafe-memory/slides/index.html
grep -c "19-unsafe-memory" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "19-unsafe-memory"` returns at least 1. (The build-index master registry already has lesson 19 registered with slug `unsafe-memory`, matching this directory, so it renders as a clickable link.)

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/19-unsafe-memory/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/19-unsafe-memory/slides/slides.md
git commit -m "feat(lesson-19): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `unsafe-memory-solutions`), compile-fail `--expect broken` passes for lesson 19.

- [ ] **Step 2: `make verify LESSON=19-unsafe-memory` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=19-unsafe-memory || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "19-unsafe-memory" dist/index.html
```

Expected: `dist/lessons/` contains all nineteen lessons. `grep -c "19-unsafe-memory"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 19 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/19-unsafe-memory/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/19-unsafe-memory/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same `read_doubled` / `sum_raw` signatures (including the `# Safety` doc on `sum_raw`); the exercise ships `todo!()` bodies, the solution ships real bodies
- `cargo test --package unsafe-memory-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/19-unsafe-memory/exercises/Cargo.toml` → compiles, 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/19-unsafe-memory` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/19-unsafe-memory` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/19-unsafe-memory/slides/index.html`
- `dist/index.html` lists lesson 19 as a clickable link
- All changes committed and pushed (plain commit messages, no co-author trailer)
- Deployed site returns HTTP 200 for `/` and `/lessons/19-unsafe-memory/slides/`
