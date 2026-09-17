# Lesson 23 — HTTP services with Axum

An HTTP service maps requests to responses. `axum` lets you write each
endpoint as an ordinary `async fn`: it pulls what you need out of the
request and routes each request to the right handler. The production
skill: because a handler is just a function, you test it by calling it —
no port, no running server.

## Learning goals

- Write an axum handler — an `async fn` returning a type that implements
  `IntoResponse` (here, `String`) — and know that a handler must return a
  future, so it's `async` even when it never awaits
- Read a URL path segment with the `Path<T>` extractor
- Read shared application state with the `State<T>` extractor, and know
  that the state type must be `Clone`
- Build a `Router` that maps method + path to handlers and supplies the
  shared state with `.with_state(...)`
- Test handlers without a server — call them directly with constructed
  extractors under `#[tokio::test]`

## Self-study notes

### Handlers and `IntoResponse`

A handler is an `async fn` whose return type implements `IntoResponse`:

```rust
async fn hello() -> String {
    "Hello!".to_string()
}
```

`String` becomes a `200 OK` response with a plain-text body. Many types
implement `IntoResponse` — `&'static str`, `(StatusCode, String)` to set
the status too, `Json<T>` for JSON bodies. axum needs a handler to return
a *future*, and an `async fn` is how you write one — so handlers are
`async` even when, like this one, they never await.

### The `Router` and routes

```rust
use axum::{Router, routing::get};

let app: Router = Router::new()
    .route("/", get(hello))
    .route("/greet/{name}", get(greet));
```

A `Router` maps a method + path to a handler: `get(greet)` means "on a
GET to this path, call `greet`" (`post`, `put`, and `delete` work the
same way). `{name}` is a path parameter. That braces syntax is new in
axum 0.8 — older examples online use `:name`, which 0.8 rejects: the code
compiles, but panics as soon as the route is added. The `: Router`
annotation names the finished, ready-to-serve router type — the same one
this lesson's `router()` function returns.

### Extractors — `Path`

A handler's parameters are *extractors*: each one pulls a piece of the
request. `Path<T>` reads segments from the URL path:

```rust
use axum::extract::Path;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}
```

With the route `/greet/{name}`, a request to `/greet/Alice` calls `greet`
with `name = "Alice"`. `Path` is a *tuple struct* — a struct whose one
field has a position instead of a name (`pub struct Path<T>(pub T);`).
Function parameters accept patterns just like `let` (you've used
`|(_word, count)|` in Lesson 11's closures), so `Path(name): Path<String>`
binds the inner `String` to `name`. Other extractors work the same way —
`Query<T>` for the query string, `State<T>` for shared state, `Json<T>`
for a JSON body. If extraction fails, axum rejects the request with an
error response and never calls your handler.

### `State` — shared data (must be `Clone`)

```rust
use axum::extract::State;

#[derive(Clone)]
struct AppState {
    greeting: String,
}

async fn greet_with_state(State(state): State<AppState>, Path(name): Path<String>) -> String {
    format!("{}, {name}!", state.greeting)
}

let app: Router = Router::new()
    .route("/hello/{name}", get(greet_with_state))
    .with_state(AppState { greeting: "Hi".to_string() });
```

`State<T>` gives handlers shared application data — configuration, a
database pool (Lesson 24), a cache. You attach it once with
`.with_state(...)`, and every handler that asks for `State<AppState>`
receives it. The state type **must be `Clone`**: axum clones it for each
request. Forget `#[derive(Clone)]` and the build fails — but the *first*
error is usually a hard-to-read "`Handler<_, _>` is not satisfied" at
`get(...)`. Scroll down: the next errors say "the trait bound
`AppState: Clone` is not satisfied" (E0277), and rustc suggests adding
`#[derive(Clone)]`.

### Testing handlers (and serving)

A handler is just an `async fn`, so you can test it like one: construct
the extractors yourself, call it, and `.await` the result.

```rust
#[tokio::test]
async fn greets() {
    assert_eq!(greet(Path("Alice".to_string())).await, "Hello, Alice!");
}
```

`Path(...)` and `State(...)` are tuple structs with public fields, so
building them by hand is easy. No TCP port, no running server, no HTTP
client — the test is fast and deterministic. What it doesn't check is
the routing itself (that `/greet/{name}` reaches `greet`). To actually
serve an app you'd bind a listener and hand it to axum —
`axum::serve(TcpListener::bind(addr).await?, app).await?` — which is
beyond this lesson.

## Exercises

### Warm-up: `greet`

Implement `greet` so it returns `"Hello, {name}!"` for the name in the
path:

```rust
pub async fn greet(Path(_name): Path<String>) -> String {
    // rename `_name` to `name`, then: format!("Hello, {name}!")
    todo!("return \"Hello, <name>!\"")
}
```

The stub names the binding `_name`: this course's lints make an unused
variable a compile *error*, and the `_` prefix keeps the unfinished stub
building. Rename it to `name` in the same edit where you replace
`todo!()` — rename it first and the build fails.

### Main: `greet_with_state`

Implement `greet_with_state` so it returns `"{greeting}, {name}!"`, using
the greeting from the shared state:

```rust
pub async fn greet_with_state(State(_state): State<AppState>, Path(_name): Path<String>) -> String {
    // rename `_state`/`_name`, then: format!("{}, {name}!", state.greeting)
    todo!("return \"<greeting>, <name>!\" using the state's greeting")
}
```

With `AppState { greeting: "Hi".to_string() }` and the path `Bob`, it
returns `"Hi, Bob!"`. As in the warm-up, rename the stub's `_state` and
`_name` when you write the body.

`AppState` and `router` are given — read `router` to see both handlers
wired with `get` and `.with_state`. Because `router` lives in
`src/lib.rs`, the crate only compiles while your handlers are valid axum
handlers, so `main_router_builds` passes before you start. If you change
a handler's signature (drop `async`, say), the build fails *inside
`router`* with "`Handler<_, _>` is not satisfied": fix the handler, not
`router`.

### Compile-fail

`exercises/compile_fails/23-state-not-clone.rs` passes a state struct
that forgot `#[derive(Clone)]` to a `with_state` function requiring
`S: Clone` — a std-only stand-in for axum's `Router`. The compiler
rejects it (E0277). Fix it by adding `#[derive(Clone)]` above
`struct AppState`.

### Run

```bash
make verify LESSON=23-axum
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
