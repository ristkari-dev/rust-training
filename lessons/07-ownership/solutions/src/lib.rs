//! Lesson 07 — reference solutions.

#[must_use]
pub fn append_excl(mut s: String) -> String {
    s.push('!');
    s
}

#[must_use]
pub fn swap_and_join(a: String, b: String) -> String {
    let mut result = b;
    result.push(' ');
    result.push_str(&a);
    result
}
