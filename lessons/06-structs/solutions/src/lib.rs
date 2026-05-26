//! Lesson 06 — reference solutions.

#[derive(Debug)]
pub struct Counter {
    count: u32,
}

impl Counter {
    #[must_use]
    #[allow(clippy::new_without_default)] // Default trait is Lesson 12
    pub fn new() -> Self {
        Counter { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    #[must_use]
    pub fn value(&self) -> u32 {
        self.count
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    #[must_use]
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    #[must_use]
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
