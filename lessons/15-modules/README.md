# Lesson 15 — Modules, crates, workspaces

As a program grows, you split it into modules — namespaces that group
related code and control what's visible. Modules map to files, files
group into a crate, and crates group into a workspace. This lesson is
hands-on with a real multi-file module tree, and it closes Phase 3.

## Learning goals

- Declare modules inline (`mod foo { ... }`) and as files (`mod foo;` →
  `foo.rs`, `pub mod bar;` → `foo/bar.rs`)
- Control visibility with `pub` — items are private by default, and the
  whole path must be public to reach an item from outside
- Refer to items by path (`crate::`, `self::`, `super::`) and bring
  names into scope with `use`
- Flatten a public API with `pub use` re-exports
- Explain the crate (a compilation unit) and workspace (a group of
  crates) picture

## Self-study notes

### Modules and the file mapping

A module is a named container for code. Declare it inline with braces,
or point at a file with `mod name;`:

```rust
mod geometry;        // in lib.rs: loads geometry.rs
```

```text
src/
├── lib.rs            // mod geometry;
├── geometry.rs       // pub mod shapes;
└── geometry/
    └── shapes.rs     // pub fn rectangle_area(...)
```

`mod geometry;` loads `geometry.rs`; `pub mod shapes;` inside it loads
`geometry/shapes.rs`. The module path mirrors the directory path. (This
is the modern form; older code used `mod.rs` files.)

### Visibility — `pub` and private-by-default

Items are **private by default** — visible only within their module and
its descendants. Add `pub` to expose them:

```rust
mod geometry {
    pub fn area() -> u32 { 0 }   // reachable from outside
    fn helper() {}               // private to `geometry`
}
```

The catch: the *whole path* must be public. A `pub fn` inside a private
`mod` is still unreachable from outside — the module needs `pub` too.

### Paths — `crate`, `self`, `super`, `use`

Refer to an item by its path. Absolute paths start at `crate`; relative
paths use `self` (here) and `super` (parent):

```rust
crate::geometry::shapes::rectangle_area(3, 4);
self::shapes::rectangle_area(3, 4);
use crate::geometry::shapes::rectangle_area;   // shortcut
rectangle_area(3, 4);                           // now callable by short name
```

`use` brings a name into scope so you can call it without the full path.

### Re-exports — `pub use`

`pub use` re-exports an item so callers can reach it by a short
crate-root path, even though it lives deep in the tree:

```rust
// in lib.rs:
pub use geometry::shapes::rectangle_area;
// callers can now write `my_crate::rectangle_area`
```

This is how libraries present a flat, friendly public API while keeping
their internals organized.

### Crates & workspaces

A **crate** is a compilation unit — a library (`lib.rs`) or a binary
(`main.rs`). A **workspace** groups several crates that share one
`Cargo.lock` and one `target/` directory. This course is itself a
workspace: `tools/*`, `lessons/*/exercises`, and `lessons/*/solutions`
are all member crates under the root `Cargo.toml`.

## Exercises

### Warm-up: `rectangle_area`

The exercise crate ships a real module tree. In
`src/geometry/shapes.rs`, implement the leaf function:

```rust
pub fn rectangle_area(width: u32, height: u32) -> u32 {
    // width * height
    todo!()
}
```

It's re-exported at the crate root, so tests reach it by both
`modules_exercises::rectangle_area` and the full
`modules_exercises::geometry::shapes::rectangle_area` path.

### Main: `total_area`

In `src/geometry.rs` (the parent module), implement `total_area`, which
calls the child module's function via a path and sums the results:

```rust
pub fn total_area(rects: &[(u32, u32)]) -> u32 {
    // rects.iter().map(|&(w, h)| shapes::rectangle_area(w, h)).sum()
    todo!()
}
```

Note the `shapes::rectangle_area` path — the parent reaching into its
child module.

### Compile-fail

`exercises/compile_fails/15-private-fn.rs` calls a function that has no
`pub`, from outside its module — the compiler rejects it (E0603 —
the function is private). Fix it by adding `pub` to the function.

### Run

```bash
make verify LESSON=15-modules
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/` for the reference implementations. Try the
exercises before peeking.
