# Lesson 25 — Observability

A service you cannot see into is a service you cannot operate. The question
is never "did it log something?" — it is "can I find the one request that
failed, among a million that didn't?" `tracing` turns what your code did
into structured events you can filter, and counters turn what it did a lot
of into numbers you can graph. The production skill: **a log line is data,
not prose**.

## Learning goals

- Emit a structured event with `tracing::info!`, keeping the message
  constant and putting the values in named fields
- Explain why `info!(path, status, "request finished")` is worth more than
  `info!("request {path} finished with {status}")`
- Test what your code logs, with a subscriber scoped to one closure rather
  than a global one
- Count events with `AtomicU64` counters behind a shared `&self`, and know
  why a counter needs no `Mutex`
- Tell a log from a metric, and read Prometheus' text exposition format

## Self-study notes

### Events, and why fields beat interpolation

An event is one record of something that happened. Write the message as a
constant and put the values beside it, as named fields:

```rust
use tracing::info;

info!(path, status, elapsed_ms, "request finished");
// as testkit::capture records it (a plain .json() subscriber also emits
// a "timestamp"):
// {"level":"INFO","fields":{"message":"request finished","path":"/health",
//  "status":200,"elapsed_ms":3},"target":"observability_exercises"}
```

`info!(path, ...)` is shorthand for `info!(path = path, ...)`, and the
message always comes **last** — fields first, message at the end. Put it
first and rustc offers to add `{}` placeholders for your fields; that
suggestion is the anti-pattern this lesson exists to prevent. The
interpolated version — `info!("request {path} finished with {status}")`
— reads just as well and throws the structure away: you can grep it, but
you cannot ask "how many requests returned 500 on `/orders` last hour?"
without parsing English back into data. Fields are what make that question
answerable.

### Levels and spans

Every event has a level: `error!`, `warn!`, `info!`, `debug!`, `trace!`.
Levels are the reader's filter, not a mood — `error!` means someone should
look, `info!` is the story of what happened.

An event is a point in time; a **span** is an interval with a name and
fields, and everything logged inside it inherits them:

```rust
#[tracing::instrument]
fn handle(path: &str) {
    info!("started");   // printed inside handle{path=…}, because the span carries it
}
```

`#[instrument]` opens a span for the whole function and records its
arguments as fields. This lesson doesn't exercise spans; add
`#[tracing::instrument]` to a function of your own and the subscriber starts
printing `handle{path=…}` around everything it logs.

### Subscribers — what records, and where

Nothing is recorded until something subscribes. Without a subscriber the
macros are no-ops, which is why a library can log freely and leave the
decision to the binary:

```rust
fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // the default is INFO
        .init();                               // human-readable lines
    // tracing_subscriber::fmt().json().init(); // one JSON object per event
}
```

The subscriber decides the format, the destination and the minimum level. It
is also where you get other crates' telemetry for free: install one, and
every instrumented dependency starts talking. Lesson 24's database layer,
unchanged, emits lines like `DEBUG sqlx::query: summary="INSERT INTO
accounts (name, …" rows_affected=1 elapsed=52.041µs …` (abridged —
sqlx also records `db.statement`, `rows_returned` and `elapsed_secs`). You
wrote none of that. Note the level: sqlx logs queries at DEBUG, so the
default INFO subscriber hides them — raise the level as above, or turn on
`tracing-subscriber`'s `env-filter` feature and filter with `EnvFilter` and
`RUST_LOG=sqlx=debug`.

### Testing what you log

If logs are data, you can assert on them. The catch: `init()` installs a
subscriber globally, once per process, and `cargo test` runs your tests in
parallel in one process — so tests would fight over it. Scope one to a
closure instead:

```rust
let ((), events) = capture(|| record_request("/health", 200, 3));
assert_eq!(events[0].field("status"), Some("200"));
```

`testkit::capture` is given. It installs a JSON subscriber for the duration
of the closure with `tracing::subscriber::with_default`, then hands back
your return value and the events it captured. It switches off timestamps,
because a test asserting on output needs that output to be the same every
run.

The scoping is per *thread*: `with_default` sets the subscriber for the
thread that calls it, and `cargo test` gives each test its own thread —
that is what makes parallel tests independent. The flip side: `capture` does
not see events logged from threads you spawn inside the closure.

### Metrics — counters you expose

A log is one record per event. A metric is aggregated state: how many, how
long, right now.

`AtomicU64` is a `u64` from `std::sync::atomic` that several threads may
touch at once. You don't read and write it with `=`; you call methods —
`fetch_add(n, ordering)` adds and returns the previous value,
`load(ordering)` reads it — and each one is indivisible, so two threads
incrementing at the same moment cannot lose an increment between them:

```rust
pub struct Counter {
    hits: AtomicU64,
}

impl Counter {
    pub fn hit(&self) {
        self.hits.fetch_add(1, Ordering::Relaxed);
    }
}
```

`fetch_add` mutates through `&self`, so eight threads can count at once with
no `Mutex` (Lesson 16's `Send`/`Sync` paying off). `Ordering::Relaxed` is
right here because these counters guard nothing else — they only need to
not lose increments. What a scraper reads from a `/metrics` endpoint is
three lines per counter — a `# HELP` description, a `# TYPE` (`counter`
means it only ever goes up), then the name and the value:

```text
# HELP requests_total Requests served.
# TYPE requests_total counter
requests_total 2
```

It reads that every few seconds and keeps the numbers over time; that is all
Prometheus is, at the bottom. Real services reach for the `metrics` crate or
OpenTelemetry rather than hand-rolling — this lesson hand-rolls so the
mechanics stay visible: one `AtomicU64`, no extra dependency.

## Exercises

### Warm-up: `record_request`

Implement `record_request` so it emits exactly one event, with the values as
named fields:

```rust
pub fn record_request(_path: &str, _status: u64, _elapsed_ms: u64) {
    todo!("emit ONE event: a constant message, with the values as named fields")
}
```

The stub's parameters start with `_` because this course's lints make an
unused variable a compile error — rename them in the same edit where you
replace `todo!()`. You add `use tracing::info;` yourself too (or call
`tracing::info!` in full, as in the Events section above); rustc will only
say `cannot find macro`, with no suggestion. One test calls the function
twice with different values and asserts the two messages are equal: an
interpolated message fails it, which is the whole point.

### Main: `Metrics::record`

Implement `Metrics::record` so it counts every request, and counts an error
as well when the status is 5xx:

```rust
pub fn record(&self, _status: u64) {
    todo!("count the request, and the error too when the status is 5xx")
}
```

Note the signature: `&self`, not `&mut self` — see the Metrics note above.
One of the tests counts into one `Arc<Metrics>` from eight threads at once.
If you write `&mut self` instead, every error rustc prints will point at
`tests/exercise.rs` rather than your code, and several will suggest adding
`mut` to a binding in the test — don't: the tests are right, the signature
is the thing to change. `Metrics`, its fields and `render_prometheus` are
given; read `render_prometheus` to see what your counters turn into.

### Compile-fail

`exercises/compile_fails/25-span-guard-temporary.rs` enters a span in the
same expression that creates it, so the span dies at the end of the
statement while the guard still borrows it (E0716 — temporary value
dropped while borrowed). Fix it by binding the span to a name first; rustc's
own suggestion shows the shape. You can check this one without waiting for
the tests: `cargo run --package compile-fails -- --expect compiles
lessons/25-observability`.

### Run

```bash
make verify LESSON=25-observability
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
