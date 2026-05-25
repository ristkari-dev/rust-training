# Lesson 04 — Compound types — design

The fourth lesson of the Rust training course. Builds on Lessons 01-03 by
introducing tuples, arrays, slices, and the `String` vs `&str` dichotomy
— the densest cluster of foundational types in Phase 1.

## Audience and prerequisites

- Has completed Lessons 01, 02, and 03
- Comfortable with `let`/`mut`, `if`-as-expression, `for`/`while`/`loop`,
  expressions vs statements
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Construct, destructure, and index tuples
2. Read and use fixed-size arrays
3. Take a slice `&[T]` as a parameter so functions work for any-length
   inputs
4. Distinguish `String` (owned, heap) from `&str` (borrowed view) and
   choose the right one for a given API
5. Convert between `&str` and `String`, and recognize the canonical
   signature pattern *take `&str`, return `String`*

## Scope

In scope: tuple syntax (`(T, U, ...)`, `.0`/`.1`, `let (x, y) = ...`),
array syntax (`[T; N]`, indexing, `.len()`), slice syntax (`&[T]`,
`&arr[..]` ranges), `&str` as a UTF-8 slice, `String` as owned/heap,
`.push_str()` / `.push()` / `String::new()` / `String::from()` /
`.to_string()`, and the conversion idioms between `&str` and `String`.

Out of scope (deferred): `Vec` (Lesson 11 — Iterators & collections);
`HashMap`; iterator adapters and chains; `match` on slices (Lesson 05);
ownership rules and borrow-checker errors more complex than the lesson's
single compile-fail (Lessons 07-09); `String` methods beyond push, char
operations, indexing strings by byte/character ranges (`s.as_bytes()`,
`s.chars()` — touched later when iterators land); `Cow<'_, str>`.

## Slide arc (10 slides)

1. **Title — Compound types.** Hook: *"Rust has two ways to talk about a
   sequence of values: own them, or look at them through a slice. Most
   of the standard library is built on this distinction."*
2. **Recap.** Lessons 01-03 covered single values, mutation, and control
   flow. Today we group values into tuples and arrays, then meet the
   *slice* — the abstraction that makes those groups usable across
   function boundaries.
3. **Tuples.** `let pair: (i32, &str) = (42, "hi");`. Access by position:
   `pair.0`, `pair.1`. Destructure: `let (n, s) = pair;`. Tuples mix
   types; arrays don't.
4. **Arrays.** `let arr: [i32; 5] = [1, 2, 3, 4, 5];`. Fixed size, same
   element type. `arr[0]` to index. `arr.len()`. Length is part of the
   type — `[i32; 5]` and `[i32; 6]` are different.
5. **Slices.** `&[T]` is a borrowed view into a sequence. `&arr[1..3]`
   is the sub-slice from index 1 to 2. A function that takes `&[i32]`
   works for any size of array — this is the big payoff: it decouples
   your function from a specific length.
6. **`&str` is a slice too.** Where `&[u8]` is a view of raw bytes,
   `&str` is a view of bytes guaranteed to be valid UTF-8. String
   literals like `"hello"` are `&'static str` — slices pointing into the
   program binary itself.
7. **`String` — the owned, heap-allocated counterpart.** `String::from("hello")`
   allocates on the heap, owns its data, and can grow with `.push_str()`
   and `.push()`. Most string data you *build at runtime* is a `String`.
8. **`String` vs `&str` — when to use which.** Rule of thumb: **take
   `&str` as a parameter** (works for both `String` and literals);
   **return `String`** when you build new data. `String` → `&str` is
   automatic via `&s`; `&str` → `String` uses `.to_string()` or
   `String::from()`.
9. **Putting it together.** Walk through the signature
   `fn join_with_dashes(words: &[&str]) -> String`: `&[&str]` is a slice
   of string slices; `String` is the owned result we build up.
   Foreshadows the main exercise.
10. **Wrap.** Five takeaways: tuples mix types; arrays are fixed-size;
    slices `&[T]` decouple functions from sizes; `&str` is a slice,
    `String` is owned; the canonical signature pattern is *take `&str`,
    return `String`*. Next: Lesson 05 — pattern matching & enums.

## Exercise spec

`lessons/04-compound-types/` follows the standard four-part lesson shape:

```
04-compound-types/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/04-string-vs-str.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `compound-types-exercises` and
`compound-types-solutions` (the lesson's "bare" name is `compound-types`).

### Warm-up: `divmod`

Signature:

```rust
pub fn divmod(a: u32, b: u32) -> (u32, u32)
```

The exercise stub (`#[must_use]` for workspace `clippy::pedantic`):

```rust
#[must_use]
pub fn divmod(_a: u32, _b: u32) -> (u32, u32) {
    todo!("return (quotient, remainder)")
}
```

Reference solution (the canonical "build a tuple as the tail expression"):

```rust
#[must_use]
pub fn divmod(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}
```

Four tests:

