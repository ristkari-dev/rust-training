# Lesson 06 — Structs & methods

Methods aren't magic — they're functions in an `impl` block. Once
you've seen the grammar, the dot-syntax you've been using since Lesson
02 (`.push_str()`, `.powi()`, and so on) makes complete sense. Today
you'll define your own struct, attach methods to it, and write your
own `::new(...)` constructor. By the end you'll have everything Phase
1 set out to give you.

## Learning goals

- Define a custom `struct` with named fields and the standard derives
  (`Debug`, `PartialEq`, `Eq`)
- Construct a struct via struct-literal syntax and access fields with
  the dot operator
- Write methods inside an `impl` block using the three receiver kinds:
  `&self` for reading, `&mut self` for mutating, `self` for consuming
- Write an associated function — typically `pub fn new(...) -> Self` —
  and call it via `Type::new(...)`
- Recognize that the dot-syntax used in earlier lessons is method-call
  syntax and follows the same rules

## Self-study notes

### Defining a struct

A struct is a **product type** — a fixed bundle of named fields, each
with its own type:

```rust
#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

The derives are the same machinery you saw with enums in Lesson 05:
they give you printing (`Debug`) and equality comparison (`PartialEq`
/ `Eq`) for free.

By default the struct and its fields are private to the module they're
declared in. To make either visible to the rest of the world, prefix
with `pub`. The exercises in this lesson use both — `Counter`'s field
is private (encapsulation), `Rectangle`'s fields are public (plain
data).

### Constructing and reading fields

Construct a struct with a struct-literal, naming every field:

```rust
let rect = Rectangle { width: 10, height: 20 };
let area = rect.width * rect.height;  // 200
```

When a local variable has the same name as a field, Rust lets you
abbreviate with the **field-init shorthand**:

```rust
let width = 10;
let height = 20;
let rect = Rectangle { width, height };  // same as { width: width, height: height }
```

### `impl` blocks

Methods and associated functions go in an `impl` block:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

`impl Rectangle` says "the following items belong to `Rectangle`." A
type can have multiple `impl` blocks (useful for organizing by trait,
later) but conventionally you have one.

### Method receivers — `&self`, `&mut self`, `self`

A method is a function whose first parameter is one of three receiver
forms:

```rust
impl Rectangle {
    // Read-only: borrows self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Mutating: borrows self mutably
    fn double_width(&mut self) {
        self.width *= 2;
    }

    // Consuming: takes self by value
    fn into_parts(self) -> (u32, u32) {
        (self.width, self.height)
    }
}
```

Treat the three kinds as grammar for now:

- `&self` — read the fields
- `&mut self` — modify the fields (caller must hold the struct in a
  `let mut` binding)
- `self` — consume the value (the struct is gone after the call
  returns)

The deep "why" — what `&` and `&mut` and ownership *actually mean* —
lands in **Lesson 07** (the start of Phase 2). For Lesson 06 you just
need to pick the right receiver for what your method does.

### Associated functions and the `new` convention

A function in an `impl` block that has no `self` receiver is an
**associated function**. It's called via `Type::name(...)` rather than
`value.name(...)`:

```rust
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

let rect = Rectangle::new(3, 5);
```

`Self` (capital S) is shorthand for the surrounding type — `Self`
inside `impl Rectangle` means `Rectangle`. Using `Self` rather than the
name explicitly makes constructors easier to rename later.

The name `new` is convention, not a keyword. You've been calling
`String::new()`, `String::from(...)`, and `Some(...)` (which is
technically a tuple-struct constructor) since Lesson 04. Now you know
how to write your own.

## Exercises

### Warm-up: `Counter`

The exercises crate ships a `Counter` struct with one private field
(`count: u32`) and an `impl` block with three stub methods:

- `Counter::new()` — associated function, returns a counter with
  `count = 0`
- `counter.increment()` — takes `&mut self`, adds 1 to `count`
- `counter.value()` — takes `&self`, returns the current `count`

Fill in the three `todo!()` bodies. All three receiver kinds appear in
this tiny struct.

### Main: `Rectangle`

The exercises crate also ships a `Rectangle` struct with two public
fields (`width: u32`, `height: u32`) and three stub methods:

- `Rectangle::new(width, height)` — associated function, builds and
  returns a `Rectangle`
- `rect.area()` — takes `&self`, returns `width * height`
- `rect.is_square()` — takes `&self`, returns whether
  `width == height`

Use the field-init shorthand `Rectangle { width, height }` in `new` —
it's the idiomatic spelling when the local names match the field
names.

### Compile-fail

`exercises/compile_fails/06-cannot-mutate-via-shared-ref.rs` ships
with a method that takes `&self` but tries to mutate a field. rustc's
error names the receiver kind directly. The fix is a one-word
insertion.

### Run

```bash
make verify LESSON=06-structs
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
