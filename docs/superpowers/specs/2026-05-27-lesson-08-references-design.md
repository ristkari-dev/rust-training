# Lesson 08 — References & borrowing — design

The second lesson of Phase 2 (Ownership deep dive). Introduces `&T`
(shared references) and `&mut T` (mutable references), the borrowing
rules, and finally explains the `&self` / `&mut self` receiver kinds
from Lesson 06.

## Audience and prerequisites

- Has completed Lessons 01-07
- Understands moves and `Copy` from Lesson 07
- Comfortable with `&self`/`&mut self` *syntax* from Lesson 06 (without
  yet knowing what `&` means)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Explain what `&T` and `&mut T` are and write functions that take
   them as parameters
2. State the borrowing rules — at any moment for any value, either
   many shared references or one mutable reference, never both
3. Choose between `T`, `&T`, `&mut T`, and `clone()` when designing
   a function signature
4. Read rustc's borrow-checker errors and resolve them by reordering
   or dropping borrows
5. Connect `&self` / `&mut self` from Lesson 06 to the reference
   syntax — they are just `&T` and `&mut T` where `T` is the receiver
   type

## Scope

In scope: shared references `&T`; mutable references `&mut T`;
function signatures using both; the two-rule borrow checker (sharing
xor mutation); the connection between `&self` / `&mut self` and the
reference types; borrows end at last use (the "non-lexical lifetimes"
behavior, demonstrated by example without naming the term).

Out of scope (deferred): explicit lifetime annotations (`'a`, `'_`) —
**Lesson 09**; interior mutability via `Cell`, `RefCell`, `Mutex` —
**Lesson 10**; lifetime elision rules formally — **Lesson 09**;
`Cow<'_, str>`; references in struct fields (which require explicit
lifetimes) — **Lesson 09**.

A small concession: returning a `&str` from a function that takes a
single `&str` parameter relies on lifetime elision. We don't write any
such function in this lesson's exercises — they all either return
`String` (owned) or modify through a mutable reference and return
nothing. This keeps elision invisible.

## Slide arc (10 slides)

1. **Title — References & borrowing.** Hook: *"Lesson 07 taught moves.
   Today: the alternative. You can let a function look at your value
   — or even modify it — without giving up ownership. The compiler
   enforces some rules to keep this safe."*
2. **Recap.** L07: assigning or passing a `String` *moves* it; the
   original binding is dead. Today's alternative: **borrow** it. You
   hand the function a reference to the value and keep ownership
   yourself. References come in two flavors.
3. **`&T` — the shared reference.**
   ```rust
   let s = String::from("hello");
   let r: &String = &s;
   println!("len = {}", r.len());
   println!("{s}");        // s is still ours
   ```
   `&T` is a read-only borrow. Multiple shared references to the same
   value can coexist. The original owner stays valid.
4. **Functions taking `&T`.**
   ```rust
   fn show(s: &str) {
       println!("{s}");
   }

   let owned = String::from("hi");
   show(&owned);
   show(&owned);            // can call as many times as you like
   ```
   No `.clone()` needed. The function reads the value through the
   reference; the caller keeps ownership.
5. **`&mut T` — the mutable reference.**
   ```rust
   fn shout(s: &mut String) {
       s.push('!');
   }

   let mut owned = String::from("hi");
   shout(&mut owned);
   ```
   `&mut T` is a borrow with permission to modify. The caller's
   binding must be `mut` to hand out a mutable reference.
6. **The borrowing rules.** At any moment for any value:
   - **either** zero or more shared references (`&T`),
   - **or** exactly one mutable reference (`&mut T`),
   - **never both at once.**

   The borrow checker enforces this at compile time. The compile-fail
   exercise drills the canonical violation.
7. **`&self` / `&mut self` revisited.** The receiver kinds from
   Lesson 06 are now demystified — they're just `&T` and `&mut T`
   where `T` is the type the method belongs to:
   ```rust
   impl Rectangle {
       fn area(&self) -> u32 { /* reads */ }
       fn double_width(&mut self) { /* mutates */ }
   }
   ```
   `&self` borrows shared; `&mut self` borrows mutable; `self` moves
   (Lesson 07). Same three modes.
