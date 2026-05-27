//! Lesson 09 — exercises.
//!
//! Implement `longest` (warm-up) and the `Excerpt` struct's methods
//! (main) so that `cargo test --manifest-path
//! lessons/09-lifetimes/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn longest<'a>(_a: &'a str, _b: &'a str) -> &'a str {
    todo!("return whichever of a and b has the greater (or equal) length")
}

pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    #[must_use]
    pub fn new(_text: &'a str) -> Self {
        todo!("construct an Excerpt holding text")
    }

    #[must_use]
    pub fn length(&self) -> usize {
        todo!("return the length of the held text")
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        todo!("return whether the held text is empty")
    }
}
