# Lesson 20 — FFI — design

The second lesson of Phase 5 (Systems programming). The C ABI is the
lingua franca between languages; FFI (Foreign Function Interface) is how
Rust calls C and how C calls Rust. The lesson covers both directions:
*importing* (declaring and calling a C function) and *exporting* (exposing
a Rust function with the C ABI). Calling across the boundary is `unsafe`
— Rust can't verify the other side. `bindgen`/`cbindgen` are covered
conceptually as the tools that automate the boilerplate. Builds directly
on Lesson 19 (`unsafe`, `#[repr(C)]`).

A deliberate constraint: the exercises use **zero native-build
infrastructure** — no `.c` file, no `cc` crate, no `build.rs`. The
warm-up calls a real libc function (`abs`, always linked into every Rust
binary); the main exports a Rust function with the C ABI and is tested
from Rust. This keeps FFI genuinely hands-on while staying CI-robust.

## Audience and prerequisites

- Has completed Lessons 01-19
- Comfortable with `unsafe`, raw pointers, `// SAFETY:` / `# Safety`,
  and `#[repr(C)]` (L19); functions and types
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Declare a foreign C function in an `unsafe extern "C"` block and call
   it inside an `unsafe {}` block
2. Explain that every FFI call is `unsafe` — Rust cannot check the C
   side's behavior or signature
3. Use C-compatible types from `core::ffi` (e.g. `c_int`) to match a C
   signature
4. Export a Rust function with the C ABI using `pub extern "C"` and
   `#[unsafe(no_mangle)]` so C can call it by name
5. Recognize the FFI boundary rules (no unwinding across it, C-compatible
   types, manual memory management) and the tooling (`bindgen`,
   `cbindgen`)

## Scope

