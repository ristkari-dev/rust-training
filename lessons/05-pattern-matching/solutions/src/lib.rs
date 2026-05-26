//! Lesson 05 — reference solutions.

#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

#[must_use]
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b),
    }
}

#[must_use]
pub fn next(light: Light) -> Light {
    match light {
        Light::Red => Light::Green,
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}
