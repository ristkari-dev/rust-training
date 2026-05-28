# Lesson 10 — Smart pointers — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the fourth lesson of Phase 2 of the Rust training course: smart pointers. Slides cover all four (`Box`, `Rc`, `RefCell`, `Arc`); exercises drill `Box` through recursive types. Warm-up is a cons-list `sum` (single recursion). Main is a binary-tree `count_nodes` (double recursion). Compile-fail is the iconic "recursive type without `Box` has infinite size."

**Architecture:** Use the existing `make new-lesson` scaffolder. The exercise crate ships the two recursive enums (`List`, `Tree`) *already using `Box`* plus the function stubs — students implement the recursive bodies. The compile-fail teaches why `Box` is necessary.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-28-lesson-10-smart-pointers-design.md`](../specs/2026-05-28-lesson-10-smart-pointers-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/10-smart-pointers

**Files (all created by the scaffolder):**
- `lessons/10-smart-pointers/README.md` (placeholder, replaced in Task 4)
- `lessons/10-smart-pointers/slides/index.html` (final — no edit needed)
- `lessons/10-smart-pointers/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/10-smart-pointers/exercises/Cargo.toml` (final — no edit needed)
- `lessons/10-smart-pointers/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/10-smart-pointers/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/10-smart-pointers/solutions/Cargo.toml` (final — no edit needed)
- `lessons/10-smart-pointers/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/10-smart-pointers/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=10-smart-pointers
```

Expected: `scaffolded ./lessons/10-smart-pointers`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/10-smart-pointers/
ls lessons/10-smart-pointers/slides/ lessons/10-smart-pointers/exercises/ lessons/10-smart-pointers/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/10-smart-pointers/exercises/Cargo.toml lessons/10-smart-pointers/solutions/Cargo.toml
```

Expected:
```
lessons/10-smart-pointers/exercises/Cargo.toml:name = "smart-pointers-exercises"
lessons/10-smart-pointers/solutions/Cargo.toml:name = "smart-pointers-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"smart-pointers-[^"]*"' | sort -u
```

Expected output:
```
"name":"smart-pointers-exercises"
"name":"smart-pointers-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/10-smart-pointers
git commit -m "chore: scaffold lessons/10-smart-pointers"
```

---

## Task 2: Exercise content (recursive enums + stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/10-smart-pointers/exercises/src/lib.rs`
- Overwrite: `lessons/10-smart-pointers/exercises/tests/exercise.rs`
- Create: `lessons/10-smart-pointers/exercises/compile_fails/10-infinite-size.rs`

- [ ] **Step 1: Overwrite `lessons/10-smart-pointers/exercises/src/lib.rs`**

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

- [ ] **Step 2: Overwrite `lessons/10-smart-pointers/exercises/tests/exercise.rs`**

```rust
use smart_pointers_exercises::{List, Tree, count_nodes, sum};

// Warm-up: sum

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

// Main: count_nodes

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

- [ ] **Step 3: Create `lessons/10-smart-pointers/exercises/compile_fails/10-infinite-size.rs`**

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

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/10-smart-pointers/exercises/Cargo.toml
```

Expected: all 8 tests fail with `not yet implemented` panic message.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package smart-pointers-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/10-smart-pointers
```

Expected: `ok: lessons/10-smart-pointers/exercises/compile_fails/10-infinite-size.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/10-smart-pointers
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/10-smart-pointers/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package smart-pointers-exercises --all-targets -- -D warnings
cargo fmt --check --package smart-pointers-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/10-smart-pointers/exercises
git commit -m "feat(lesson-10): add recursive enums, exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/10-smart-pointers/solutions/src/lib.rs`
- Overwrite: `lessons/10-smart-pointers/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/10-smart-pointers/solutions/src/lib.rs`**

```rust
//! Lesson 10 — reference solutions.

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub enum Tree {
    Leaf,
    Node(i32, Box<Tree>, Box<Tree>),
}

#[must_use]
pub fn sum(list: &List) -> i32 {
    match list {
        List::Cons(value, rest) => value + sum(rest),
        List::Nil => 0,
    }
}

#[must_use]
pub fn count_nodes(tree: &Tree) -> usize {
    match tree {
        Tree::Node(_, left, right) => 1 + count_nodes(left) + count_nodes(right),
        Tree::Leaf => 0,
    }
}
```