In scope: the C ABI and `extern "C"`; the Rust 2024 `unsafe extern "C"`
block for declaring foreign functions; calling a foreign function (always
`unsafe`); C-compatible scalar types via `core::ffi` (`c_int`); wrapping
an FFI call in a safe Rust API; exporting a Rust function to C with
`pub extern "C"` + `#[unsafe(no_mangle)]` (the Rust 2024 spelling); the
boundary rules (no panics/unwinding across FFI, use `#[repr(C)]` types,
C doesn't understand Rust ownership) conceptually; `bindgen`/`cbindgen`
conceptually. The warm-up calls libc's `abs`; the main exports an
`add_in_rust` function.

Out of scope (deferred or skipped): compiling a hand-written `.c` file
(no `cc` crate / `build.rs` / C-toolchain dependency); passing pointers,
strings (`CString`/`CStr`/`c_char`), or structs across the boundary
beyond a mention; `#[repr(C)]` struct layout in the exercises (covered in
L19); callbacks / function pointers across FFI; `catch_unwind` and
`extern "C-unwind"`; dynamic loading (`libloading`); linking external
libraries with `#[link]`; actually running `bindgen`/`cbindgen` (they're
described, not invoked); variadics; ABI other than `"C"`. FFI is
introduced as *call one C function, export one Rust function*; the
pointer/string marshalling and build-integration depth is out of band.

## Slide arc (10 slides)

1. **Title — FFI.** Hook: *"The C ABI is the common language every system
   speaks. FFI lets Rust call C functions and lets C call Rust — the same
   way you'd link any two native libraries. The catch: across that
   boundary, the compiler can't protect you."*
2. **The C ABI — why FFI.** Rust can interoperate with C (and anything
   that speaks the C ABI: C++, Python, Go, ...) because the C calling
   convention is a stable, universal contract. FFI is how you reuse an
   existing C library from Rust, or expose a Rust library to a C program.
3. **Calling into C.**
   ```rust
   use core::ffi::c_int;

   unsafe extern "C" {
       fn abs(input: c_int) -> c_int;   // declares libc's abs
   }
   ```
   An `unsafe extern "C"` block *declares* functions that exist elsewhere
   (here, in the C standard library, which every Rust program links).
   You're promising the signature matches the real one.
4. **C-compatible types.** The types on both sides must agree. `core::ffi`
   provides the C scalar types — `c_int`, `c_char`, `c_uint`, ... — which
   map to C's `int`, `char`, etc. (`c_int` is `i32` on common platforms.)
   For aggregates you'd use `#[repr(C)]` structs (Lesson 19).
5. **The unsafe call.**
   ```rust
   pub fn abs_via_c(n: i32) -> i32 {
       // SAFETY: `abs` is a pure libc function with no preconditions for
       // any `c_int` except `i32::MIN`.
       unsafe { abs(n) }
   }
   ```
   Every FFI call is `unsafe` — Rust can't verify the C side won't
   misbehave. The idiom is to wrap the unsafe call in a safe function with
   a `// SAFETY:` comment, giving callers a safe API.
6. **Exporting to C.**
   ```rust
   #[unsafe(no_mangle)]
   pub extern "C" fn add_in_rust(a: c_int, b: c_int) -> c_int {
       a + b
   }
   ```
   `extern "C"` gives the function the C calling convention;
   `#[unsafe(no_mangle)]` keeps its symbol name exactly `add_in_rust` (no
   Rust name-mangling) so a C program can link and call it. (In Rust 2024,
   `no_mangle` is written `#[unsafe(no_mangle)]`.)
7. **Boundary rules.** Across FFI you give up Rust's guarantees, so:
   never let a panic unwind across the boundary (it's undefined behavior —
   catch it or abort); use only C-compatible types (`#[repr(C)]`, the
   `core::ffi` scalars); and remember C knows nothing about Rust ownership
   or lifetimes — memory passed across must be managed by hand.
8. **Tooling — `bindgen` & `cbindgen`.** Writing `extern` declarations by
   hand is error-prone for a big API. **`bindgen`** generates Rust
   declarations from a C header; **`cbindgen`** generates a C header from
   your Rust `extern "C"` exports. They automate the boilerplate the
   exercises do by hand.
9. **Putting it together.** Walk through the exercises: `abs_via_c`
   declares libc's `abs` in an `unsafe extern "C"` block and calls it in
   an `unsafe {}` block, wrapped in a safe API (warm-up — the import
   side); `add_in_rust` is exported with `extern "C"` +
   `#[unsafe(no_mangle)]` (main — the export side). The compile-fail calls
   a foreign function without an `unsafe` block.
10. **Wrap — bridging languages.** Five takeaways: the C ABI is the
    universal interop contract; `unsafe extern "C"` declares foreign
    functions; every FFI call is `unsafe`; `extern "C"` +
    `#[unsafe(no_mangle)]` exports Rust to C; `bindgen`/`cbindgen`
    automate the declarations. Next: **Lesson 21 — I/O & the filesystem**.

## Exercise spec

`lessons/20-ffi/` follows the standard four-part lesson shape:

```
20-ffi/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/20-ffi-call-no-unsafe.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `ffi-exercises` and `ffi-solutions` (the lesson's
"bare" name is `ffi`; the import idents are `ffi_exercises` /
`ffi_solutions`). This matches the build-index master registry slug
`ffi`, so the landing page links it without any change. No external
dependencies; `abs` is resolved from libc, which every Rust binary links.

### Exercise stub (`exercises/src/lib.rs`)

The stub ships both signatures with `todo!()` bodies. It does **not**
ship the `unsafe extern "C" { fn abs ... }` block — an unused foreign
declaration trips `dead_code` (in the denied `unused` group), verified
during design. The student writes the `extern` block as part of
implementing `abs_via_c`, which also teaches declaring a foreign
function. The `use core::ffi::c_int;` import stays because `add_in_rust`'s
signature uses it. The crate and its tests compile; the tests fail at
runtime with the `todo!()` panic.

```rust
//! Lesson 20 — exercises.
//!
//! Implement `abs_via_c` (warm-up) and `add_in_rust` (main) so that
//! `cargo test --manifest-path lessons/20-ffi/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

use core::ffi::c_int;

#[must_use]
pub fn abs_via_c(_n: i32) -> i32 {
    todo!("declare the C `abs` function in an `unsafe extern \"C\"` block, then call it in an `unsafe` block and return its result")
}

#[unsafe(no_mangle)]
pub extern "C" fn add_in_rust(_a: c_int, _b: c_int) -> c_int {
    todo!("add the two C integers and return the result")
}
```

### Warm-up: `abs_via_c`

Reference solution:

```rust
use core::ffi::c_int;

unsafe extern "C" {
    fn abs(input: c_int) -> c_int;
}

#[must_use]
pub fn abs_via_c(n: i32) -> i32 {
    // SAFETY: `abs` is a pure libc function with no preconditions for any
    // `c_int` argument (except `i32::MIN`, which is not used here).
    unsafe { abs(n) }
}
```

Pedagogical packing: the import side of FFI. `unsafe extern "C" { fn abs
... }` declares libc's `abs` (resolved at link time — libc is always
linked, so no build step is needed). `c_int` (from `core::ffi`) matches
C's `int`; it is `i32` on common platforms, so passing `n: i32` is exact.
The call is `unsafe` (Rust can't verify the C side), wrapped in a safe
`abs_via_c` API with a `// SAFETY:` comment — the idiomatic safe-wrapper
pattern. Returns `i32`, so `#[must_use]`.

Four tests:

```rust
#[test]
fn warmup_abs_negative() {
    assert_eq!(abs_via_c(-5), 5);
}

#[test]
fn warmup_abs_positive() {
    assert_eq!(abs_via_c(7), 7);
}

#[test]
fn warmup_abs_zero() {
    assert_eq!(abs_via_c(0), 0);
}

#[test]
fn warmup_abs_large() {
    assert_eq!(abs_via_c(-1000), 1000);
}
```

### Main: `add_in_rust`

Reference solution:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn add_in_rust(a: c_int, b: c_int) -> c_int {
    a + b
}
```

Pedagogical packing: the export side of FFI. `extern "C"` gives the
function the C calling convention; `#[unsafe(no_mangle)]` (the Rust 2024
spelling) keeps the symbol name exactly `add_in_rust` so a C program could
link and call it. It takes and returns `c_int` (C-compatible types). The
logic is trivial on purpose — the lesson is the *boundary*, not the
arithmetic. The function is a normal Rust function too, so the tests call
it directly. (`#[must_use]` is not used: `extern "C"` exported functions
conventionally omit it, and clippy does not require it here.)

