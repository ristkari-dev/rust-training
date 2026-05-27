//! Lesson 08 — reference solutions.

#[must_use]
pub fn wrap_in_quotes(s: &str) -> String {
    let mut result = String::from("\"");
    result.push_str(s);
    result.push('"');
    result
}

// clippy::ptr_arg would suggest &mut str, but we need &mut String to
// call push_str and push on the heap-owning type. The whole lesson is
// about borrowing the owned String, so the &mut String signature IS
// the point.
#[allow(clippy::ptr_arg)]
pub fn merge_into(target: &mut String, parts: &[&str], separator: &str) {
    let mut first = true;
    for part in parts {
        if !first {
            target.push_str(separator);
        }
        target.push_str(part);
        first = false;
    }
}
