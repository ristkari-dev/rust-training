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
