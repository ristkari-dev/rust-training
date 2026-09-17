// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// axum's `Router` requires its state type to be `Clone` — axum clones the
// state for each request. If you forget `#[derive(Clone)]` on your state
// struct, the build fails with E0277: "the trait bound `AppState: Clone`
// is not satisfied" (usually listed right after a harder-to-read
// "`...: Handler<_, _>` is not satisfied" error on `get(...)`).
//
// This file reproduces the `AppState: Clone` error with a plain generic
// function (`with_state` here stands in for axum's `Router`, requiring
// `S: Clone`). The `AppState` struct below is missing its `Clone` derive,
// so the call fails the bound.
//
// The fix: derive `Clone` on the state type.
//
// Hint: add `#[derive(Clone)]` above `struct AppState`.

fn with_state<S: Clone>(state: S) -> S {
    state.clone()
}

struct AppState {
    greeting: String,
}

fn main() {
    let _ = with_state(AppState {
        greeting: "Hi".to_string(),
    });
}
