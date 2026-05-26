# Lesson 06 — Structs & methods — design

The sixth and final lesson of Phase 1 (Programming 101 in Rust).
Introduces product types (`struct`), `impl` blocks, and method receivers
(`&self`, `&mut self`, `self`) plus associated functions.

## Audience and prerequisites

- Has completed Lessons 01-05
- Comfortable with enums + `match` from Lesson 05, the `#[derive(...)]`
  syntax pattern, and basic borrowing notation (`&str`) from Lesson 04
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Define a custom `struct` with named fields and the standard derives
   (`Debug`, `PartialEq`, `Eq`)
2. Construct a struct via struct-literal syntax and access fields with
   the dot operator
3. Write methods inside an `impl` block using the three receiver kinds:
   `&self` for reading, `&mut self` for mutating, `self` for consuming
4. Write an associated function — typically `pub fn new(...) -> Self` —
   and call it via `Type::new(...)`
5. Recognize that the dot-syntax used in earlier lessons
   (`.push_str()`, `.powi()`, etc.) is method-call syntax and follows
   the same rules

## Scope

In scope: named-field `struct` declarations; `pub`/private fields;
struct-literal construction; field access via `.`; `impl` blocks; the
three receiver kinds (`&self`, `&mut self`, `self`) at a working-grammar
level; associated functions (functions in an `impl` block with no
`self` receiver) and the `new` convention; the field-init shorthand
(`Rectangle { width, height }`); `Self` as return type in
constructors; `#[derive(Debug, PartialEq, Eq)]` on structs.

Out of scope (deferred): tuple structs (`struct Point(f64, f64)`) and
unit structs (`struct Marker;`) — too much shape variety for one
lesson; full borrowing/ownership semantics behind the three receivers
(Lesson 07 — Ownership & moves through Lesson 09 — Lifetimes);
explicit `Self` type usages beyond return type; lifetime annotations
on struct fields (Lesson 09); generic structs (Lesson 12); the
`Default` trait (Lesson 12); the deep "what does `&` mean here" story
— students treat the receiver kinds as grammar with "you'll see why
next lesson" pointers to ownership.

## Slide arc (10 slides)

1. **Title — Structs & methods.** Hook: *"Methods aren't magic — they're
   functions in an `impl` block. Once you've seen the grammar, the
   dot-syntax you've been using since Lesson 02 makes complete sense."*
2. **Recap.** Phase 1 has built up: values & types (L01-02), control
   flow (L03), compound types (L04), sum types & match (L05). Today is
   the final foundation — **product types** (named fields, fixed
   shape) and the methods attached to them.
3. **Defining a struct.** `struct Rectangle { width: u32, height: u32 }`.
   Fields are named and typed; `pub` controls visibility. Derive
   `Debug` to print, `PartialEq` / `Eq` to compare — same machinery as
   Lesson 05's enums.
4. **Constructing and reading.**
   `let rect = Rectangle { width: 10, height: 20 };` then access with
   `rect.width`. The struct-literal must name every field. Show the
   field-init shorthand: when a local has the same name as the field,
   `Rectangle { width, height }` works.
5. **`impl` blocks.** `impl Rectangle { ... }` — the block where
   methods live, attached to the type by name. A type can have multiple
   `impl` blocks; conventionally you have one.
6. **`&self` methods.** `fn area(&self) -> u32 { self.width * self.height }`.
   The `&self` receiver borrows the struct for reading. Called via
   dot-syntax: `rect.area()`. `self.field` reaches the field through
   the receiver.
7. **`&mut self` and `self`.** `&mut self` borrows for mutation:
   `fn double_width(&mut self) { self.width *= 2; }`. Bare `self`
   consumes the value (full ownership transfers in). For now, treat the
   difference as: `&self` reads, `&mut self` modifies, `self` consumes.
   The deep "why" lands in Lesson 07.
8. **Associated functions.** A function in an `impl` block with no
   `self` receiver — called via `Type::name(...)`. The convention is
   `new` as a constructor returning `Self`:

   ```rust
   impl Rectangle {
       pub fn new(width: u32, height: u32) -> Self {
           Rectangle { width, height }
       }
   }
   ```

   You've been calling `String::new()`, `String::from(...)` since
   Lesson 04 — now you know how to write your own.
9. **Putting it together.** Walk through the main exercise:
   `Rectangle::new(width, height)` constructor, `area(&self)` reading
   both fields, `is_square(&self)` returning `bool`. The compile-fail
   exercise drives slide 6/7's `&self` vs `&mut self` distinction
   home.
10. **Wrap — and Phase 1 close.** Five takeaways: structs are product
    types; `impl` blocks hold methods and associated functions; `&self`
    reads, `&mut self` mutates, `self` consumes; `Type::new(...)` is
    the constructor convention; methods are functions, the dot-syntax
    is just sugar. **Phase 1 complete.** Next: Phase 2 starts with
    **Lesson 07 — Ownership & moves**.

## Exercise spec

`lessons/06-structs/` follows the standard four-part lesson shape:

```
06-structs/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/06-cannot-mutate-via-shared-ref.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `structs-exercises` and `structs-solutions` (the
lesson's "bare" name is `structs`).

### Shared struct definitions

Both the exercises crate and the solutions crate ship the `Counter`
and `Rectangle` struct declarations plus the `impl` block scaffolding.
The exercise version has `todo!()` bodies for each method; the
solution version has the working implementations.

**Field visibility — a deliberate contrast:**

- `Counter`'s `count` field is **private**. Methods are the only API.
  This is the encapsulation pattern.
- `Rectangle`'s `width` and `height` fields are **`pub`**. Tests can
  read them directly via `rect.width`. This is the plain-data pattern.

Lesson 06 doesn't make a fuss about the contrast — but it's there for
students who notice, and Lesson 15 (Modules, crates, workspaces) will
return to it.

### Exercise stub (`exercises/src/lib.rs`)

```rust
//! Lesson 06 — exercises.

