//! Lesson 06 — exercises.
//!
//! Fill in the `todo!()` method bodies so that
//! `cargo test --manifest-path lessons/06-structs/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

#[derive(Debug)]
#[allow(dead_code)]
pub struct Counter {
    count: u32,
}

impl Counter {
    #[must_use]
    #[allow(clippy::new_without_default)] // Default trait is Lesson 12
    pub fn new() -> Self {
        todo!("return a Counter with count = 0")
    }

    pub fn increment(&mut self) {
        todo!("add 1 to self.count")
    }

    #[must_use]
    pub fn value(&self) -> u32 {
        todo!("return self.count")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    #[must_use]
    pub fn new(_width: u32, _height: u32) -> Self {
        todo!("build a Rectangle with the given fields")
    }

    #[must_use]
    pub fn area(&self) -> u32 {
        todo!("return width * height")
    }

    #[must_use]
    pub fn is_square(&self) -> bool {
        todo!("return whether width == height")
    }
}
