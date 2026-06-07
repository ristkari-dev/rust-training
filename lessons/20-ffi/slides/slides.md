# FFI

> The C ABI is the common language every system speaks. FFI lets Rust call C functions and lets C call Rust — the same way you'd link any two native libraries. The catch: across that boundary, the compiler can't protect you.

---

## The C ABI — why FFI

Rust can interoperate with C — and anything that speaks the C ABI (C++, Python, Go, ...) — because the C calling convention is a stable, universal contract.

FFI is how you reuse an existing C library from Rust, or expose a Rust library to a C program.

---

## Calling into C

```rust
use core::ffi::c_int;

unsafe extern "C" {
    fn abs(input: c_int) -> c_int;   // declares libc's abs
}
```

An `unsafe extern "C"` block *declares* functions that exist elsewhere (here, in the C standard library, which every Rust program links). You promise the signature matches the real one.

---

## C-compatible types

The types on both sides must agree. `core::ffi` provides the C scalar types — `c_int`, `c_char`, `c_uint`, ... — which map to C's `int`, `char`, etc.

`c_int` is `i32` on common platforms. For aggregates you'd use `#[repr(C)]` structs (Lesson 19).

---

## The unsafe call

```rust
pub fn abs_via_c(n: i32) -> i32 {
    // SAFETY: `abs` is a pure libc function with no preconditions for
    // any `c_int` except `i32::MIN`.
    unsafe { abs(n) }
}
```

Every FFI call is `unsafe` — Rust can't verify the C side. The idiom is to wrap the unsafe call in a safe function with a `// SAFETY:` comment.

---

## Exporting to C

```rust
#[unsafe(no_mangle)]
pub extern "C" fn add_in_rust(a: c_int, b: c_int) -> c_int {
    a + b
}
```

`extern "C"` gives the function the C calling convention; `#[unsafe(no_mangle)]` keeps its symbol name exactly `add_in_rust` so a C program can link and call it. (In Rust 2024, `no_mangle` is written `#[unsafe(no_mangle)]`.)

---

## Boundary rules

Across FFI you give up Rust's guarantees, so:

- never let a panic unwind across the boundary (undefined behavior — catch it or abort)
- use only C-compatible types (`#[repr(C)]`, the `core::ffi` scalars)
- C knows nothing about Rust ownership — memory passed across must be managed by hand

---

## Tooling — `bindgen` & `cbindgen`

Writing `extern` declarations by hand is error-prone for a big API.

- **`bindgen`** generates Rust declarations from a C header
- **`cbindgen`** generates a C header from your Rust `extern "C"` exports

They automate the boilerplate the exercises do by hand.

---

## Putting it together

Today's exercises:

- **Warm-up** `abs_via_c` — declare libc's `abs` in an `unsafe extern "C"` block and call it (the import side)
- **Main** `add_in_rust` — export a Rust fn with `extern "C"` + `#[unsafe(no_mangle)]` (the export side)

The compile-fail calls a foreign function without an `unsafe` block.

---

## Wrap — bridging languages

- the C ABI is the universal interop contract
- `unsafe extern "C"` declares foreign functions
- every FFI call is `unsafe`
- `extern "C"` + `#[unsafe(no_mangle)]` exports Rust to C
- `bindgen` / `cbindgen` automate the declarations

Next: **Lesson 21 — I/O & the filesystem**.
