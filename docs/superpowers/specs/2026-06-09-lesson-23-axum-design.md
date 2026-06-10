# Lesson 23 — HTTP services with Axum — design

The second lesson of Phase 6 (Production services). `axum` is the async
web framework built on tokio (and tower/hyper). You write **handlers**
(async functions that return something implementing `IntoResponse`), pull
request data with **extractors** (`Path`, `State`, …), and share
application data with **`State`**. The production skill at the center: a
handler is just an `async fn`, so you test it by calling it directly —
no TCP port, no running server. Builds on async (L18), traits (L12), and
adding a dependency (L14/L18/L22).

A deliberate constraint: the exercises test handlers **without binding a
port** — call the handler functions directly with constructed extractors
under `#[tokio::test]`. A complete `router()` ships too; its compiling
*proves* the handlers are valid axum handlers, and real serving
(`axum::serve` + a `TcpListener`) is shown conceptually.

## Audience and prerequisites

- Has completed Lessons 01-22
- Comfortable with `async`/`.await` + `#[tokio::test]` (L18), traits
  (L12), structs + derives (L06), and adding a dependency (L14/L18/L22)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Write an axum handler — an `async fn` that returns a type implementing
   `IntoResponse` (here, `String`) — and recognize that handlers must be
   `async` even when they do no awaiting
2. Use the `Path<T>` extractor to read a URL path segment in a handler
3. Use the `State<T>` extractor to read shared application state, and
   know that the state type must be `Clone`
4. Build a `Router` that maps method + path to handlers and supplies the
   shared state with `.with_state(...)`
5. Test handlers without a server — call them directly with constructed
   extractors under `#[tokio::test]`

## Scope

In scope: axum handlers (`async fn ... -> impl IntoResponse`, here
`String`); the `Path<T>` and `State<T>` extractors; building a `Router`
(`Router::new().route("/p/{x}", get(handler)).with_state(state)`, with
axum 0.8's `{param}` path syntax); the requirement that a `State` type be
`Clone`; testing handlers by calling them directly under `#[tokio::test]`
(no TCP). New infrastructure: add `axum = "0.8"` to the workspace
dependencies (tokio is already one). The exercises drill a `Path`-only
handler (warm-up) and a `State` + `Path` handler (main); `AppState` and a
complete `router()` are given.

Out of scope (deferred or skipped): running a real server
(`axum::serve` + `tokio::net::TcpListener`) beyond a mention; testing via
`tower::ServiceExt::oneshot` and reading response bodies; `Json<T>`
request/response bodies and serde (Lesson 29 covers serde); the `Query`
extractor beyond a mention; custom extractors and `FromRequest`;
middleware / tower layers; status codes and error responses
(`IntoResponse` for errors) beyond a mention; `Arc`-wrapping large state;
nested/merged routers; WebSockets. HTTP services are introduced as
*handlers + extractors + State + Router, tested directly*; serving,
bodies, and middleware are out of band.

## Slide arc (10 slides)

1. **Title — HTTP services with Axum.** Hook: *"An HTTP service maps
   requests to responses. axum lets you write each endpoint as an
   ordinary async function — it pulls what you need out of the request
   for you, and routes each request to the right handler."*
2. **What is axum.** A modern async web framework built on tokio,
   tower, and hyper. You describe your endpoints as **handler**
   functions and wire them into a **router**; axum does the networking,
   parsing, and dispatch. It's the most popular Rust web framework.
3. **Handlers.**
   ```rust
   async fn greet() -> String {
       "Hello!".to_string()
   }
   ```
   A handler is an `async fn` whose return type implements
   `IntoResponse` — `String`, `(StatusCode, String)`, `Json<T>`, and many
   more. Handlers are always `async` (axum requires it), even one like
   this that never awaits.
4. **The `Router`.**
   ```rust
   use axum::{Router, routing::get};

   let app = Router::new()
       .route("/greet/{name}", get(greet));
   ```
   A `Router` maps a method + path to a handler. `get(greet)` says "on a
   GET to this path, call `greet`". `{name}` is a path parameter (axum
   0.8 syntax).
