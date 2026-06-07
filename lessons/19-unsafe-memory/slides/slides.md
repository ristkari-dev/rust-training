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
