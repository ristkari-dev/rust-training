# Lesson 07 — Ownership & moves — design

The first lesson of Phase 2 (Ownership deep dive). Introduces the
three ownership rules, move semantics on assignment and function call,
`Copy` types, `.clone()`, and the pattern of returning ownership from
functions.

## Audience and prerequisites

- Has completed all of Phase 1 (Lessons 01-06)
- Comfortable with `String`/`&str` from Lesson 04, struct methods and
  receiver kinds from Lesson 06
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. State the three ownership rules and apply them when reading code
2. Explain that assignment of a non-`Copy` value transfers ownership,
   leaving the original binding unusable
3. Recognize when a function call moves a value (passing by value)
   versus when a `Copy` type is bitwise duplicated
4. Use `.clone()` deliberately to escape a move when keeping the
   original is genuinely needed
5. Write a function that takes ownership, mutates the value, and
   returns ownership — including the `mut s: String` parameter
   shorthand

## Scope

In scope: the three ownership rules; move semantics on `let t = s` and
on function calls; `Copy` as the marker that types skip the move
(integer types, `f64`, `bool`, `char`, plus tuples of `Copy` types);
non-`Copy` heap-owning types using `String` as the canonical example;
`.clone()` as the explicit duplication tool; returning ownership from
functions; `mut` in parameter position (`fn append_excl(mut s: String)`).

