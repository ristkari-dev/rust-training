//! Lesson 02 — reference solutions.

const HUNDRED: f64 = 100.0;

#[must_use]
#[allow(clippy::let_and_return)] // pedagogical: the let+annotation pattern is the point
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // Shadowing the parameter binding — not mutation.
    let f = f - 32.0;
    // Annotation is redundant (inference works) but spells out the type.
    let scaled: f64 = f * 5.0 / 9.0;
    scaled
}

#[must_use]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)] // u32 -> i32 for powi: safe for realistic year counts
pub fn compound_interest(principal: f64, rate_percent: f64, years: u32) -> f64 {
    let rate = rate_percent / HUNDRED;
    let factor: f64 = 1.0 + rate;
    principal * factor.powi(years as i32)
}
