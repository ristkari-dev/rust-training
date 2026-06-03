# Lesson 19 — Memory, layout, and `unsafe` — design

The first lesson of Phase 5 (Systems programming). Rust is safe by
default; `unsafe` is the escape hatch for the handful of operations the
compiler can't verify. It does *not* turn off the borrow checker — it
unlocks specific "superpowers", chiefly dereferencing a raw pointer and
calling an `unsafe fn`. The discipline is keeping unsafe small, sound,
and documented (`// SAFETY:` / `# Safety`). `#[repr]` and layout
(`size_of`/`align_of`) are covered conceptually, pointing toward
Lesson 20 (FFI). This is the first lesson whose subject is deliberately
escaping the safety guarantees — so all exercise code is sound (no UB).

## Audience and prerequisites

- Has completed Lessons 01-18
- Comfortable with references/borrowing (L08-09), slices (L04), `Vec`
  (L11), and methods
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Explain that `unsafe` does not disable the borrow checker — it unlocks
   a few extra operations the compiler can't verify (e.g. dereferencing
   a raw pointer)
2. Create a raw pointer (`&raw const x` / `*const T`) — which is safe —
   and dereference it inside an `unsafe {}` block — which is not
3. Write a `// SAFETY:` comment justifying why an `unsafe` operation is
   sound
4. Define an `unsafe fn` with a `# Safety` doc section stating the
   invariants the caller must uphold, and call it inside an `unsafe {}`
   block
5. Recognize memory-layout tools (`size_of`, `align_of`, `#[repr(C)]`)
   and the soundness rule: unsafe code must never cause undefined
   behavior

## Scope

In scope: what `unsafe` is and is not (it unlocks a few operations, not a
license to break the borrow checker); the unsafe "superpowers"
(emphasis on raw-pointer deref and calling `unsafe fn`); raw pointers
(`*const T`/`*mut T`, creating with `&raw const`/`&raw mut` is safe,
dereferencing is unsafe); the `unsafe {}` block and the `// SAFETY:`
convention; `unsafe fn` with a `# Safety` contract and calling it;
pointer arithmetic (`.add`); memory layout conceptually (`size_of`,
`align_of`, `#[repr(C)]` vs default). The exercises drill a contained
unsafe deref in a safe fn (warm-up) and an `unsafe fn` with a safety
contract + pointer arithmetic (main).

Out of scope (deferred or skipped): `transmute` and byte-reinterpretation
(too easy to make unsound for a teaching exercise); `union`s; mutable
statics; implementing `unsafe trait`s (e.g. `Send`/`Sync`); `MaybeUninit`;
`Box::into_raw`/`from_raw` and manual allocation; `slice::from_raw_parts`
beyond the spirit of the main exercise; `NonNull`; the full UB list and
the aliasing/provenance model (mentioned as "never cause UB", not
detailed); miri (Lesson 27); FFI / `extern` (Lesson 20). Layout is
*described* (`#[repr(C)]`, `size_of`) but not exercised — the hands-on
part is raw pointers + `unsafe`.

## Slide arc (10 slides)

1. **Title — Memory, layout, and `unsafe`.** Hook: *"Rust guarantees
   memory safety — but a few low-level operations can't be checked by the
   compiler. `unsafe` is the escape hatch: it unlocks those operations
   and asks you to uphold the rules by hand."*
2. **Phase 5 — safe by default.** Everything so far has been *safe* Rust:
   the compiler proves no use-after-free, no data races, no
   out-of-bounds. `unsafe` doesn't switch that off — it unlocks a small
   set of extra operations the compiler can't verify, and trusts you to
   keep them sound.
3. **The unsafe "superpowers".** Inside `unsafe`, you may: dereference a
   raw pointer; call an `unsafe fn`; access a mutable `static`; implement
   an `unsafe trait`; access a `union` field. That's *all* `unsafe` adds.
   The borrow checker, type checker, and everything else still apply.
   Most Rust never needs any of this.
4. **Raw pointers.**
   ```rust
   let n = 42;
   let ptr: *const i32 = &raw const n;   // creating a raw pointer is SAFE
   // let v = *ptr;                       // dereferencing is NOT
   ```
   `*const T` and `*mut T` are raw pointers. Unlike references, they can
   be null, dangling, or unaligned, and the compiler tracks none of that.
   Making one is safe; *using* one is where the danger is.