8. **Borrows vs moves — when to use which.** Decision table:

   | Situation | Use |
   |---|---|
   | Function only reads the value | `fn foo(x: &T)` |
   | Function modifies the value in place | `fn foo(x: &mut T)` |
   | Function takes the value to consume/transform | `fn foo(x: T) -> T` |
   | You need a copy you can hand around | `clone()` then pass owned |

   In real Rust, most functions take `&T` or `&mut T`. Ownership-taking
   is reserved for genuinely transforming the value or storing it
   somewhere new.
9. **Putting it together.** Walk through the exercises:
   - `wrap_in_quotes(s: &str) -> String` — borrow to read, return new
     owned data.
   - `merge_into(target: &mut String, parts: &[&str], separator: &str)`
     — one mut, two shared, all live together.

   Aside: every reference has a *lifetime* — for today, elision
   handles all our cases. **Lesson 09** is when we annotate them
   explicitly.
10. **Wrap — Phase 2 progress.** Five takeaways: `&T` reads, `&mut T`
    modifies, both leave the original owner intact; many shared OR one
    mutable, never both; `&self`/`&mut self` are these receiver kinds
    applied to methods; choose `&T` / `&mut T` / `T` based on what the
    function does; ownership transfer is rare in real Rust code. Next:
    **Lesson 09 — Lifetimes**.

## Exercise spec

`lessons/08-references/` follows the standard four-part lesson shape:

```
08-references/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/08-mut-and-shared.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `references-exercises` and `references-solutions`
(the lesson's "bare" name is `references`).

### Exercise stub (`exercises/src/lib.rs`)

```rust
//! Lesson 08 — exercises.
//!
//! Implement `wrap_in_quotes` (warm-up) and `merge_into` (main) so
//! that `cargo test --manifest-path
//! lessons/08-references/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn wrap_in_quotes(_s: &str) -> String {
    todo!("return a String containing s wrapped in double quotes, e.g. wrap_in_quotes(\"hi\") -> \"\\\"hi\\\"\"")
}

pub fn merge_into(_target: &mut String, _parts: &[&str], _separator: &str) {
    todo!("append parts joined by separator to target; do nothing if parts is empty")
}
```

### Warm-up: `wrap_in_quotes`

Signature:

```rust
pub fn wrap_in_quotes(s: &str) -> String
```

Reference solution:

```rust
#[must_use]
pub fn wrap_in_quotes(s: &str) -> String {
    let mut result = String::from("\"");
    result.push_str(s);
    result.push('"');
    result
}
```

Pedagogical packing: shared-reference parameter (`&str`); method calls
through the reference (`push_str(s)` — `&str` flows in directly);
owned `String` returned. The canonical "borrow to read, build new
owned data" pattern.

Four tests:

```rust
#[test]
fn warmup_typical() {
    assert_eq!(wrap_in_quotes("hello"), "\"hello\"");
}

#[test]
fn warmup_empty() {
    assert_eq!(wrap_in_quotes(""), "\"\"");
}

#[test]
fn warmup_with_spaces() {
    assert_eq!(wrap_in_quotes("hello world"), "\"hello world\"");
}

#[test]
fn warmup_with_inner_quote() {
    assert_eq!(wrap_in_quotes("she said \"hi\""), "\"she said \"hi\"\"");
}
```

The "inner quote" test deliberately doesn't escape the inner quotes —
`wrap_in_quotes` is a plain wrap, not a JSON-style escape. The
function's contract is "wrap with `\"`" not "produce parseable JSON."

### Main: `merge_into`

Signature:

```rust
pub fn merge_into(target: &mut String, parts: &[&str], separator: &str)
```

Reference solution (uses the `first: bool` flag pattern from L04):

```rust
pub fn merge_into(target: &mut String, parts: &[&str], separator: &str) {
    let mut first = true;
    for part in parts {
        if !first {
            target.push_str(separator);
        }
        target.push_str(part);
        first = false;
    }
}
```

This signature is the lesson's climax: **one mutable reference**
(`target`), **two shared references** (`parts`, `separator`), all
live during the function call. The rule "many shared OR one mutable"
is *per value* — different values each have their own borrows.