5. **Extractors.** A handler's parameters are **extractors** — each pulls
   a piece of the request:
   - `Path<T>` — segments from the URL path
   - `Query<T>` — the query string
   - `State<T>` — shared application state
   - `Json<T>` — a JSON request body

   axum builds them from the request before calling your handler.
6. **The `Path` extractor.**
   ```rust
   use axum::extract::Path;

   async fn greet(Path(name): Path<String>) -> String {
       format!("Hello, {name}!")
   }
   ```
   With the route `/greet/{name}`, a request to `/greet/Alice` extracts
   `name = "Alice"`. The pattern `Path(name)` destructures the extractor
   to get the inner value.
7. **`State` — shared data.**
   ```rust
   use axum::extract::State;

   #[derive(Clone)]
   struct AppState { greeting: String }

   async fn hello(State(state): State<AppState>, Path(name): Path<String>) -> String {
       format!("{}, {name}!", state.greeting)
   }
   // ...Router::new().route(...).with_state(AppState { greeting: ... })
   ```
   `State<T>` carries shared application data (config, a database pool,
   …). You attach it with `.with_state(...)`, and **the state type must
   be `Clone`** — axum clones it per request.
8. **Testing handlers.** A handler is just an `async fn`, so you can test
   it directly — construct the extractors and call it:
   ```rust
   #[tokio::test]
   async fn greets() {
       assert_eq!(greet(Path("Alice".to_string())).await, "Hello, Alice!");
   }
   ```
   No TCP, no running server. (To actually serve it you'd bind a
   listener: `axum::serve(TcpListener::bind(addr).await?, app).await?`.)
9. **Putting it together.** Walk through the exercises: `greet` reads a
   `Path<String>` and returns a greeting (warm-up); `greet_with_state`
   adds `State<AppState>` to use the shared greeting (main). `AppState`
   and a complete `router()` (wiring both with `get` + `.with_state`) are
   given — `router` compiling is proof the handlers are valid axum
   handlers. The compile-fail forgets `#[derive(Clone)]` on the state.
10. **Wrap — services in Rust.** Five takeaways: a handler is an
    `async fn` returning `impl IntoResponse`; a `Router` maps method+path
    to handlers; extractors (`Path`, `State`, …) pull request data; the
    `State` type must be `Clone`; test handlers by calling them directly.
    Next: **Lesson 24 — Persistence** (`sqlx`, migrations, transactions).

## Exercise spec

`lessons/23-axum/` follows the standard four-part lesson shape, plus
dependencies in each crate's `Cargo.toml`:

```
23-axum/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml          # adds axum + tokio (workspace deps)
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/23-state-not-clone.rs
└── solutions/
    ├── Cargo.toml          # adds axum + tokio (workspace deps)
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `axum-exercises` and `axum-solutions` (the lesson's
"bare" name is `axum`; the import idents are `axum_exercises` /
`axum_solutions`). This matches the build-index master registry slug
`axum`, so the landing page links it without any change.

### Cargo.toml dependencies

`axum` is **not** yet a workspace dependency. The plan adds
`axum = "0.8"` to the root `[workspace.dependencies]` (resolves to 0.8.x —
verified 0.8.9 during design). `tokio` is already a workspace dependency
(from L18, `features = ["rt", "macros"]`), which is enough for
`#[tokio::test]`. Both lesson crates add, after `[lints]`:

```toml
[dependencies]
axum = { workspace = true }
tokio = { workspace = true }
```

`Cargo.lock` is gitignored (CI regenerates it); CI fetches axum's
dependency tree from crates.io (the cache key hashes `**/Cargo.toml`, so
it refetches cleanly).

### Exercise stub (`exercises/src/lib.rs`)