5. **The `unsafe {}` block.**
   ```rust
   let ptr = &raw const n;
   // SAFETY: `ptr` came from the live local `n`, so it is non-null,
   // aligned, and points to an initialized `i32`.
   let value = unsafe { *ptr };
   ```
   The dereference goes inside an `unsafe {}` block. The `// SAFETY:`
   comment records *why* it's sound — `unsafe` is a promise you make to
   the compiler, and the comment is you showing your work.
6. **`unsafe fn` & safety contracts.**
   ```rust
   /// # Safety
   /// `ptr` must be valid for reads of `len` consecutive `i32`s.
   unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 { /* ... */ }

   let v = [1, 2, 3];
   let total = unsafe { sum_raw(v.as_ptr(), v.len()) };
   ```
   When a function's correctness depends on invariants the *caller* must
   uphold, mark it `unsafe fn` and document them under `# Safety`.
   Calling it requires an `unsafe {}` block — the caller accepts the
   contract.
7. **Memory layout.** Safe tools to inspect layout:
   ```rust
   assert_eq!(std::mem::size_of::<i32>(), 4);
   assert_eq!(std::mem::align_of::<i32>(), 4);
   ```
   By default Rust may reorder a struct's fields for packing. `#[repr(C)]`
   forces a predictable, C-compatible layout — essential when you share
   data with C (Lesson 20).
8. **Soundness — the one rule.** Unsafe code must *never* cause undefined
   behavior (UB) — no matter how it's called. Practices: keep `unsafe`
   blocks tiny; wrap them in safe APIs that uphold the invariants;
   write a `// SAFETY:` for every one. The `miri` interpreter (Lesson 27)
   can catch many UB bugs in tests.
9. **Putting it together.** Walk through the exercises: `read_doubled`
   makes a raw pointer with `&raw const`, dereferences it in an `unsafe`
   block, and doubles the value (warm-up — contained unsafe in a safe
   fn); `sum_raw` is an `unsafe fn` that sums `len` values via
   `*ptr.add(i)` under a `# Safety` contract (main). The compile-fail
   dereferences a raw pointer with no `unsafe` block.
10. **Wrap — Phase 5 begins.** Five takeaways: `unsafe` unlocks a few
    operations, not a borrow-checker bypass; creating a raw pointer is
    safe, dereferencing isn't; `// SAFETY:` documents why each use is
    sound; an `unsafe fn` states a `# Safety` contract the caller upholds;
    unsafe code must never cause UB. Next: **Lesson 20 — FFI** (calling C
    from Rust and back).

## Exercise spec

`lessons/19-unsafe-memory/` follows the standard four-part lesson shape:

```
19-unsafe-memory/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/19-deref-outside-unsafe.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `unsafe-memory-exercises` and
`unsafe-memory-solutions` (the lesson's "bare" name is `unsafe-memory`;
the import idents are `unsafe_memory_exercises` /
`unsafe_memory_solutions`). This matches the build-index master registry
slug `unsafe-memory`, so the landing page links it without any change. No
external dependencies.

### Exercise stub (`exercises/src/lib.rs`)

The stub ships both signatures with `todo!()` bodies. The `unsafe fn`
`sum_raw` keeps its `# Safety` doc section even as a stub — clippy's
`missing_safety_doc` (a denied pedantic lint) requires it on any public
`unsafe fn`. The crate and its tests compile; the tests fail at runtime
with the `todo!()` panic.

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

### Warm-up: `read_doubled`

Reference solution:

```rust
#[must_use]
pub fn read_doubled(n: i32) -> i32 {
    let ptr = &raw const n;
    // SAFETY: `ptr` was just created from the live local `n`, so it is
    // non-null, aligned, and points to an initialized `i32`.
    let value = unsafe { *ptr };
    value * 2
}
```

Pedagogical packing: a *safe* function containing one sound `unsafe`
operation. `&raw const n` creates a `*const i32` directly (the modern
2024 raw-borrow operator — clippy's `borrow_as_ptr` rejects the older
`&n as *const i32` form, verified during design); creating the pointer is
safe. Dereferencing it (`*ptr`) requires the `unsafe {}` block, and the
`// SAFETY:` comment justifies why it's sound. Returns `i32`, so
`#[must_use]` is appropriate.

Four tests:

```rust
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
```

### Main: `sum_raw`

Reference solution:

