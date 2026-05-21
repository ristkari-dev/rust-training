//! Lesson 01 — reference solution.

#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