Four tests:

```rust
#[test]
fn main_add_basic() {
    assert_eq!(add_in_rust(2, 3), 5);
}

#[test]
fn main_add_zero() {
    assert_eq!(add_in_rust(0, 0), 0);
}

#[test]
fn main_add_negative() {
    assert_eq!(add_in_rust(-4, 10), 6);
}

#[test]
fn main_add_commutes() {
    assert_eq!(add_in_rust(8, -8), 0);
}
```

**Eight tests total** (four warm-up + four main). Test arithmetic:
`abs_via_c(-5)` → `5`; `add_in_rust(-4, 10)` → `6`. The warm-up tests
exercise a real libc call (linked via `cargo test`); the main tests call
the exported function directly from Rust.

### Compile-fail: `20-ffi-call-no-unsafe.rs`

Path: `exercises/compile_fails/20-ffi-call-no-unsafe.rs`. A self-contained
file that declares a foreign function and calls it *outside* an `unsafe`
block. Ships broken; the student wraps the call in `unsafe { }`. The
`compile-fails` tool type-checks with `rustc --crate-type=lib
--emit=metadata` (no linking), so the E0133 error fires at type-check and
libc never needs to be linked (verified during design).

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Calling a foreign (C) function is one of the operations the compiler
// can't verify: it has no way to check that the C side matches the
// declared signature or behaves well. So an FFI call — like dereferencing
// a raw pointer — is only allowed inside an `unsafe` block. Here the call
// to `abs` is in ordinary safe code, so the compiler rejects it.
//
// rustc reports E0133: "call to unsafe function `abs` is unsafe and
// requires unsafe block".
//
// The fix: wrap the call in an `unsafe { }` block (in real code, behind a
// safe wrapper with a `// SAFETY:` comment).
//
// Hint: change `let result = abs(-5);` to `let result = unsafe { abs(-5) };`.

use core::ffi::c_int;

unsafe extern "C" {
    fn abs(input: c_int) -> c_int;
}

fn main() {
    let result = abs(-5);
    println!("{result}");
}
```

Pass condition: the student wraps the call — `let result = unsafe {
abs(-5) };`. rustc reports E0133 "call to unsafe function `abs` is unsafe
and requires unsafe block" — verified during design. After the fix the
file type-checks.

This is the lesson's centerpiece: crossing the FFI boundary is inherently
unsafe, and the `unsafe {}` block is how the caller takes responsibility
for it — the same rule as raw-pointer deref in Lesson 19.

## README structure

`lessons/20-ffi/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - The C ABI and `extern "C"`
  - Calling into C — `unsafe extern` and C types
  - The unsafe call and safe wrappers
  - Exporting to C — `extern "C"` + `#[unsafe(no_mangle)]`
  - Boundary rules and tooling (`bindgen`/`cbindgen`)
- **Exercises** — four subsections: Warm-up (`abs_via_c`), Main
  (`add_in_rust`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"Calling into C" and "Exporting to C" sections are the heaviest — they
carry the two directions of the boundary.

## Lint expectations

Lesson 20's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design:

- `abs_via_c` is a safe wrapper containing one `unsafe` FFI call with a
  `// SAFETY:` comment; it returns `i32`, so it carries `#[must_use]`.
- The `unsafe extern "C"` block (Rust 2024 requires the `unsafe` keyword)
  and the `core::ffi::c_int` type compile clean.
- `add_in_rust` uses `extern "C"` + `#[unsafe(no_mangle)]` (the Rust 2024
  spelling of `no_mangle`); no `#[must_use]` is applied to it.
- The **exercise stub** omits the `unsafe extern "C"` block (an unused
  foreign declaration trips `dead_code` in the denied `unused` group);
  the student adds it. The `use core::ffi::c_int;` import stays because
  `add_in_rust`'s signature uses it. The stub lints clean (verified).
- The two crates each export a `#[unsafe(no_mangle)]` `add_in_rust`; this
  does not collide because each crate is compiled to its own test binary
  (no single artifact links both).

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/20-ffi/` exists with the four-part structure
- Cargo manifests use the correct package names (`ffi-exercises`,
  `ffi-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `abs_via_c` / `add_in_rust` signatures; the exercise ships `todo!()`
  bodies (and no `extern` block), the solution ships real bodies (with the
  `unsafe extern "C"` block)
- `cargo test --package ffi-solutions` → 8 tests pass (the warm-up tests
  link and call libc's `abs`)
- `cargo test --manifest-path lessons/20-ffi/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/20-ffi`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/20-ffi`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/20-ffi/slides/index.html`
- `dist/index.html` lists lesson 20 as a clickable link (registry slug
  `ffi` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/20-ffi/slides/` returns 200

## Open questions

None.