`AppState` (a `Clone` struct) and the complete `router()` ship as given;
the two handlers ship with `todo!()` bodies. Each handler carries
`#[allow(clippy::unused_async)]` with an explanatory comment: axum
requires handlers to be `async`, but these do no awaiting, which would
otherwise trip clippy's `unused_async` — and this allow is needed in the
*solution* too, since the finished handlers also never await (real axum
handlers commonly don't). The crate and tests compile; the tests fail at
runtime with the `todo!()` panic.

```rust
//! Lesson 23 — exercises.
//!
//! Implement `greet` (warm-up) and `greet_with_state` (main) so that
//! `cargo test --manifest-path lessons/23-axum/exercises/Cargo.toml`
//! passes. `AppState` and `router` are given. The tests live in
//! `tests/exercise.rs`.

use axum::Router;
use axum::extract::{Path, State};
use axum::routing::get;

/// Shared application state. Must be `Clone` for axum's `State`.
#[derive(Clone)]
pub struct AppState {
    pub greeting: String,
}

// axum requires handlers to be `async`, even when they do no awaiting —
// which is why this `#[allow]` is here. Your finished handler still won't
// await, so leave it.
#[allow(clippy::unused_async)]
pub async fn greet(Path(_name): Path<String>) -> String {
    todo!("return \"Hello, <name>!\"")
}

#[allow(clippy::unused_async)]
pub async fn greet_with_state(State(_state): State<AppState>, Path(_name): Path<String>) -> String {
    todo!("return \"<greeting>, <name>!\" using the state's greeting")
}

/// Build the router wiring both handlers, with shared state.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/greet/{name}", get(greet))
        .route("/hello/{name}", get(greet_with_state))
        .with_state(state)
}
```

### Warm-up: `greet`

Reference solution:

```rust
#[allow(clippy::unused_async)]
pub async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}
```

Pedagogical packing: the simplest axum handler — an `async fn` taking one
extractor and returning a `String` (which implements `IntoResponse`). The
`Path(name): Path<String>` parameter destructures the `Path` extractor to
the path segment. The `#[allow(clippy::unused_async)]` is honest: axum's
`Handler` trait is implemented for `async` functions, so the handler must
be `async`, but it never awaits — `unused_async` (pedantic) would
otherwise reject it. No `#[must_use]` (an `async fn` returns a
`#[must_use]` `Future` already).

Four tests (`#[tokio::test]`, calling the handler directly):

```rust
#[tokio::test]
async fn warmup_alice() {
    assert_eq!(greet(Path("Alice".to_string())).await, "Hello, Alice!");
}

#[tokio::test]
async fn warmup_bob() {
    assert_eq!(greet(Path("Bob".to_string())).await, "Hello, Bob!");
}

#[tokio::test]
async fn warmup_empty() {
    assert_eq!(greet(Path(String::new())).await, "Hello, !");
}

#[tokio::test]
async fn warmup_unicode() {
    assert_eq!(greet(Path("Zoe".to_string())).await, "Hello, Zoe!");
}
```

### Main: `greet_with_state`

Reference solution:

```rust
#[allow(clippy::unused_async)]
pub async fn greet_with_state(State(state): State<AppState>, Path(name): Path<String>) -> String {
    format!("{}, {name}!", state.greeting)
}
```

Pedagogical packing: a handler with *two* extractors — `State<AppState>`
for the shared greeting and `Path<String>` for the name. It reads
`state.greeting` and formats it with the name. This is the payoff of
`State`: configuration/shared data made available to every handler. The
`AppState` type derives `Clone` (required by axum). Same
`#[allow(clippy::unused_async)]` and no `#[must_use]` as the warm-up.

Four tests (build `State(AppState { ... })` + `Path(...)` directly; one
builds the `router`):

```rust
#[tokio::test]
async fn main_hi_bob() {
    let state = AppState {
        greeting: "Hi".to_string(),
    };
    assert_eq!(
        greet_with_state(State(state), Path("Bob".to_string())).await,
        "Hi, Bob!"
    );
}

#[tokio::test]
async fn main_welcome_sam() {
    let state = AppState {
        greeting: "Welcome".to_string(),
    };
    assert_eq!(
        greet_with_state(State(state), Path("Sam".to_string())).await,
        "Welcome, Sam!"
    );
}

#[tokio::test]
async fn main_hey_empty() {
    let state = AppState {
        greeting: "Hey".to_string(),
    };
    assert_eq!(
        greet_with_state(State(state), Path(String::new())).await,
        "Hey, !"
    );
}

#[tokio::test]
async fn main_router_builds() {
    // `router` wires the handlers into a Router; building it proves they
    // are valid axum handlers.
    let _router = router(AppState {
        greeting: "Hello".to_string(),
    });
}
```

