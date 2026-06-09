// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// A command-line program signals success or failure to the shell through
// its EXIT CODE. In Rust you return one from `main` as a
// `std::process::ExitCode` — not as a bare integer. `0` is an integer
// literal, not an `ExitCode`, so the compiler rejects returning it from a
// function declared `-> ExitCode`.
//
// rustc reports E0308: "mismatched types: expected `ExitCode`, found
// integer".
//
// The fix: return an actual `ExitCode`. `ExitCode::SUCCESS` is the
// success code (0); `ExitCode::FAILURE` is a generic failure; or
// `ExitCode::from(2)` for a specific code.
//
// Hint: change `0` to `ExitCode::SUCCESS`.

use std::process::ExitCode;

fn main() -> ExitCode {
    0
}
