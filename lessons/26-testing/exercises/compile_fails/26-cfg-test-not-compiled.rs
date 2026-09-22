// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// `#[cfg(test)]` means "compile this only when building tests". In a normal
// build the item is not merely unused - it does not exist at all, so
// anything that calls it fails to resolve. rustc reports E0425 ("cannot
// find function") and then points straight at the gate: "found an item that
// was configured out".
//
// That is the whole reason unit tests can sit beside your code without ever
// being shipped in it.
//
// The fix: `double` is called by ordinary, non-test code, so it is not
// test-only. Delete the `#[cfg(test)]` line above it.

#[cfg(test)]
fn double(x: i32) -> i32 {
    x * 2
}

pub fn quadruple(x: i32) -> i32 {
    double(double(x))
}
