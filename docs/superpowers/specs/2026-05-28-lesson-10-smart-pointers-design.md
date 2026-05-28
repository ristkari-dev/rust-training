# Lesson 10 — Smart pointers — design

The fourth lesson of Phase 2 (Ownership deep dive). Introduces the four
core smart pointers — `Box`, `Rc`, `RefCell`, `Arc` — conceptually, and
exercises `Box` deeply through recursive types.

## Audience and prerequisites

- Has completed Lessons 01-09
- Comfortable with ownership, borrowing, lifetimes, enums + `match`
  (L05), and methods
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Use `Box<T>` to heap-allocate a value and to define recursive types
2. Explain why a recursive type needs indirection (`Box`) — infinite
   size otherwise
3. Describe `Rc<T>` as shared ownership via reference counting
   (`Rc::new`, `Rc::clone`, `Rc::strong_count`)
4. Describe `RefCell<T>` as interior mutability with a runtime borrow
   check, and recognize the `Rc<RefCell<T>>` shared-mutable-state combo
5. Recognize `Arc<T>` as the thread-safe `Rc`, deferred for hands-on
   use to the concurrency lesson

## Scope

In scope: `Box<T>` for heap allocation and recursive types (the
exercises' focus); `Rc<T>` shared ownership (conceptual, slides + README);
`RefCell<T>` interior mutability (conceptual); `Rc<RefCell<T>>` combo
(conceptual); `Arc<T>` as thread-safe `Rc` (brief, with a forward
pointer to L16); recursion + `match` on recursive enums (reinforces L05).

Out of scope (deferred or skipped): `Box<dyn Trait>` trait objects
(Lesson 13); `Weak<T>` and reference cycles; `Arc<Mutex<T>>` hands-on
(Lesson 16 — Concurrency); `Cell<T>` (RefCell's Copy-only sibling);
custom `Deref`/`Drop` implementations (touched only as "mostly
automatic"); `Cow<'_, T>`; `Pin<T>`. The four pointers are all
*taught* on slides, but only `Box` is *exercised* — `Rc`/`RefCell`/`Arc`
have either contrived or heavy exercises at this stage, so they're
covered conceptually and reappear in context later.

## Slide arc (10 slides)

1. **Title — Smart pointers.** Hook: *"A smart pointer is a type that
   owns data like a normal value, but adds behavior: heap allocation,
   shared ownership, or interior mutability. Four of them cover almost
   everything."*
2. **Recap.** Phase 2 has been about ownership and borrowing. Smart
   pointers are types that *own* their data (so the ownership rules
   apply) but wrap it with extra capabilities. They're regular structs
   in the standard library — nothing magic.
3. **`Box<T>` — heap allocation.** `let b = Box::new(5);` puts a value
   on the heap and gives you an owner of known, pointer-size. Deref
   with `*b` (mostly automatic). Single owner, dropped when it goes out
   of scope — same ownership rules as any value. The simplest smart
   pointer.
4. **`Box<T>` for recursive types.**
   ```rust
   enum List {
       Cons(i32, Box<List>),
       Nil,
   }
   ```
   A `List` that contained a `List` directly would have infinite size
   — the compiler can't lay it out. `Box` is a fixed-size pointer to
   heap memory, breaking the cycle. The compile-fail exercise shows the
   error you get without it.
5. **`Rc<T>` — shared ownership.**
   ```rust
   let a = Rc::new(String::from("shared"));
   let b = Rc::clone(&a);
   println!("{}", Rc::strong_count(&a));  // 2
   ```
   When one value needs *multiple* owners (single-threaded), `Rc`
   tracks how many owners exist. The data is dropped only when the last
   `Rc` is dropped. `Rc::clone` is cheap — it bumps a counter, not the
   data.
6. **`RefCell<T>` — interior mutability.**
   ```rust
   let cell = RefCell::new(5);
   *cell.borrow_mut() += 1;
   println!("{}", cell.borrow());  // 6
   ```
   Normally you need `&mut` to mutate. `RefCell` lets you mutate
   through a shared `&` reference by moving the borrow check to
   *runtime*. Break the rules at runtime (two `borrow_mut`s at once)
   and it panics.
7. **`Rc<RefCell<T>>` — shared mutable state.**
   ```rust
   let shared = Rc::new(RefCell::new(0));
   let clone = Rc::clone(&shared);
   *clone.borrow_mut() += 1;
   println!("{}", shared.borrow());  // 1
   ```
   `Rc` gives shared ownership; `RefCell` gives the mutation. A common
   combo for tree/graph structures.
8. **`Arc<T>` — thread-safe `Rc`.** `Arc` is `Rc` with *atomic*
   reference counting, safe to share across threads. Same API
   (`Arc::new`, `Arc::clone`). We'll use `Arc<Mutex<T>>` (the
   thread-safe analogue of `Rc<RefCell<T>>`) in **Lesson 16**. For
   single-threaded code, prefer `Rc` — it's faster.
9. **Putting it together.** Walk through the exercises: a cons `List`
   with `sum` (warm-up — single recursion through `Box`) and a binary
   `Tree` with `count_nodes` (main — double recursion). The compile-fail
   drives home why `Box` is mandatory for recursive types.
10. **Wrap — Phase 2 nearly done.** Five takeaways: `Box<T>`
    heap-allocates and enables recursive types; `Rc<T>` gives shared
    ownership via reference counting; `RefCell<T>` moves borrow-checking
    to runtime for interior mutability; `Rc<RefCell<T>>` combines them
    for shared mutable state; `Arc<T>` is the thread-safe `Rc`. Next:
    **Lesson 11 — Iterators & collections** (`Vec`, `HashMap`), which
    closes out Phase 2.

## Exercise spec

`lessons/10-smart-pointers/` follows the standard four-part lesson shape:

```
10-smart-pointers/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/10-infinite-size.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `smart-pointers-exercises` and
`smart-pointers-solutions` (the lesson's "bare" name is
`smart-pointers`).

### Exercise stub (`exercises/src/lib.rs`)

The crate ships both recursive enums *already using `Box`* plus the
function stubs. Students implement the recursive function bodies — the
enums are given so the lesson focuses on writing recursion, and the
compile-fail teaches *why* the `Box` is there.

```rust
//! Lesson 10 — exercises.
//!
//! Implement `sum` (warm-up) and `count_nodes` (main) so that
//! `cargo test --manifest-path
//! lessons/10-smart-pointers/exercises/Cargo.toml` passes. The tests
//! live in `tests/exercise.rs`.

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub enum Tree {
    Leaf,
    Node(i32, Box<Tree>, Box<Tree>),
}

#[must_use]
pub fn sum(_list: &List) -> i32 {
    todo!("recursively sum the values in the cons list")
}

#[must_use]
pub fn count_nodes(_tree: &Tree) -> usize {
    todo!("recursively count the Node variants in the tree")
}
```

### Warm-up: `sum`

Reference solution:

```rust
#[must_use]
pub fn sum(list: &List) -> i32 {
    match list {
        List::Cons(value, rest) => value + sum(rest),
        List::Nil => 0,
    }
}
```

Pedagogical packing: matching a `&List` (match ergonomics binds
`value: &i32`, `rest: &Box<List>`), recursive call through the `Box`
(`&Box<List>` deref-coerces to `&List`), `Nil` base case.
`value + sum(rest)` works because `&i32 + i32` is a valid addition
(std provides the reference `Add` impl). If clippy ever suggests
dereferencing, `*value + sum(rest)` is the fallback — but it shouldn't
fire.

Four tests:

```rust
#[test]
fn warmup_nil() {
    assert_eq!(sum(&List::Nil), 0);
}

#[test]
fn warmup_single() {
    assert_eq!(sum(&List::Cons(5, Box::new(List::Nil))), 5);
}

#[test]
fn warmup_three() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    assert_eq!(sum(&list), 6);
}

#[test]
fn warmup_negatives_cancel() {
    let list = List::Cons(-5, Box::new(List::Cons(5, Box::new(List::Nil))));
    assert_eq!(sum(&list), 0);
}
```

### Main: `count_nodes`

Reference solution:

```rust
#[must_use]
pub fn count_nodes(tree: &Tree) -> usize {
    match tree {
        Tree::Node(_, left, right) => 1 + count_nodes(left) + count_nodes(right),
        Tree::Leaf => 0,
    }
}
```

Pedagogical packing: double recursion (left + right), the `_` pattern
ignoring the node's value, recursion through two `Box` children, `Leaf`
base case returning 0.

Four tests:

```rust
#[test]
fn main_leaf() {
    assert_eq!(count_nodes(&Tree::Leaf), 0);
}

#[test]
fn main_single_node() {
    let tree = Tree::Node(5, Box::new(Tree::Leaf), Box::new(Tree::Leaf));
    assert_eq!(count_nodes(&tree), 1);
}

#[test]
fn main_three_nodes() {
    let tree = Tree::Node(
        1,
        Box::new(Tree::Node(2, Box::new(Tree::Leaf), Box::new(Tree::Leaf))),
        Box::new(Tree::Node(3, Box::new(Tree::Leaf), Box::new(Tree::Leaf))),
    );
    assert_eq!(count_nodes(&tree), 3);
}

#[test]
fn main_lopsided() {
    let tree = Tree::Node(
        1,
        Box::new(Tree::Node(
            2,
            Box::new(Tree::Node(3, Box::new(Tree::Leaf), Box::new(Tree::Leaf))),
            Box::new(Tree::Leaf),
        )),
        Box::new(Tree::Leaf),
    );
    assert_eq!(count_nodes(&tree), 3);
}
```

**Eight tests total** (four warm-up + four main). The verbose
`Box::new(...)` nesting in the tests is itself instructive — students
see how recursive structures are built.

### Compile-fail: `10-infinite-size.rs`

Path: `exercises/compile_fails/10-infinite-size.rs`. Ships broken; the
student wraps the recursive field in `Box` until the file compiles.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// A recursive type — one that contains itself — has, in principle,
// infinite size: a List contains a List contains a List... forever.
// The compiler can't lay that out in memory, so it refuses.
//
// The fix is to put the recursive part behind a pointer of known,
// fixed size: Box<T>. A Box is just a pointer to heap memory, so its
// size is the same no matter how big the thing it points to is.
//
// rustc will say "recursive type `List` has infinite size" and even
// suggest the fix: insert a `Box`.
//
// Hint: change the recursive field from `List` to `Box<List>`.

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let _list = List::Nil;
}
```

Pass condition: student changes `Cons(i32, List)` to
`Cons(i32, Box<List>)`. rustc's error literally suggests inserting
`Box`. After the fix, `List::Nil` in `main` still compiles, so the only
change needed is the enum field.

This is the lesson's centerpiece: the compiler error isn't a rule being
enforced, it's a fundamental impossibility (a type can't have infinite
size), and `Box` is the resolution.