> Pedagogical notes:
> - `sum` matches `&List`. Match ergonomics bind `value: &i32` and `rest: &Box<List>`. The recursive call `sum(rest)` works because `&Box<List>` deref-coerces to `&List`. `value + sum(rest)` is `&i32 + i32`, which std supports via its reference `Add` impl.
> - `count_nodes` is double recursion. The `_` ignores the node's value; `1 + count_nodes(left) + count_nodes(right)` combines the two children's counts.
> - **If clippy fires on `value + sum(rest)`** (e.g., suggesting a dereference), change it to `*value + sum(rest)` and report the deviation. It is not expected to fire, but this is the one known fallback.

- [ ] **Step 2: Overwrite `lessons/10-smart-pointers/solutions/tests/exercise.rs`**

```rust
use smart_pointers_solutions::{List, Tree, count_nodes, sum};

// Warm-up: sum

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

// Main: count_nodes

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

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package smart-pointers-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package smart-pointers-solutions --all-targets -- -D warnings
cargo fmt --check --package smart-pointers-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires on `value + sum(rest)`, switch to `*value + sum(rest)` (the documented fallback) and report it; otherwise STOP and report any unexpected lint.

- [ ] **Step 5: Commit**

```bash
git add lessons/10-smart-pointers/solutions
git commit -m "feat(lesson-10): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/10-smart-pointers/README.md`

- [ ] **Step 1: Overwrite `lessons/10-smart-pointers/README.md`**

The complete file content (write this as the file's actual content, starting with the `# Lesson 10` heading). Code fences inside the markdown are plain triple-backticks.

```markdown
# Lesson 10 — Smart pointers

A smart pointer is a type that owns data like a normal value, but
adds behavior: heap allocation, shared ownership, or interior
mutability. The standard library has four you'll reach for
constantly — `Box`, `Rc`, `RefCell`, and `Arc`. This lesson
introduces all four and drills the most foundational, `Box`, through
recursive data structures.

## Learning goals

- Use `Box<T>` to heap-allocate a value and to define recursive types
- Explain why a recursive type needs indirection (`Box`) — infinite
  size otherwise
- Describe `Rc<T>` as shared ownership via reference counting
- Describe `RefCell<T>` as interior mutability with a runtime borrow
  check, and recognize the `Rc<RefCell<T>>` combo
- Recognize `Arc<T>` as the thread-safe `Rc`

## Self-study notes

### `Box<T>` — heap allocation and recursive types

`Box<T>` puts a value on the heap and gives you an owner of fixed,
pointer size:

​```rust
let b = Box::new(5);
println!("{}", *b);   // *b dereferences to read the value (mostly automatic)
​```

A `Box` is a single owner with the normal ownership rules — it's
dropped when it goes out of scope, freeing the heap memory.

Where `Box` becomes *necessary* is recursive types. A type that
contains itself would have infinite size:

​```rust
enum List {
    Cons(i32, Box<List>),   // Box gives a fixed-size pointer
    Nil,
}
​```

Without the `Box`, the compiler can't compute the size of `List` (it
contains a `List`, which contains a `List`...). `Box` is a fixed-size
pointer to heap memory, so it breaks the cycle. This lesson's
exercises and compile-fail center on exactly this.

### `Rc<T>` — shared ownership

When one value needs *multiple* owners in single-threaded code, use
`Rc<T>` (reference counting):

​```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);                 // a second owner
println!("{}", Rc::strong_count(&a));  // 2
​```

`Rc::clone` doesn't copy the data — it bumps a counter and hands you
another handle to the same data. The data is freed only when the last
`Rc` is dropped.

### `RefCell<T>` — interior mutability

Normally Rust requires `&mut` to mutate. `RefCell<T>` lets you mutate
through a shared `&` reference by moving the borrow check to
*runtime*:

​```rust
use std::cell::RefCell;

