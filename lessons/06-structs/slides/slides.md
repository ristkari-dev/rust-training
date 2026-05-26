# Structs & methods

> Methods aren't magic — they're functions in an `impl` block. Once you've seen the grammar, the dot-syntax you've been using since Lesson 02 makes complete sense.

---

## Recap

Phase 1 so far: values & types (L01-02), control flow (L03), compound types (L04), sum types & match (L05).

Today: the final foundation — **product types** (named fields, fixed shape) and the methods attached to them.

---

## Defining a struct

```rust
#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

- Named fields with explicit types
- `pub` on the struct or fields controls visibility
- Derive `Debug` to print, `PartialEq`/`Eq` to compare (same machinery as Lesson 05)

---

## Constructing and reading

```rust
let rect = Rectangle { width: 10, height: 20 };
let area = rect.width * rect.height;          // 200
```

The struct-literal must name **every** field.

When a local has the same name as the field, use the **shorthand**:

```rust
let width = 10;
let height = 20;
let rect = Rectangle { width, height };       // same as { width: width, height: height }
```

---

## `impl` blocks

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

- `impl Type { ... }` is where methods (and associated functions) live
- A type can have multiple `impl` blocks, but conventionally one
- The block is attached by name — `impl Rectangle` says "these methods belong to Rectangle"

---

## `&self` methods

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

let rect = Rectangle { width: 7, height: 7 };
rect.area();        // 49
rect.is_square();   // true
```

- `&self` borrows the struct for **reading**
- Called via dot-syntax: `rect.area()`
- `self.field` reaches the field through the receiver

---

## `&mut self` and `self`

```rust
impl Rectangle {
    fn double_width(&mut self) {
        self.width *= 2;        // &mut self lets us mutate
    }
}

let mut rect = Rectangle { width: 5, height: 10 };
rect.double_width();            // rect.width is now 10
```

The three receiver kinds:

- `&self`     — borrow for reading
- `&mut self` — borrow for mutating
- `self`      — take ownership (consumes the value)

The "why" behind these lands in **Lesson 07: Ownership & moves**. For now: read, modify, consume.

---

## Associated functions

A function in an `impl` block with **no `self` receiver** — called via `Type::name(...)`:

```rust
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

let rect = Rectangle::new(3, 5);
```

- `Self` is shorthand for the surrounding type
- `new` is the conventional constructor name — not a language requirement
- You've been calling `String::new()`, `String::from(...)` since Lesson 04. Now you know how to write your own.

---

## Putting it together

The main exercise: implement methods on `Rectangle`:

```rust
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
```

The compile-fail exercise: write `increment(&self) { self.count += 1; }` — the compiler will tell you exactly why it doesn't work.

---

## Wrap — Phase 1 complete

- **Structs** are product types — named fields, fixed shape
- **`impl` blocks** hold methods and associated functions
- **`&self`** reads, **`&mut self`** mutates, **`self`** consumes
- **`Type::new(...)`** is the constructor convention
- Methods are functions; the dot-syntax is just sugar

**Phase 1 complete.** You've now built up: values & types, control flow, compound types, sum types, product types.

Next: **Phase 2** opens with **Lesson 07 — Ownership & moves**.
