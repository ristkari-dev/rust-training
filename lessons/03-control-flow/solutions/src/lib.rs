//! Lesson 03 — reference solutions.

#[must_use]
// The lesson teaches `if` as an expression. Clippy's `comparison_chain`
// lint would suggest rewriting this with `match n.cmp(&0)`, but that
// would defeat the pedagogical point — students need to see the
// if/else-if/else chain used as the function's tail expression.
#[allow(clippy::comparison_chain)]
pub fn classify(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}

#[must_use]
pub fn count_digits(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut remaining = n;
    let mut count: u32 = 0;
    while remaining > 0 {
        remaining /= 10;
        count += 1;
    }
    count
}