let cell = RefCell::new(5);
*cell.borrow_mut() += 1;          // mutate via a shared reference
println!("{}", cell.borrow());    // 6
​```

The borrowing rules still apply — but they're checked at runtime
instead of compile time. Two simultaneous `borrow_mut()`s panic.

### `Rc<RefCell<T>>` — shared mutable state

Stack them for the common "multiple owners that can all mutate"
pattern:

​```rust
let shared = Rc::new(RefCell::new(0));
let clone = Rc::clone(&shared);

*clone.borrow_mut() += 1;
println!("{}", shared.borrow());  // 1 — the mutation is visible through both handles
​```

`Rc` provides shared ownership; `RefCell` provides the mutation.
You'll see this combination in tree and graph structures where nodes
need shared, mutable access.

### `Arc<T>` — the thread-safe `Rc`

`Arc<T>` is `Rc<T>` with *atomic* reference counting, making it safe
to share across threads. The API is identical:

​```rust
use std::sync::Arc;

let a = Arc::new(42);
let b = Arc::clone(&a);
​```

The thread-safe analogue of `Rc<RefCell<T>>` is `Arc<Mutex<T>>`,
which we'll use in **Lesson 16 — Concurrency**. For single-threaded
code, prefer `Rc` — the atomic operations in `Arc` have a small cost.

## Exercises

### Warm-up: `sum`

The exercises crate ships a cons-list enum already using `Box`:

​```rust
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
​```

Implement `sum(list: &List) -> i32` that recursively sums all the
values. Match on the two variants: `Cons` holds a value and the rest
of the list; `Nil` is the empty base case (sum 0).

### Main: `count_nodes`

The exercises crate also ships a binary-tree enum:

​```rust
pub enum Tree {
    Leaf,
    Node(i32, Box<Tree>, Box<Tree>),
}
​```

Implement `count_nodes(tree: &Tree) -> usize` that counts the `Node`
variants. A `Node` contributes 1 plus the counts of its left and
right children; a `Leaf` contributes 0. This is double recursion —
one call for each child.

### Compile-fail

`exercises/compile_fails/10-infinite-size.rs` ships with a recursive
enum that does *not* use `Box`, so it has infinite size. rustc's
error literally suggests inserting a `Box`. Wrap the recursive field
in `Box<List>` to fix it.

### Run

​```bash
make verify LESSON=10-smart-pointers
​```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
```