```rust
#[test] fn warmup_typical()       { assert_eq!(divmod(10, 3), (3, 1)); }
#[test] fn warmup_exact()         { assert_eq!(divmod(20, 4), (5, 0)); }
#[test] fn warmup_divide_by_one() { assert_eq!(divmod(7, 1),  (7, 0)); }
#[test] fn warmup_zero_dividend() { assert_eq!(divmod(0, 5),  (0, 0)); }
```

Note: tests deliberately avoid `divmod(_, 0)` (which would panic). The
warm-up doesn't need to teach panic semantics; it just needs to teach
tuple construction.

### Main: `join_with_dashes`

Signature:

```rust
pub fn join_with_dashes(words: &[&str]) -> String
```

The exercise stub:

```rust
#[must_use]
pub fn join_with_dashes(_words: &[&str]) -> String {
    todo!("join words with '-' between them; return \"\" for empty input")
}
```

Reference solution (the canonical "build a `String` by iterating a slice"):

```rust
#[must_use]
pub fn join_with_dashes(words: &[&str]) -> String {
    let mut out = String::new();
    let mut first = true;
    for w in words {
        if !first {
            out.push('-');
        }
        out.push_str(w);
        first = false;
    }
    out
}
```

Pedagogical elements packed in:

- `&[&str]` parameter — a slice of `&str`, exercising the
  slice-of-slices insight from slide 9
- `String::new()` — empty owned, heap-allocated `String`
- `for w in words` — iterating a slice directly (no indexing)
- `let mut first = true;` — the canonical "do something between items
  but not before the first" pattern; reinforces Lesson 02 `mut`
- `out.push('-')` — push a single `char`
- `out.push_str(w)` — push a `&str` (deref coercion handles `&&str` →
  `&str` invisibly so this just works)
- Tail expression `out` returns the `String`

Four tests:

```rust
#[test] fn main_empty()  { assert_eq!(join_with_dashes(&[]),                       ""); }
#[test] fn main_single() { assert_eq!(join_with_dashes(&["solo"]),                 "solo"); }
#[test] fn main_two()    { assert_eq!(join_with_dashes(&["a", "b"]),               "a-b"); }
#[test] fn main_three()  { assert_eq!(join_with_dashes(&["red", "green", "blue"]), "red-green-blue"); }
```

**Eight tests total** (four warm-up + four main).

### Compile-fail: `04-string-vs-str.rs`

Path: `exercises/compile_fails/04-string-vs-str.rs`. Ships broken; the
student edits it until it compiles by converting the string literal to
an owned `String`.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// String literals like "hello" are NOT `String` — they're `&'static str`,
// slices into the program binary itself. The binding below declares its
// type as `String`, but the value on the right is a `&str`. rustc will
// tell you exactly that.
//
// Hint: convert the literal into an owned `String`. Two equivalent
// idioms:
//
//     let s: String = "hello".to_string();
//     let s: String = String::from("hello");
//
// Pick either.

fn greet() -> String {
    let s: String = "hello";
    s
}

fn main() {
    let g = greet();
    println!("{g}");
}
```

`make test` (and CI) runs the compile-fails tool with `--expect broken`
and sees `ok` for the shipped state. `make verify LESSON=04-compound-types`
runs `--expect compiles` and refuses to pass until the student converts
the literal.

This compile-fail directly reinforces slide 6's `&str` vs `String`
distinction: rustc's error message literally names both types in
conflict — "expected `String`, found `&str`" — putting the slide's
content into the compiler's own voice.

## README structure

`lessons/04-compound-types/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with these subsections:
  - Tuples
  - Arrays
  - Slices
  - `String` vs `&str`
  - Conversions and signature patterns
- **Exercises** — pointer to `make verify LESSON=04-compound-types`
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`String` vs `&str`" section is the heaviest — it's the conceptual
climax — and the "Conversions and signature patterns" section is the
practical synthesis that previews the exercise.

## Lint expectations

Lesson 04's reference solution code should be clippy-clean without
`#[allow]` attributes:

- `divmod` is a single-expression function — no lints fire.
- `join_with_dashes` uses `for w in words` (not indexed iteration), so
  `clippy::needless_range_loop` does not fire.
- The `let mut first = true;` flag pattern is idiomatic and lint-free.

The tests file's `assert_eq!` calls all compare `String`/`&str` or tuple
of integers — no `clippy::float_cmp` concern.

If clippy fires unexpectedly, the implementer should fix the code rather
than add an allow attribute (same convention as Lessons 02-03 noted).

## Done criteria

- `lessons/04-compound-types/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`compound-types-exercises`, `compound-types-solutions`)
- `cargo test --package compound-types-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/04-compound-types/exercises/Cargo.toml`
  → both stubs panic with `todo!()`, the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/04-compound-types`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/04-compound-types`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/04-compound-types/slides/index.html`
- `dist/index.html` lists lesson 04 as a clickable lesson
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/04-compound-types/slides/`
  returns 200

## Open questions

None.