## README structure

`lessons/10-smart-pointers/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - `Box<T>` — heap allocation and recursive types
  - `Rc<T>` — shared ownership
  - `RefCell<T>` — interior mutability
  - `Rc<RefCell<T>>` — shared mutable state
  - `Arc<T>` — the thread-safe `Rc` (brief)
- **Exercises** — four subsections: Warm-up (`sum`), Main
  (`count_nodes`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`Box<T>`" section is the heaviest — it covers both heap allocation and
the recursive-types motivation that the exercises drill.

## Lint expectations

Lesson 10's reference solution code should be clippy-clean without
`#[allow]` attributes:

- `sum` matches and recurses; `value + sum(rest)` uses the reference
  `Add` impl (`&i32 + i32`). If `clippy::op_ref` or similar fires
  unexpectedly, `*value + sum(rest)` is the fallback.
- `count_nodes` uses `_` to ignore the node value and double recursion.
- The enums need no derives — tests compare `i32`/`usize` results, not
  enum values.

If clippy fires on anything unexpected, fix the code rather than adding
allows. The one documented fallback is `*value` in `sum` if the
reference-add form trips a lint.

## Done criteria

- `lessons/10-smart-pointers/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`smart-pointers-exercises`, `smart-pointers-solutions`)
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the
  `List` and `Tree` enums identically (with `Box`)
- `cargo test --package smart-pointers-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/10-smart-pointers/exercises/Cargo.toml`
  → both stubs panic with `todo!()`, the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/10-smart-pointers`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/10-smart-pointers`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/10-smart-pointers/slides/index.html`
- `dist/index.html` lists lesson 10 as a clickable lesson
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/10-smart-pointers/slides/`
  returns 200

## Open questions

None.