> **IMPORTANT for the implementer:** the code-fence escape in the markdown above uses an invisible zero-width character (shown as `​```) in front of each triple-backtick block — that's only there so this plan file can nest fenced markdown inside an outer fenced markdown block. When you write the actual `README.md`, every fence must be three PLAIN backticks `` ``` `` with NO leading invisible character. After writing, `grep -c '^```' lessons/10-smart-pointers/README.md` should return 18 (9 code blocks × 2 fence lines).

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/10-smart-pointers/README.md
grep -c '^### ' lessons/10-smart-pointers/README.md
grep -c '^```' lessons/10-smart-pointers/README.md
```

Expected:
- First line: `# Lesson 10 — Smart pointers`
- `grep -c '^### '` returns 9 (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns 18 (9 code blocks × 2 fence lines)

- [ ] **Step 3: Commit**

```bash
git add lessons/10-smart-pointers/README.md
git commit -m "docs(lesson-10): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/10-smart-pointers/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/10-smart-pointers/slides/slides.md`**

The complete file content (write this as the file's actual content, starting with the `# Smart pointers` heading):

````
# Smart pointers

> A smart pointer is a type that owns data like a normal value, but adds behavior: heap allocation, shared ownership, or interior mutability. Four of them cover almost everything.

---

## Recap

Phase 2 has been about ownership and borrowing.

Smart pointers are types that **own** their data (so the ownership rules apply) but wrap it with extra capabilities. They're regular structs in the standard library — nothing magic.

---

## `Box<T>` — heap allocation

```rust
let b = Box::new(5);
println!("{}", *b);   // deref to read — mostly automatic
```

- Puts a value on the heap, gives you a pointer-size owner
- Single owner; dropped when it goes out of scope
- Same ownership rules as any value

The simplest smart pointer.

---

## `Box<T>` for recursive types

This is where `Box` becomes **necessary**:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

A `List` containing a `List` directly would have infinite size — the compiler can't lay it out.

`Box` is a fixed-size pointer to heap memory, breaking the cycle. The compile-fail exercise shows the error without it.

---

## `Rc<T>` — shared ownership

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);                 // both own it now
println!("{}", Rc::strong_count(&a));  // 2
```

- For one value with **multiple** owners (single-threaded)
- Reference-counted: dropped when the last `Rc` is dropped
- `Rc::clone` bumps a counter — it doesn't copy the data

---

## `RefCell<T>` — interior mutability

```rust
use std::cell::RefCell;

let cell = RefCell::new(5);
*cell.borrow_mut() += 1;          // mutate through a shared reference
println!("{}", cell.borrow());    // 6
```

- Normally you need `&mut` to mutate
- `RefCell` moves the borrow check to **runtime**
- Break the rules at runtime (two `borrow_mut`s at once) → panic

---

## `Rc<RefCell<T>>` — shared mutable state

```rust
let shared = Rc::new(RefCell::new(0));
let clone = Rc::clone(&shared);

*clone.borrow_mut() += 1;
println!("{}", shared.borrow());  // 1 — seen through the other handle
```

`Rc` gives shared ownership; `RefCell` gives the mutation. A common combo for tree/graph structures.

---

## `Arc<T>` — thread-safe `Rc`

`Arc` is `Rc` with **atomic** reference counting — safe to share across threads.

Same API:

```rust
use std::sync::Arc;

let a = Arc::new(42);
let b = Arc::clone(&a);
```

We'll use `Arc<Mutex<T>>` (the thread-safe analogue of `Rc<RefCell<T>>`) in **Lesson 16 — Concurrency**. For single-threaded code, prefer `Rc` — it's faster.

---

## Putting it together

Today's exercises:

- **Warm-up** a cons `List` with `sum` — single recursion through `Box`
- **Main** a binary `Tree` with `count_nodes` — double recursion

The compile-fail drives home why `Box` is mandatory for recursive types: without it, the type has infinite size.

---

## Wrap — Phase 2 nearly done

- **`Box<T>`** heap-allocates and enables recursive types
- **`Rc<T>`** gives shared ownership via reference counting
- **`RefCell<T>`** moves borrow-checking to runtime for interior mutability
- **`Rc<RefCell<T>>`** combines them for shared mutable state
- **`Arc<T>`** is the thread-safe `Rc`

Next: **Lesson 11 — Iterators & collections** (`Vec`, `HashMap`), which closes out Phase 2.
````

The OUTER fence above (quadruple backticks) is just delimiting this content in the prompt. The FILE you write should NOT contain the outer fence — only the inner triple-backtick `rust` blocks.

The file should:
- Start with `# Smart pointers` on line 1
- Have exactly 9 `---` slide separators (between 10 slides)
- Contain 6 triple-backtick `rust` code fences

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 10**

```bash
make slides-build
test -f dist/lessons/10-smart-pointers/slides/slides.md
test -f dist/lessons/10-smart-pointers/slides/index.html
grep -c "10-smart-pointers" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "10-smart-pointers"` returns at least 1.

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/10-smart-pointers/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/10-smart-pointers/slides/slides.md
git commit -m "feat(lesson-10): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `smart-pointers-solutions`), compile-fail `--expect broken` passes for lesson 10.

- [ ] **Step 2: `make verify LESSON=10-smart-pointers` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=10-smart-pointers || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "10-smart-pointers" dist/index.html
```

Expected: `dist/lessons/` contains all ten lessons. `grep -c "10-smart-pointers"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 10 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/10-smart-pointers/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/10-smart-pointers/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the `List` and `Tree` enums identically (with `Box`)
- `cargo test --package smart-pointers-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/10-smart-pointers/exercises/Cargo.toml` → 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/10-smart-pointers` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/10-smart-pointers` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/10-smart-pointers/slides/index.html`
- `dist/index.html` lists lesson 10 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/10-smart-pointers/slides/`