**Eight tests total** (four warm-up + four main), all `#[tokio::test]`.
The handlers are called directly with constructed extractors — no TCP, no
server — so the tests are deterministic. The `main_router_builds` test
exercises the given `router()` and confirms the handlers wire up.

### Compile-fail: `23-state-not-clone.rs`

Path: `exercises/compile_fails/23-state-not-clone.rs`. A self-contained,
std-only file (the `compile-fails` tool type-checks with `rustc
--crate-type=lib --emit=metadata`, which can't link `axum`). It mirrors
axum's `Router::with_state` — a generic function with a `Clone` bound —
and passes a state type that forgot `#[derive(Clone)]`.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// axum's `Router::with_state(state)` requires the state type to be
// `Clone` — axum clones it for each request. If you forget
// `#[derive(Clone)]` on your state struct, the compiler rejects it with:
// "the trait bound `AppState: Clone` is not satisfied".
//
// This file reproduces that exact error with a plain generic function
// (`with_state` here stands in for axum's, requiring `S: Clone`). The
// `AppState` struct below is missing its `Clone` derive, so the call
// fails the bound.
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
```

Pass condition: the student adds `#[derive(Clone)]` to `AppState`. rustc
reports E0277 "the trait bound `AppState: Clone` is not satisfied" —
verified during design; this is byte-for-byte the error real axum code
produces. After the derive the file type-checks.

This is the lesson's centerpiece for `State`: shared state must be
`Clone`, and the compiler's message points straight at the missing
derive — the single most common axum beginner error.

## README structure

`lessons/23-axum/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - Handlers and `IntoResponse`
  - The `Router` and routes
  - Extractors — `Path`
  - `State` — shared data (must be `Clone`)
  - Testing handlers (and serving)
- **Exercises** — four subsections: Warm-up (`greet`), Main
  (`greet_with_state`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`State`" and "Testing handlers" sections are the heaviest — they carry
the shared-state idea and the testable-handler skill.

## Lint expectations

Lesson 23's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without unexpected `#[allow]` attributes —
verified during design:

- Both handlers carry `#[allow(clippy::unused_async)]` — axum's `Handler`
  trait requires `async fn`s, but these never await, which `unused_async`
  (pedantic) would otherwise reject. This is the one documented allow,
  present in both the stub and the solution (real axum handlers commonly
  don't await).
- **No `#[must_use]` anywhere**: the async handlers return `#[must_use]`
  `Future`s, and `router` returns a `Router` (already `#[must_use]`), so
  adding the attribute would trip `clippy::double_must_use` (verified).
- `AppState` derives `Clone` (required by axum's `State`).
- The route paths use axum 0.8 syntax (`/greet/{name}`), not the old
  `:name` form.
- The **exercise stub** keeps all imports used (by the handler signatures
  and `router`), so no unused-import warnings; the `todo!()` bodies lint
  clean (verified).

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/23-axum/` exists with the four-part structure
- Root `Cargo.toml` `[workspace.dependencies]` includes `axum = "0.8"`;
  both lesson `Cargo.toml`s declare `axum = { workspace = true }` and
  `tokio = { workspace = true }`
- Cargo manifests use the correct package names (`axum-exercises`,
  `axum-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `AppState`, `greet` / `greet_with_state` signatures, and `router`; the
  exercise ships `todo!()` handler bodies, the solution ships real bodies
- `cargo test --package axum-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/23-axum/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/23-axum`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/23-axum`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/23-axum/slides/index.html`
- `dist/index.html` lists lesson 23 as a clickable link (registry slug
  `axum` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/23-axum/slides/` returns 200

## Open questions

None.
