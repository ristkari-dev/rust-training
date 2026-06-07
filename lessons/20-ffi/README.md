# Lesson 20 — FFI

The C ABI is the common language every system speaks. FFI (Foreign
Function Interface) lets Rust call C functions and lets C call Rust — the
way you'd link any two native libraries. The catch: across that boundary,
the compiler can't protect you, so every call is `unsafe`. This lesson
covers both directions.

## Learning goals

- Declare a foreign C function in an `unsafe extern "C"` block and call
  it inside an `unsafe {}` block
- Explain that every FFI call is `unsafe` — Rust can't check the C side
- Use C-compatible types from `core::ffi` (e.g. `c_int`) to match a C
  signature
- Export a Rust function with the C ABI using `pub extern "C"` and
  `#[unsafe(no_mangle)]`
- Recognize the FFI boundary rules and the tooling (`bindgen`,
  `cbindgen`)

## Self-study notes

### The C ABI and `extern "C"`

Rust interoperates with C — and anything that speaks the C ABI (C++,
Python, Go, ...) — because the C calling convention is a stable,
universal contract. `extern "C"` selects that convention. FFI is how you
reuse a C library from Rust, or expose a Rust library to a C program.

### Calling into C — `unsafe extern` and C types

An `unsafe extern "C"` block *declares* functions defined elsewhere — here
in the C standard library, which every Rust program links:

```rust
use core::ffi::c_int;

unsafe extern "C" {
    fn abs(input: c_int) -> c_int;   // libc's abs
}
```

The types must match the real C signature. `core::ffi` provides the C
scalar types (`c_int`, `c_char`, ...); `c_int` is `i32` on common
platforms.

### The unsafe call and safe wrappers

Every FFI call is `unsafe` — Rust can't verify the C side. Wrap it in a
safe function with a `// SAFETY:` comment so callers get a safe API:

```rust
pub fn abs_via_c(n: i32) -> i32 {
    // SAFETY: `abs` is a pure libc function with no preconditions for any
    // `c_int` (except `i32::MIN`).
    unsafe { abs(n) }
}
```

### Exporting to C — `extern "C"` + `#[unsafe(no_mangle)]`

To let C call *your* Rust function, give it the C ABI and a stable symbol
name:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn add_in_rust(a: c_int, b: c_int) -> c_int {
    a + b
}
```

`extern "C"` sets the calling convention; `#[unsafe(no_mangle)]` (the Rust
2024 spelling) keeps the symbol name exactly `add_in_rust` so a C linker
can find it.

### Boundary rules and tooling

Across FFI you give up Rust's guarantees: never let a panic unwind across
the boundary (it's undefined behavior), use only C-compatible types
(`#[repr(C)]`, the `core::ffi` scalars), and manage any shared memory by
hand — C knows nothing about Rust ownership. Writing declarations by hand
scales poorly, so **`bindgen`** generates Rust declarations from a C
header and **`cbindgen`** generates a C header from your Rust exports.

## Exercises

### Warm-up: `abs_via_c`

Implement `abs_via_c(n: i32) -> i32`. Declare the C `abs` function in an
`unsafe extern "C"` block, then call it inside an `unsafe` block and
return the result:

```rust
// unsafe extern "C" { fn abs(input: c_int) -> c_int; }

pub fn abs_via_c(n: i32) -> i32 {
    // unsafe { abs(n) }
    todo!()
}
```

Add a `// SAFETY:` comment. `abs` comes from libc, which your test binary
already links — no build setup needed.

### Main: `add_in_rust`

Implement the exported function `add_in_rust(a: c_int, b: c_int) -> c_int`
(the `extern "C"` + `#[unsafe(no_mangle)]` attributes are given) so it
returns the sum of its two C integers:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn add_in_rust(a: c_int, b: c_int) -> c_int {
    // a + b
    todo!()
}
```

It's a normal Rust function too, so the tests call it directly — but the
C ABI and `#[unsafe(no_mangle)]` mean a C program could link and call it.

### Compile-fail

`exercises/compile_fails/20-ffi-call-no-unsafe.rs` calls a foreign
function outside an `unsafe` block, which the compiler rejects (E0133 — an
FFI call is unsafe). Fix it by wrapping the call in `unsafe { }`.

### Run

```bash
make verify LESSON=20-ffi
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