The body's `for part in parts` iterates `&[&str]` yielding `&&str`.
Deref coercion handles `&&str → &str` when passing to `push_str`
(same pattern as L04's `join_with_dashes`).

`merge_into` returns nothing — it modifies `target` in place. This
contrasts with L07's "take ownership, transform, return ownership"
pattern.

Four tests:

```rust
#[test]
fn main_typical() {
    let mut target = String::from("items: ");
    merge_into(&mut target, &["a", "b", "c"], "-");
    assert_eq!(target, "items: a-b-c");
}

#[test]
fn main_empty_parts_no_change() {
    let mut target = String::from("unchanged");
    merge_into(&mut target, &[], "-");
    assert_eq!(target, "unchanged");
}

#[test]
fn main_single_part_no_separator() {
    let mut target = String::new();
    merge_into(&mut target, &["solo"], "-");
    assert_eq!(target, "solo");
}

#[test]
fn main_multi_char_separator() {
    let mut target = String::new();
    merge_into(&mut target, &["one", "two", "three"], ", ");
    assert_eq!(target, "one, two, three");
}
```

**Eight tests total** (four warm-up + four main).

### Compile-fail: `08-mut-and-shared.rs`

Path: `exercises/compile_fails/08-mut-and-shared.rs`. Ships broken;
the student reorders the code so the shared borrow ends before the
mutable borrow begins.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Rust's borrowing rules: at any moment, for any value, you can have
// either
//   - any number of SHARED references (`&T`), OR
//   - exactly one MUTABLE reference (`&mut T`).
//
// You CANNOT have a mutable reference and shared references to the
// same value at the same time. The borrow checker enforces this — it
// prevents data races by construction.
//
// The function below tries to take a `&mut s` while a shared `&s` is
// still in use. rustc will say "cannot borrow `s` as mutable because
// it is also borrowed as immutable."
//
// Hint: read the rustc error. The fix is to drop the shared borrow
// before taking the mutable one — for example, by moving the
// `println!` of `r1` to BEFORE the line that takes `&mut s`. Once
// `r1` is no longer used, its borrow ends, and the mutable borrow
// becomes legal.

fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    r2.push_str(" world");
    println!("{r1}");
}
```

Pass condition: student reorders the code so the shared borrow's last
use happens before the mutable borrow starts. One valid rewrite (the
hint suggests):

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{r1}");
    let r2 = &mut s;
    r2.push_str(" world");
}
```

rustc's error message names both reference kinds — "cannot borrow as
mutable because it is also borrowed as immutable" — making the
diagnostic concrete. The fix surfaces the subtle but important
insight that **borrows end at last use** (not at end-of-scope), which
is the so-called "non-lexical lifetimes" behavior. The lesson doesn't
name the term; it just demonstrates the behavior by showing that
reordering fixes the problem.

## README structure

`lessons/08-references/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - `&T` — the shared reference
  - `&mut T` — the mutable reference
  - The borrowing rules
  - `&self` and `&mut self` revisited
  - Borrows vs moves — when to use which
- **Exercises** — four subsections: Warm-up (`wrap_in_quotes`), Main
  (`merge_into`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block.
The "borrowing rules" section is the heaviest — it states the rules
precisely, demonstrates the canonical violation, and notes that
borrows end at last use.

## Lint expectations

Lesson 08's reference solution code should be clippy-clean without
`#[allow]` attributes:

- `wrap_in_quotes` is `String::from(...)` + `push_str` + `push` + tail
  return — same pattern as L04.
- `merge_into` uses `for part in parts` + a `first: bool` flag — same
  pattern as L04's `join_with_dashes`. No `needless_range_loop` risk
  because we don't index.

If clippy fires on anything unexpected, fix the code rather than
adding allows.

## Done criteria

- `lessons/08-references/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`references-exercises`, `references-solutions`)
- `cargo test --package references-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/08-references/exercises/Cargo.toml`
  → both stubs panic with `todo!()`, the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/08-references`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/08-references`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/08-references/slides/index.html`
- `dist/index.html` lists lesson 08 as a clickable lesson
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/08-references/slides/`
  returns 200

## Open questions

None.