Out of scope (deferred): references and borrowing (`&T`, `&mut T`) —
**Lesson 08**; lifetimes and lifetime annotations — **Lesson 09**;
smart pointers (`Box`, `Rc`, `Arc`, `RefCell`) — **Lesson 10**; the
`Drop` trait explicitly (mentioned only in the "owner goes out of
scope" sense, not as a trait to implement); partial moves on structs
and enums (corner case, can wait); the `Copy` derive on user-defined
types (Lesson 12's trait-deriving lesson).

A small concession: `&` appears in the std-library method calls
students will make (e.g., `s.push_str(&other)`). The lesson
acknowledges this with one slide aside but does not teach borrowing —
**Lesson 08** is the proper home.

## Slide arc (10 slides)

1. **Title — Ownership & moves.** Hook: *"There is exactly one owner.
   When the owner goes out of scope, the value is dropped. Everything
   that surprises you about Rust descends from those two sentences."*
2. **Recap.** Phase 1 gave you the language's syntax. Today opens
   Phase 2 — *what Rust does that other languages don't*. Ownership is
   the first piece, and the foundation for everything that follows
   (borrowing, lifetimes, smart pointers).
3. **The three rules.** Each value has exactly one owner. There is
   only one owner at a time. When the owner goes out of scope, the
   value is dropped.
4. **Move on assignment.**
   ```rust
   let s = String::from("hello");
   let t = s;          // ownership moved: s -> t
   println!("{t}");
   // println!("{s}"); // ERROR: s no longer owns anything
   ```
5. **Move on function call.** Functions take ownership through their
   parameters:
   ```rust
   fn take(s: String) { /* ... */ }
   let s = String::from("hello");
   take(s);            // s moved in
   // take(s);         // ERROR: use after move
   ```
   The compile-fail exercise drills this exact case.
6. **`Copy` types skip the move.**
   ```rust
   let n: i32 = 5;
   let m = n;          // bitwise duplicated
   println!("{n} {m}");
   ```
   Primitive types (`i32`, `f64`, `bool`, `char`, plus tuples of Copy
   types) implement `Copy`. `String` is NOT Copy — it owns heap data.
7. **`.clone()` — explicit duplication.**
   ```rust
   let s = String::from("hello");
   take(s.clone());    // hand the function a copy
   println!("{s}");    // original still owns
   ```
   `clone` is deliberately explicit. Rust doesn't auto-clone because
   cloning a `String` allocates new heap memory.
8. **Returning ownership.**
   ```rust
   fn append_excl(mut s: String) -> String {
       s.push('!');
       s
   }
   let s = String::from("hi");
   let s = append_excl(s); // s moved in, returned, re-bound
   ```
   The `mut s` in the parameter list says "I want to mutate my local
   copy after taking ownership." Caller mutability and parameter
   mutability are independent.
9. **Putting it together.** Walk through `append_excl` (warm-up) and
   `swap_and_join` (main). Quick aside: references (`&str`, `&self`)
   appear in std-lib methods you'll call — those are *borrowing*, the
   topic of **Lesson 08**. For today, focus on what *your* function
   signatures say about ownership.
10. **Wrap — Phase 2 launched.** Five takeaways: one owner at a time;
    assignment and function calls move non-`Copy` values; `Copy` types
    skip the move; `.clone()` is the explicit escape hatch; returning
    ownership is the cleanest pattern. Next: **Lesson 08 — References
    & borrowing**.

## Exercise spec

`lessons/07-ownership/` follows the standard four-part lesson shape:

```
07-ownership/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/07-use-after-move.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `ownership-exercises` and `ownership-solutions`
(the lesson's "bare" name is `ownership`).

### Exercise stub (`exercises/src/lib.rs`)

```rust
//! Lesson 07 — exercises.
//!
//! Implement `append_excl` (warm-up) and `swap_and_join` (main) so
//! that `cargo test --manifest-path
//! lessons/07-ownership/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn append_excl(_s: String) -> String {
    todo!("take ownership of s, push '!' onto the end, return it")
}

#[must_use]
pub fn swap_and_join(_a: String, _b: String) -> String {
    todo!("return b followed by a space followed by a, e.g. swap_and_join(\"hello\", \"world\") -> \"world hello\"")
}
```

### Warm-up: `append_excl`

Signature:

```rust
pub fn append_excl(s: String) -> String
```

Reference solution (showcases the `mut` parameter binding):

```rust
#[must_use]
pub fn append_excl(mut s: String) -> String {
    s.push('!');
    s
}
```

Pedagogical packing: ownership flows in via the parameter, the `mut`
keyword lets the local binding mutate, `s.push('!')` mutates,
ownership flows out via the tail-expression return.

Four tests:

```rust
#[test]
fn warmup_typical() {
    assert_eq!(append_excl(String::from("hello")), "hello!");
}

#[test]
fn warmup_empty() {
    assert_eq!(append_excl(String::new()), "!");
}

#[test]
fn warmup_existing_punctuation() {
    assert_eq!(append_excl(String::from("oh no.")), "oh no.!");
}

#[test]
fn warmup_multibyte_chars() {
    assert_eq!(append_excl(String::from("café")), "café!");
}
```

The multibyte-chars test quietly demonstrates that `String::push` works
on UTF-8 strings — this avoids any beginner confusion about whether
"strings are arrays of bytes." Lesson 21 will treat the byte/char
distinction properly.

### Main: `swap_and_join`

Signature:

```rust
pub fn swap_and_join(a: String, b: String) -> String
```

Reference solution (mutation style — pedagogically rich):

```rust
#[must_use]
pub fn swap_and_join(a: String, b: String) -> String {
    let mut result = b;
    result.push(' ');
    result.push_str(&a);
    result
}
```

Both `a` and `b` are moved into the function. `b` is re-bound as
`mut result`, mutated, then returned. The `&a` in `push_str(&a)` is a
borrow — slide 9 (and the README) flag it as L08's topic.

The student may equivalently pass the tests with the `format!` one-liner:

```rust
pub fn swap_and_join(a: String, b: String) -> String {
    format!("{b} {a}")
}
```

Both are accepted by the tests. The reference solution uses the
mutation style because it surfaces the move+mutate pattern; the
`format!` version is idiomatic Rust but doesn't *show* the mutation.

Four tests:

```rust
#[test]
fn main_typical() {
    assert_eq!(
        swap_and_join(String::from("hello"), String::from("world")),
        "world hello"
    );
}

#[test]
fn main_single_chars() {
    assert_eq!(swap_and_join(String::from("a"), String::from("b")), "b a");
}

#[test]
fn main_empty_first() {
    assert_eq!(swap_and_join(String::new(), String::from("hi")), "hi ");
}

#[test]
fn main_empty_second() {
    assert_eq!(swap_and_join(String::from("hi"), String::new()), " hi");
}
```

**Eight tests total** (four warm-up + four main).

### Compile-fail: `07-use-after-move.rs`

Path: `exercises/compile_fails/07-use-after-move.rs`. Ships broken;
the student adds `.clone()` to the first call until the file compiles.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// In Rust, passing a value to a function transfers ownership — unless
// the type is `Copy` (i32, bool, etc.). `String` is NOT Copy; it owns
// heap data. After you pass a `String` to a function, you can't use
// the original binding any more — ownership has moved.
//
// The function below calls `print_string(s)` twice on the same
// binding. The second call is a use-after-move and will fail.
//
// Hint: read the rustc error. It will say "value used here after move"
// and point to where the value moved (the first call). The simplest
// fix is to call `print_string(s.clone())` on the first call so the
// function receives a copy and the original `s` stays usable.

fn print_string(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("hello");
    print_string(s);
    print_string(s);
}
```

Pass condition: student adds `.clone()` to the first call — making it
`print_string(s.clone());`. rustc's error literally points at both the
move site (first call) and the use site (second call), so the
pedagogical loop closes inside the compiler's diagnostic.

This compile-fail directly reinforces slides 5 and 7: rustc's error
message names "value used here after move" and the lesson's escape
hatch (`.clone()`) is the obvious one-token fix.

## README structure

`lessons/07-ownership/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - The three ownership rules
  - Moves on assignment and function calls
  - `Copy` types and why they're different
  - `.clone()` — explicit duplication
  - Returning ownership from functions
- **Exercises** — four subsections: Warm-up (`append_excl`), Main
  (`swap_and_join`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

A note in the README mirrors slide 9's aside:

> You'll see `&` in some of the std-lib method calls you make (like
> `push_str(&a)`). That's borrowing — it's how you give a function
> read-access to a value without transferring ownership. We cover
> borrowing properly in **Lesson 08**; for now, just type the `&`
> where the compiler asks for it.

This note appears in the "Returning ownership from functions"
subsection where it would naturally surface for a reader.

## Lint expectations

Lesson 07's reference solution code should be clippy-clean without
`#[allow]` attributes:

- `append_excl` uses `mut s: String` parameter, `s.push('!')`, tail
  return — all idiomatic.
- `swap_and_join` uses `let mut result = b;` then `push`/`push_str` —
  standard pattern. `&a` is the canonical borrow for `push_str`.

The tests file does not need `#![allow(clippy::float_cmp)]` — all
assertions compare `String` to `&str` literals.

If clippy fires on anything unexpected, fix the code rather than
adding allows.

## Done criteria

- `lessons/07-ownership/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`ownership-exercises`, `ownership-solutions`)
- `cargo test --package ownership-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/07-ownership/exercises/Cargo.toml`
  → both stubs panic with `todo!()`, the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/07-ownership`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/07-ownership`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/07-ownership/slides/index.html`
- `dist/index.html` lists lesson 07 as a clickable lesson
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/07-ownership/slides/` returns 200

## Open questions

None.
