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
let value = unsafe { *p };
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
    todo!()
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
sharing data with C (Lesson 20). The soundness rule always holds: unsafe
code must never cause undefined behavior.

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