```rust
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

Pedagogical packing: an `unsafe fn` — a function whose soundness depends
on a caller-upheld invariant, stated in the `# Safety` doc (required by
clippy's `missing_safety_doc`). It walks `len` elements with pointer
arithmetic (`ptr.add(i)`) and dereferences each inside an `unsafe {}`
block — required even inside an `unsafe fn` under the 2024 edition's
`unsafe_op_in_unsafe_fn` (verified). The explicit `for i in 0..len` loop
is used because `.add`/deref can't go in a plain iterator closure;
`clippy::needless_range_loop` does not fire (there is no slice being
indexed). Returns `i32`, so `#[must_use]`.

Four tests (all build an array, pass `as_ptr()`/`len()`, and call inside
`unsafe {}` with a `// SAFETY:` comment; arrays are used rather than
`vec![...]` because `clippy::useless_vec` rejects a `vec!` literal that's
never used as a `Vec` — verified):

```rust
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

**Eight tests total** (four warm-up + four main). Test arithmetic:
`read_doubled(-5)` → `-10`; `sum_raw([1,2,3,4])` → `10`;
`sum_raw([-2,5,10])` → `13`. All exercise code is sound (every raw
pointer points to a live, sufficiently-sized allocation), so the tests
pass deterministically.

### Compile-fail: `19-deref-outside-unsafe.rs`

Path: `exercises/compile_fails/19-deref-outside-unsafe.rs`. A
self-contained file that creates a raw pointer and dereferences it
*outside* an `unsafe` block. Ships broken; the student wraps the deref in
`unsafe { }`.

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

Pass condition: the student wraps the deref — `let value = unsafe { *ptr
};`. rustc reports E0133 "dereference of raw pointer is unsafe and
requires unsafe ... block" — verified during design. After the fix the
file compiles.

This is the lesson's centerpiece: dereferencing a raw pointer is the
canonical unsafe operation, and the `unsafe {}` block is how you take
responsibility for it.

## README structure

`lessons/19-unsafe-memory/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - What `unsafe` is (and isn't)
  - Raw pointers
  - The `unsafe {}` block and `// SAFETY:`
  - `unsafe fn` and `# Safety` contracts
  - Memory layout — `size_of`, `align_of`, `#[repr(C)]`
- **Exercises** — four subsections: Warm-up (`read_doubled`), Main
  (`sum_raw`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`unsafe fn` and `# Safety` contracts" and "What `unsafe` is" sections
are the heaviest — they carry the lesson's core idea.

## Lint expectations

Lesson 19's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design (the most lint-sensitive lesson so far; the exact forms
below were chosen specifically to satisfy clippy):

- The warm-up uses `&raw const n`, **not** `&n as *const i32`:
  `clippy::borrow_as_ptr` (pedantic) rejects the cast form and suggests
  the raw-borrow operator.
- `sum_raw` is a public `unsafe fn` and so carries a `# Safety` doc
  section — required by `clippy::missing_safety_doc`. The stub keeps this
  doc too.
- The deref inside `sum_raw` is wrapped in its own `unsafe {}` block even
  though the function is already `unsafe fn` — required by the 2024
  edition `unsafe_op_in_unsafe_fn` lint.
- `sum_raw` uses an explicit `for i in 0..len` loop; `clippy::needless_range_loop`
  does not fire because no slice is being indexed (it's pointer
  arithmetic).
- The tests build arrays (`[1, 2, 3, 4]`), not `vec![...]`:
  `clippy::useless_vec` rejects a `vec!` literal that is only borrowed,
  never used as a `Vec`.
- Both functions return `i32`, so each carries `#[must_use]` (not a
  `Result`, so no `double_must_use`).
- `read_doubled` takes `i32` (not a raw pointer), so
  `clippy::not_unsafe_ptr_arg_deref` does not apply; `sum_raw` takes a raw
  pointer but is correctly `unsafe fn`, so that lint does not fire either.

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/19-unsafe-memory/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`unsafe-memory-exercises`, `unsafe-memory-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `read_doubled` / `sum_raw` signatures (including the `# Safety` doc on
  `sum_raw`); the exercise ships `todo!()` bodies, the solution ships real
  bodies
- `cargo test --package unsafe-memory-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/19-unsafe-memory/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/19-unsafe-memory`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/19-unsafe-memory`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/19-unsafe-memory/slides/index.html`
- `dist/index.html` lists lesson 19 as a clickable link (registry slug
  `unsafe-memory` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/19-unsafe-memory/slides/`
  returns 200

## Open questions

None.
