//! Lesson 04 — reference solutions.

#[must_use]
pub fn divmod(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}

#[must_use]
pub fn join_with_dashes(words: &[&str]) -> String {
    let mut out = String::new();
    let mut first = true;
    for w in words {
        if !first {
            out.push('-');
        }
        out.push_str(w);
        first = false;
    }
    out
}
