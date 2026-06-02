// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Calling an `async fn` does NOT run it — it returns a `Future`, a lazy
// value that produces the result only once you `.await` it. So
// `double(21)` is a `Future`, not an `i32`. Binding it to `let doubled:
// i32` is a type mismatch: you asked for an `i32` but got a future.
//
// rustc reports E0308 ("mismatched types: expected `i32`, found future")
// and even suggests the fix: `.await` the future.
//
// The fix: `.await` the call so the future runs and yields its `i32`.
// (`.await` is allowed here because `run` is itself `async`.)
//
// Hint: change `double(21)` to `double(21).await`.

async fn double(n: i32) -> i32 {
    n * 2
}

async fn run() -> i32 {
    let doubled: i32 = double(21);
    doubled
}

fn main() {
    // `run()` returns a future; the type error inside `run`'s body is a
    // compile error whether or not we ever run it.
    let _ = run();
}
