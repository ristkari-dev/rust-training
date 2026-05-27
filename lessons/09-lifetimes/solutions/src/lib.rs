//! Lesson 09 — reference solutions.

#[must_use]
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    #[must_use]
    pub fn new(text: &'a str) -> Self {
        Excerpt { text }
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.text.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}
