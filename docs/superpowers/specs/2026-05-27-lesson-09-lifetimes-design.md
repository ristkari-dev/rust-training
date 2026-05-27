# Lesson 09 — Lifetimes — design

The third lesson of Phase 2 (Ownership deep dive). Introduces lifetime
syntax (`'a`) and elision intuition: every reference has a lifetime,
the compiler infers it most of the time, and you learn the syntax for
the two cases where it can't.

## Audience and prerequisites

- Has completed Lessons 01-08
- Comfortable with `&T` / `&mut T` and the borrowing rules from
  Lesson 08
- Has seen `&'static str` mentioned in slides since L04 but treated
  it as opaque syntax
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Explain that every reference has a lifetime — a scope during
   which it's valid — and that the compiler tracks it
2. Identify when lifetime elision applies (single input reference,
   methods with `&self`) and when it doesn't
3. Write a function signature with explicit `<'a>` annotation when
   elision fails — e.g., a function returning a reference tied to
   one of two input references
4. Define a struct that holds a reference, declaring and using the
   lifetime parameter consistently across struct, `impl`, and methods
5. Recognize `&'static str` as a reference that lives for the entire
   program

## Scope

In scope: the *intuition* of lifetimes (scope during which a
reference is valid); elision behavior (informal — the rules-by-example
approach, not the formal three-rule enumeration); explicit `<'a>` on
functions and structs; `'static` mentioned as the special "lives for
the whole program" lifetime; the canonical `longest` function and
`Excerpt<'a>` struct examples.

Out of scope (deferred or skipped): the formal three elision rules
spelled out (taught as intuition, not memorized rules); the `'_`
placeholder syntax; higher-rank trait bounds (`for<'a>`); lifetime
subtyping / variance; multiple lifetime parameters with different
relationships; lifetimes on traits and trait objects (Lesson 13);
the `Drop` trait and how it interacts with lifetimes.

## Slide arc (10 slides)

1. **Title — Lifetimes.** Hook: *"Every reference has a lifetime.
   Most of the time, Rust figures it out for you. Today we learn the
   syntax for when it can't — and why."*
2. **Recap.** L08 introduced `&T` and `&mut T`. We mentioned in
   passing that every reference has a *lifetime* and that elision
   usually handles them silently. Today we open the box.
3. **Every reference has a lifetime.** Conceptually: a lifetime is
   the *scope during which a reference is valid*. The compiler tracks
   it for every reference. Most of the time it's implicit and you
   never see it. When you do — like `&'static str` for string
   literals — that's lifetime syntax surfacing.
4. **Elision handles the common cases.**
   ```rust
   fn first_char(s: &str) -> &str { /* ... */ }   // returned ref borrows from s
   fn area(rect: &Rectangle) -> u32 { /* ... */ } // no output borrow
   ```
   For one-input-reference functions and methods (`&self`), Rust
   deduces the relationship. You've been writing code like this
   since Lesson 04 without thinking about it.
5. **When elision fails.** Two reference parameters, one returned
   reference — the compiler can't guess which one the output
   borrows from:
   ```rust
   fn longest(a: &str, b: &str) -> &str {     // ERROR: ambiguous
       if a.len() >= b.len() { a } else { b }
   }
   ```
   rustc says: *"this function's return type contains a borrowed
   value, but the signature does not say whether it is borrowed
   from `a` or `b`."* It's up to you to spell out the relationship.
6. **`<'a>` syntax.**
   ```rust
   fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
       if a.len() >= b.len() { a } else { b }
   }
   ```
   Read this as: "for some lifetime `'a`, both inputs and the
   output all share that lifetime." The compiler picks `'a` at each
   call site to be the *shortest* of the two inputs' lifetimes —
   that's the lifetime of the returned reference.
7. **The `longest` function.** Walk through the warm-up exercise:
   - Without `'a`, the signature is ambiguous and won't compile.
   - With `'a`, the compiler accepts the body and the call site.
   - At each call, the returned reference is valid for as long as
     *both* input references are valid — the shorter of the two.
8. **Structs holding references.** The second case where lifetimes
   go explicit. A struct field that's a reference forces the struct
   itself to declare a lifetime parameter:
   ```rust
   struct Excerpt<'a> {
       text: &'a str,
   }
   ```
   This says: "the struct cannot outlive the `&str` it holds." If
   the original string is dropped, any `Excerpt` referencing it is
   invalidated by the compiler. The compile-fail exercise drills the
   canonical mistake — forgetting the `<'a>`.
9. **`'static` — the special lifetime.** `&'static str` is a
   reference that lives for the entire program. String literals are
   `&'static str` (slices into the compiled binary). Most other
   references have shorter, scope-limited lifetimes. You'll
   occasionally need `'static` in trait bounds and APIs, but it's
   not common in everyday function signatures.
10. **Wrap — Phase 2 progress.** Five takeaways: every reference has
    a lifetime; elision handles the common cases; explicit `<'a>` is
    needed for multi-input functions returning a reference, and for
    any struct with a reference field; lifetime parameters say "all
    these references share a relationship"; `'static` lives for the
    whole program. Next: **Lesson 10 — Smart pointers**.

## Exercise spec

`lessons/09-lifetimes/` follows the standard four-part lesson shape:

```
09-lifetimes/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/09-missing-lifetime.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `lifetimes-exercises` and `lifetimes-solutions`
(the lesson's "bare" name is `lifetimes`).

### Exercise stub (`exercises/src/lib.rs`)

The stub itself contains the lifetime annotations — students don't
have to write `<'a>` from scratch. The lesson is "understand what
these mean and why they're necessary."

```rust
//! Lesson 09 — exercises.
//!
//! Implement `longest` (warm-up) and the `Excerpt` struct's methods
//! (main) so that `cargo test --manifest-path
//! lessons/09-lifetimes/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn longest<'a>(_a: &'a str, _b: &'a str) -> &'a str {
    todo!("return whichever of a and b has the greater (or equal) length")
}

pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    #[must_use]
    pub fn new(_text: &'a str) -> Self {
        todo!("construct an Excerpt holding text")
    }

    #[must_use]
    pub fn length(&self) -> usize {
        todo!("return the length of the held text")
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        todo!("return whether the held text is empty")
    }
}
```

### Warm-up: `longest`

Signature:

```rust
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
```

Reference solution:

```rust
#[must_use]
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Four tests:

```rust
#[test]
fn warmup_b_is_longer() {
    assert_eq!(longest("hi", "hello"), "hello");
}

#[test]
fn warmup_a_is_longer() {
    assert_eq!(longest("hello", "hi"), "hello");
}

#[test]
fn warmup_equal_takes_a() {
    assert_eq!(longest("same", "size"), "same");
}

#[test]
fn warmup_empty_a() {
    assert_eq!(longest("", "anything"), "anything");
}
```

The "equal length → takes a" test pins the tie-break (the `>=`
comparison) so a student writing `>` and passing the other three
tests can't slip through.

### Main: `Excerpt<'a>`

Reference solution:

```rust
pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    #[must_use]
    pub fn new(text: &'a str) -> Self {
        Excerpt { text }
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.text.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}
```

Pedagogical packing: the `<'a>` lifetime parameter appears on the
struct declaration, on the `impl` block, and on the constructor's
parameter type — three places, all required and all related. The
`length` and `is_empty` methods take `&self` and return owned types
(`usize`/`bool`), so no further reference plumbing is needed.

Four tests:

```rust
#[test]
fn main_text_field_accessible() {
    let e = Excerpt::new("hello");
    assert_eq!(e.text, "hello");
}

#[test]
fn main_length() {
    assert_eq!(Excerpt::new("hello").length(), 5);
}

#[test]
fn main_is_empty_true() {
    assert!(Excerpt::new("").is_empty());
}

#[test]
fn main_is_empty_false() {
    assert!(!Excerpt::new("hi").is_empty());
}
```

**Eight tests total** (four warm-up + four main).

### Compile-fail: `09-missing-lifetime.rs`

Path: `exercises/compile_fails/09-missing-lifetime.rs`. Ships broken;
the student adds `<'a>` to the struct and `&'a str` to the field
until the file compiles.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Any struct that holds a reference must declare a lifetime parameter
// and use it on the field. Otherwise the compiler can't track how
// long the struct is valid — and refuses to compile.
//
// The struct below tries to hold a `&str` without a lifetime
// parameter. rustc will say "missing lifetime specifier" and suggest
// the fix.
//
// Hint: read the rustc error. The fix is to declare `<'a>` on the
// struct and use `&'a str` for the field. Like this:
//
//     struct Excerpt<'a> {
//         text: &'a str,
//     }

struct Excerpt {
    text: &str,
}

fn main() {
    let s = String::from("hello world");
    let e = Excerpt { text: &s };
    println!("{}", e.text);
}
```

Pass condition: student adds `<'a>` to the struct and `&'a str` to
the field. rustc's error message includes a `--help` suggestion with
the exact replacement code — part of the lesson is "the compiler
often tells you how to fix it; read carefully."

This compile-fail directly reinforces slide 8: structs with reference
fields *must* declare a lifetime parameter.

## README structure

`lessons/09-lifetimes/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - Every reference has a lifetime
  - Elision handles the common cases
  - When elision fails, and the `<'a>` syntax
  - Lifetimes in struct fields
  - `'static` — the special lifetime
- **Exercises** — four subsections: Warm-up (`longest`), Main
  (`Excerpt`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block.
The "When elision fails" section is the heaviest — it shows the
failing-elision example, the compiler's error in prose, and the
`<'a>` fix.

## Lint expectations

Lesson 09's reference solution code should be clippy-clean without
`#[allow]` attributes:

- `longest<'a>(a: &'a str, b: &'a str) -> &'a str` — elision does NOT
  apply here (two input lifetimes), so `<'a>` is required, not
  redundant. `clippy::needless_lifetimes` does not fire.
- `Excerpt<'a>` and its impl — explicit lifetimes are required
  because the struct holds a reference. Methods take `&self` and
  return owned types, so no further annotation needed beyond the
  `impl<'a>` block header.

If clippy fires on anything unexpected, fix the code rather than
adding allows.

## Done criteria

- `lessons/09-lifetimes/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`lifetimes-exercises`, `lifetimes-solutions`)
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the
  `Excerpt<'a>` struct with the same lifetime annotations
- `cargo test --package lifetimes-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/09-lifetimes/exercises/Cargo.toml`
  → both stubs panic with `todo!()`, the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/09-lifetimes`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/09-lifetimes`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/09-lifetimes/slides/index.html`
- `dist/index.html` lists lesson 09 as a clickable lesson
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/09-lifetimes/slides/` returns 200

## Open questions

None.
