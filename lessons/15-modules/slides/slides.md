# Modules, crates, workspaces

> As a program grows, you split it into modules — namespaces that group related code and control what's visible. Modules map to files, files group into a crate, and crates group into a workspace.

---

## Why modules

They organize code into namespaces, control visibility (public API vs internal), and prevent name clashes.

Everything you've written so far lived in one `lib.rs`. Real crates split into many modules.

---

## Defining modules

```rust
mod geometry {            // inline module
    pub fn area() -> u32 { 0 }
}

// or, in its own file:
mod geometry;             // looks for geometry.rs
```

A module is a named container. Write it inline with braces, or point at a file with `mod name;`.

---

## The file mapping

```text
src/
├── lib.rs            // mod geometry;
├── geometry.rs       // pub mod shapes;
└── geometry/
    └── shapes.rs     // pub fn rectangle_area(...)
```

`mod geometry;` loads `geometry.rs`. `pub mod shapes;` inside it loads `geometry/shapes.rs`. The module path mirrors the directory path.

---

## Visibility — `pub`

Items are **private by default** — visible only within their module and its descendants. Add `pub` to expose them.

The catch: the *whole path* must be public. A `pub fn` inside a private `mod` is still unreachable — both the module and the function need `pub`.

---

## Paths — `crate`, `self`, `super`, `use`

```rust
crate::geometry::shapes::rectangle_area(3, 4);  // from the crate root
self::shapes::rectangle_area(3, 4);             // relative to here
super::sibling_fn();                            // parent module
use crate::geometry::shapes::rectangle_area;    // bring it into scope
```

Absolute paths start at `crate`; relative paths use `self`/`super`. `use` creates a shortcut to call by the short name.

---

## Re-exports — `pub use`

```rust
// in lib.rs:
pub use geometry::shapes::rectangle_area;
```

`pub use` re-exports an item, so callers reach it by a short crate-root path (`my_crate::rectangle_area`) even though it lives deep in the tree. This is how libraries present a flat public API.

---

## Crates & workspaces

A **crate** is a compilation unit — a library (`lib.rs`) or a binary (`main.rs`).

A **workspace** groups several crates that share one `Cargo.lock` and one `target/`. *This course is a workspace*: `tools/*`, `lessons/*/exercises`, and `lessons/*/solutions` are all member crates.

---

## Putting it together

Today's exercise is a real module tree:

- **Warm-up** `rectangle_area` — a leaf function in `geometry/shapes.rs`
- **Main** `total_area` in `geometry.rs`, calling `shapes::rectangle_area` via a path

`lib.rs` re-exports both with `pub use`. The compile-fail shows a private function called from outside its module.

---

## Wrap — Phase 3 complete

- `mod` declares a module, inline or as a file
- The module tree mirrors the file tree
- Items are private until `pub`, and the whole path must be public
- Paths (`crate`/`self`/`super`) and `use` navigate the tree
- `pub use` flattens the public API; crates group into workspaces

Next: **Phase 4 — Lesson 16, Threads & channels** (concurrency).