#[derive(Debug)]
pub struct Counter {
    count: u32,
}

impl Counter {
    #[must_use]
    #[allow(clippy::new_without_default)] // Default trait is Lesson 12
    pub fn new() -> Self {
        todo!("return a Counter with count = 0")
    }

    pub fn increment(&mut self) {
        todo!("add 1 to self.count")
    }

    #[must_use]
    pub fn value(&self) -> u32 {
        todo!("return self.count")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        todo!("build a Rectangle with the given fields")
    }

    #[must_use]
    pub fn area(&self) -> u32 {
        todo!("return width * height")
    }

    #[must_use]
    pub fn is_square(&self) -> bool {
        todo!("return whether width == height")
    }
}
```

### Warm-up: `Counter`

Reference solution:

```rust
impl Counter {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Counter { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    #[must_use]
    pub fn value(&self) -> u32 {
        self.count
    }
}
```

Pedagogical packing: all three receiver kinds (no-receiver associated
function, `&mut self`, `&self`) in one 10-line struct.

Four tests:

```rust
#[test]
fn warmup_new_starts_at_zero() {
    assert_eq!(Counter::new().value(), 0);
}

#[test]
fn warmup_increment_once() {
    let mut c = Counter::new();
    c.increment();
    assert_eq!(c.value(), 1);
}

#[test]
fn warmup_increment_thrice() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    c.increment();
    assert_eq!(c.value(), 3);
}

#[test]
fn warmup_two_counters_are_independent() {
    let mut a = Counter::new();
    let mut b = Counter::new();
    a.increment();
    a.increment();
    b.increment();
    assert_eq!(a.value(), 2);
    assert_eq!(b.value(), 1);
}
```

The "two counters are independent" test is the subtle one — it
confirms that each `Counter::new()` produces a fresh instance, not
aliased state.

### Main: `Rectangle`

Reference solution:

```rust
impl Rectangle {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    #[must_use]
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    #[must_use]
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
```

Note the **field-init shorthand**: `Rectangle { width, height }`
instead of `Rectangle { width: width, height: height }`. The slide
calls this out (slide 4).

Four tests:

```rust
#[test]
fn main_new_sets_fields() {
    let r = Rectangle::new(3, 5);
    assert_eq!(r.width, 3);
    assert_eq!(r.height, 5);
}

#[test]
fn main_area() {
    assert_eq!(Rectangle::new(3, 5).area(), 15);
}

#[test]
fn main_is_square_true() {
    assert!(Rectangle::new(7, 7).is_square());
}

#[test]
fn main_is_square_false() {
    assert!(!Rectangle::new(3, 5).is_square());
}
```

**Eight tests total** (four warm-up + four main).

### Compile-fail: `06-cannot-mutate-via-shared-ref.rs`

Path: `exercises/compile_fails/06-cannot-mutate-via-shared-ref.rs`.
Ships broken; the student changes `&self` to `&mut self` until the
file compiles.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Method receivers come in three kinds:
//   - `&self`     — borrow for reading
//   - `&mut self` — borrow for mutation
//   - `self`      — take ownership
//
// The `increment` method below tries to modify `self.count`, but its
// receiver is `&self` — a read-only borrow. The compiler refuses.
//
// Hint: read the rustc error. It will mention "cannot assign" and
// "behind a `&` reference". The fix is to change the receiver from
// `&self` to `&mut self`.

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&self) {
        self.count += 1;
    }

    fn value(&self) -> u32 {
        self.count
    }
}

fn main() {
    let mut c = Counter::new();
    c.increment();
    println!("{}", c.value());
}
```

Pass condition: student changes `fn increment(&self)` to
`fn increment(&mut self)` — one word insertion. rustc's error literally
names the issue — *"cannot assign to `self.count`, which is behind a
`&` reference"* — driving slides 6/7's distinction home in the
compiler's voice.

## README structure

`lessons/06-structs/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - Defining a struct
  - Constructing and reading fields
  - `impl` blocks
  - Method receivers — `&self`, `&mut self`, `self`
  - Associated functions and the `new` convention
- **Exercises** — four subsections: Warm-up (`Counter`), Main
  (`Rectangle`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"Method receivers" section is the heaviest — it's the conceptual
climax and what the compile-fail exercise drills.

## Lint expectations

Lesson 06's reference solution code needs **one** `#[allow]`:

- `#[allow(clippy::new_without_default)]` on `Counter::new()`.
  Justification: implementing `Default` would require introducing the
  `Default` trait, which is Lesson 12 territory. Same allow appears on
  the exercise stub so students don't have to think about it.

Other potential lints not expected to fire:

- `clippy::must_use_candidate` is preempted by `#[must_use]` on every
  appropriate function.
- `clippy::needless_pass_by_value` is allowed workspace-wide.
- `clippy::float_cmp` does not apply — all tests are integer/bool.

If clippy fires on anything else, fix the code rather than adding
allows.

## Done criteria

- `lessons/06-structs/` exists with the four-part structure
- Cargo manifests use the correct package names (`structs-exercises`,
  `structs-solutions`)
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define
  `Counter` and `Rectangle` with identical signatures and derives
- `cargo test --package structs-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/06-structs/exercises/Cargo.toml`
  → all method bodies panic with `todo!()`, the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/06-structs`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/06-structs`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/06-structs/slides/index.html`
- `dist/index.html` lists lesson 06 as a clickable lesson
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/06-structs/slides/` returns 200

## Open questions

None.
