# Lesson 25 — Observability — design

The last lesson of Phase 6 (Production services). A service you cannot see
into is a service you cannot operate. `tracing` turns what your code did
into structured **events** and **spans**; counters turn what it did *a lot
of* into numbers a scraper can graph. The production skill at the center:
**a log line is data, not prose** — the message stays constant and the
values go in named fields, so the output can be searched, filtered and
aggregated instead of grepped. Builds on traits (L12), `Arc` and shared
state (L10/L17), threads and `Send`/`Sync` (L16), and testing without the
real thing (L21/L22/L23/L24).

A deliberate constraint: the exercises assert on what was logged, using a
**given** capture harness that installs a subscriber for the duration of one
closure — never globally — so the tests stay independent while `cargo test`
runs them in parallel.

## Audience and prerequisites

- Has completed Lessons 01-24
- Comfortable with `Arc` and interior mutability (L10/L17), spawning threads
  (L16), closures (L11), and adding a dependency (L14/L18/L22/L23/L24)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Emit a structured event with `tracing::info!`, keeping the message
   constant and putting the values in named fields
2. Explain why `info!(path, status, "request finished")` is worth more than
   `info!("request {path} finished with {status}")` — the first can be
   filtered and aggregated, the second can only be grepped
3. Test what code logs, by installing a subscriber scoped to one closure
   rather than a global one
4. Count events with `AtomicU64` counters behind a shared `&self`, and
   explain why counters need no `Mutex`
5. Describe the difference between a log (one line per event) and a metric
   (aggregated state you expose), and read Prometheus' text exposition
   format

## Scope

In scope: `tracing::info!` with named fields; the message-as-constant rule;
levels (`info!`/`warn!`/`error!`); what a **span** is and what
`#[tracing::instrument]` adds, shown but not exercised; installing a
subscriber with `tracing_subscriber::fmt()`, including the JSON formatter;
scoping a subscriber to one closure with `tracing::subscriber::with_default`
(the given `testkit::capture`); counters as `AtomicU64` with
`Ordering::Relaxed`; the Prometheus text exposition format (`# HELP`,
`# TYPE`, `name value`). New infrastructure: `tracing`,
`tracing-subscriber` (with the `json` feature) and `serde_json` join
`[workspace.dependencies]`.

Out of scope (deferred or skipped): OpenTelemetry and distributed tracing;
trace/span propagation across services; log shipping and aggregation
backends; `EnvFilter` and `RUST_LOG` beyond a mention; writing a custom
`Layer`; span sampling; metric labels and histograms; the `metrics` crate
as a dependency (named as what production reaches for, with the
hand-rolling justified by keeping the counter's mechanics visible); async instrumentation beyond a
mention. Observability is introduced as *structured events you can assert
on, and counters you can expose*; shipping telemetry anywhere is out of
band.

## New dependency infrastructure

Three new entries in the root `[workspace.dependencies]`:

```toml
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
```

Verified during design on rustc 1.98.1: `tracing` resolves to 0.1.44 and
`tracing-subscriber` to 0.3.23, and there is no MSRV trouble of the kind
lesson 24 hit — the highest declared `rust-version` in the tree is 1.71.
`tracing`'s default features are kept because `#[instrument]` lives behind
its `attributes` feature. `tracing-subscriber`'s defaults already include
`fmt`, which is the sole gate for the `tracing_subscriber::fmt()` entry
point; `json` is added for the capture harness. `serde_json` is an
implementation detail of that harness — students never touch it, and serde
proper is Lesson 29.

Both lesson crates add, after `[lints]`:

```toml
[dependencies]
serde_json = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
```

No C toolchain and no external service: this lesson locks 7 new packages
(`tracing-subscriber` and its tree) and costs a few seconds of build time —
`tracing` and `serde_json` are already in the workspace graph via sqlx and
axum.

## Slide arc (10 slides)

1. **Title — Observability.** Hook: *"A service you cannot see into is a
   service you cannot operate. The question is never 'did it log
   something?' — it is 'can I find the one request that failed, among a
   million that didn't?'"*
2. **Two kinds of signal.** A **log** is one record per event: what
   happened, once, with detail. A **metric** is aggregated state: how many,
   how long, right now. You need both, and they answer different questions.
3. **Structured events.**
   ```rust
   info!(path, status, elapsed_ms, "request finished");
   ```
   The message is a constant; the values are named fields. Compare
   `info!("request {path} finished with {status}")` — just as readable, but
   nothing can filter it by `status` afterwards.
4. **Levels.** `error!`, `warn!`, `info!`, `debug!`, `trace!`. Levels are
   for the *reader's* filter, not for your feelings: `error!` means someone
   should look, `info!` means this is the story of what happened.
5. **Spans.** An event is a point; a **span** is an interval with a name and
   fields, and everything logged inside it inherits them.
   ```rust
   #[tracing::instrument]
   fn handle(path: &str) { /* every event in here carries path */ }
   ```
6. **Subscribers.** Nothing is recorded until something subscribes —
   `tracing_subscriber::fmt().init()` prints human-readable lines,
   `.json()` prints one JSON object per event, and `.with_max_level(...)`
   sets the bar (INFO by default). Without a subscriber the macros are
   no-ops, which is why a library can log freely.
7. **You get other crates' telemetry for free.** Install one subscriber and
   every instrumented dependency starts talking — sqlx logs at DEBUG, so
   the default INFO subscriber hides it. Lesson 24's database layer,
   unchanged, emits (abridged):
   ```text
   DEBUG sqlx::query: summary="INSERT INTO accounts (name, …" rows_affected=1 elapsed=52.041µs
   ```
   You wrote none of that.
8. **Testing what you log.** A global subscriber is process-wide and
   one-shot, which parallel tests cannot share. Scope one to a closure
   instead — the given `testkit::capture` returns your value *and* the
   events, so a test asserts on both.
9. **Putting it together.** Today's exercises: `record_request` emits one
   structured event (warm-up); `Metrics::record` counts requests and 5xx
   errors with atomics (main). `testkit::capture` and
   `render_prometheus` are given — read them. The compile-fail enters a
   span that dies at the end of the statement.
10. **Wrap — observability in Rust.** Five takeaways: the message is
    constant and the values are fields; levels are the reader's filter; a
    span gives every event inside it context; a subscriber decides what is
    recorded and where; a counter is aggregated state you expose, not prose
    you log. Next: **Lesson 26 — Testing** (unit, integration, doc tests,
    property testing).

## Exercise spec

`lessons/25-observability/` follows the standard four-part lesson shape:

```
25-observability/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml          # adds tracing + tracing-subscriber + serde_json
│   ├── src/
│   │   ├── lib.rs
│   │   └── testkit.rs      # given
│   ├── tests/exercise.rs
│   └── compile_fails/25-span-guard-temporary.rs
└── solutions/
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   └── testkit.rs
    └── tests/exercise.rs
```

Cargo package names: `observability-exercises` and
`observability-solutions` (import idents `observability_exercises` /
`observability_solutions`). This matches the build-index registry slug
`observability`, so the landing page links lesson 25 unchanged.

### Given: `src/testkit.rs` (both crates, byte-identical)

The harness students do not write. It installs a JSON subscriber for the
duration of one closure with `tracing::subscriber::with_default`, then
parses each line into an `Event`. Verified during design: four tests
capturing 2000 events each, run in parallel three times, never saw each
other's output.

```rust
//! GIVEN: capture what your code logs, so a test can assert on it.
//!
//! `capture` installs a JSON subscriber for the duration of one closure and
//! nothing else — not globally — so tests stay independent even though
//! `cargo test` runs them in parallel.
//!
//! You only ever call `capture`. The rest of this file is plumbing you are
//! not expected to follow: `Buffer` is a `Vec<u8>` behind an `Arc<Mutex<_>>`
//! that the subscriber writes into, and `parse` turns each JSON line back
//! into an `Event`.

use std::collections::BTreeMap;
use std::io;
use std::sync::{Arc, Mutex};

use tracing_subscriber::fmt::MakeWriter;

/// One captured event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// `INFO`, `WARN`, `ERROR`, …
    pub level: String,
    /// The event's constant message.
    pub message: String,
    /// Every other field, as text.
    pub fields: BTreeMap<String, String>,
}

impl Event {
    /// The value of one field, or `None` if the event doesn't carry it.
    #[must_use]
    pub fn field(&self, name: &str) -> Option<&str> {
        self.fields.get(name).map(String::as_str)
    }
}

#[derive(Clone, Default)]
struct Buffer(Arc<Mutex<Vec<u8>>>);

impl io::Write for Buffer {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for Buffer {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

fn parse(raw: &str) -> Vec<Event> {
    raw.lines()
        .map(|line| {
            let value: serde_json::Value =
                serde_json::from_str(line).expect("captured line was not JSON");
            let mut fields: BTreeMap<String, String> = value["fields"]
                .as_object()
                .expect("event had no fields")
                .iter()
                .map(|(k, v)| (k.clone(), text(v)))
                .collect();
            let message = fields.remove("message").unwrap_or_default();
            Event {
                level: value["level"].as_str().unwrap_or_default().to_string(),
                message,
                fields,
            }
        })
        .collect()
}

fn text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// Run `f` with a private subscriber; return its value and what it logged.
pub fn capture<T>(f: impl FnOnce() -> T) -> (T, Vec<Event>) {
    let buffer = Buffer::default();
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_writer(buffer.clone())
        .without_time()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    let out = tracing::subscriber::with_default(subscriber, f);
    let raw = String::from_utf8(buffer.0.lock().unwrap().clone()).expect("log was not UTF-8");
    (out, parse(&raw))
}
```

`.without_time()` is what makes the captured output deterministic (the JSON
formatter never emits colour, so there is nothing else to switch off);
`with_default` rather than `init()` is what makes it per-test. Returning `(T, Vec<Event>)` means a test asserts on the return
value and the log in one call, with no guard for a student to drop early.

### Exercise stub (`exercises/src/lib.rs`)

`Metrics`, its fields and `render_prometheus` ship as given; the two
exercises ship with `todo!()` bodies. The crate and tests compile; the
tests fail at runtime with the `todo!()` panic.

```rust
//! Lesson 25 — exercises.
//!
//! Implement `record_request` (warm-up) and `Metrics::record` (main) so that
//! `cargo test --manifest-path lessons/25-observability/exercises/Cargo.toml`
//! passes. `testkit::capture` and `Metrics::render_prometheus` are given.
//! The tests live in `tests/exercise.rs`.
//!
//! You add `use tracing::info;` yourself — the stub ships without it,
//! because an unused import is a compile error in this course.

pub mod testkit;

use std::sync::atomic::{AtomicU64, Ordering};

/// Record that a request finished.
///
/// One event: a constant message, with the values as named fields.
// The `_` prefixes keep the unfinished stubs compiling (unused variables
// are errors in this course). Rename them when you write the body.
pub fn record_request(_path: &str, _status: u64, _elapsed_ms: u64) {
    todo!("emit ONE event: a constant message, with the values as named fields")
}

/// Counters a service exposes for scraping.
#[derive(Debug, Default)]
pub struct Metrics {
    requests: AtomicU64,
    errors: AtomicU64,
}

impl Metrics {
    /// Count one finished request, and one error if the status is 5xx.
    pub fn record(&self, _status: u64) {
        todo!("count the request, and the error too when the status is 5xx")
    }

    /// GIVEN: render the counters in Prometheus' text exposition format.
    ///
    /// This is what a `/metrics` endpoint serves; a scraper reads it every
    /// few seconds and keeps the numbers over time.
    #[must_use]
    pub fn render_prometheus(&self) -> String {
        let requests = self.requests.load(Ordering::Relaxed);
        let errors = self.errors.load(Ordering::Relaxed);
        format!(
            "# HELP requests_total Requests served.\n\
             # TYPE requests_total counter\n\
             requests_total {requests}\n\
             # HELP errors_total Requests that failed.\n\
             # TYPE errors_total counter\n\
             errors_total {errors}\n"
        )
    }
}
```

Note the shape constraint: `render_prometheus` must be given rather than
exercised, because it is the only reader of the two counter fields. With
both methods `todo!()`, the workspace's denied `unused` turns the fields
and the `Ordering` import into compile errors and the stub does not build —
verified during design.

### Warm-up: `record_request`

Reference solution:

```rust
/// Record that a request finished.
///
/// One event: a constant message, with the values as named fields.
pub fn record_request(path: &str, status: u64, elapsed_ms: u64) {
    info!(path, status, elapsed_ms, "request finished");
}
```

Pedagogical packing: the entire structured-logging idea in one line. Field
shorthand (`path` rather than `path = path`) is the idiom students will see
everywhere; the message sits last and stays constant. The test that makes
this real is `warmup_message_is_constant`, which calls the function twice
with different values and asserts the two messages are equal — an
interpolated `info!("request {path} finished")` fails it with
`left: "request /health finished…"` / `right: "request /orders finished…"`,
and fails the two field tests as well (verified during design).

Four tests:

```rust
#[test]
fn warmup_emits_exactly_one_event() {
    let ((), events) = capture(|| record_request("/health", 200, 3));
    assert_eq!(events.len(), 1, "expected one event, got {events:?}");
    assert_eq!(events[0].level, "INFO");
}

#[test]
fn warmup_values_are_fields() {
    let ((), events) = capture(|| record_request("/health", 200, 3));
    let event = events
        .first()
        .expect("no event was recorded - record_request must emit one");
    assert_eq!(
        event.field("path"),
        Some("/health"),
        "None means the event carries no field called `path` - the values belong beside the message, not inside it"
    );
    assert_eq!(event.field("status"), Some("200"));
    assert_eq!(event.field("elapsed_ms"), Some("3"));
}

#[test]
fn warmup_message_is_constant() {
    let ((), one) = capture(|| record_request("/health", 200, 3));
    let ((), two) = capture(|| record_request("/orders", 500, 41));
    let (one, two) = (
        one.first().expect("no event recorded for /health"),
        two.first().expect("no event recorded for /orders"),
    );
    assert_eq!(
        one.message, two.message,
        "the message must not change with the values - put them in fields, not in the text"
    );
}

#[test]
fn warmup_fields_survive_different_values() {
    let ((), events) = capture(|| record_request("/orders", 503, 41));
    let event = events
        .first()
        .expect("no event was recorded - record_request must emit one");
    assert_eq!(
        event.field("path"),
        Some("/orders"),
        "the same fields must appear whatever the values are"
    );
    assert_eq!(event.field("status"), Some("503"));
    assert_eq!(event.field("elapsed_ms"), Some("41"));
}
```

### Main: `Metrics::record`

Reference solution:

```rust
/// Count one finished request, and one error if the status is 5xx.
pub fn record(&self, status: u64) {
    self.requests.fetch_add(1, Ordering::Relaxed);
    if status >= 500 {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }
}
```

Pedagogical packing: three lines that carry the whole metrics idea and the
course's only appearance of atomics. `&self` rather than `&mut self` is the
point — `fetch_add` mutates through a shared reference, so a counter needs
no `Mutex` and eight threads can hit it at once (Lesson 16's `Send`/`Sync`
payoff, and Lesson 17's alternative seen from the other side).
`Ordering::Relaxed` is correct here because these counters synchronize
nothing else; the README says that in one sentence rather than opening the
memory-ordering rabbit hole.

Four tests:

```rust
#[test]
fn main_counts_requests() {
    let metrics = Metrics::default();
    metrics.record(200);
    metrics.record(404);
    assert!(metrics.render_prometheus().contains("requests_total 2"));
}

#[test]
fn main_counts_server_errors_only() {
    let metrics = Metrics::default();
    metrics.record(200);
    metrics.record(404);
    metrics.record(500);
    metrics.record(503);
    let rendered = metrics.render_prometheus();
    assert!(
        rendered.contains("errors_total 2"),
        "only 5xx counts as an error: {rendered}"
    );
}

#[test]
fn main_counters_are_shared_across_threads() {
    let metrics = Arc::new(Metrics::default());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let metrics = Arc::clone(&metrics);
            thread::spawn(move || {
                for _ in 0..1000 {
                    metrics.record(200);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert!(
        metrics.render_prometheus().contains("requests_total 8000"),
        "8 threads x 1000 requests must all be counted"
    );
}

#[test]
fn main_renders_the_exposition_format() {
    let metrics = Metrics::default();
    metrics.record(200);
    metrics.record(500);
    assert_eq!(
        metrics.render_prometheus(),
        "# HELP requests_total Requests served.\n\
         # TYPE requests_total counter\n\
         requests_total 2\n\
         # HELP errors_total Requests that failed.\n\
         # TYPE errors_total counter\n\
         errors_total 1\n"
    );
}
```

**Eight tests total** (four warm-up + four main). They are deterministic and
parallel-safe: the warm-up tests each install their own subscriber, and the
metrics tests share nothing.

### Compile-fail: `25-span-guard-temporary.rs`

Path: `exercises/compile_fails/25-span-guard-temporary.rs`. Self-contained
and std-only — the `compile-fails` tool type-checks with bare `rustc` and no
`--extern`, so it cannot use `tracing`. It mirrors the shape of
`tracing::Span::enter`, whose guard borrows the span. (`Span::entered`
takes the span *by value* instead and hands back an owned guard — the
borrowing one is `enter`, which is what this file stands in for.)

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Entering a span with `Span::enter` hands you a GUARD that borrows the
// span, and the span stays open until the guard drops. So the span itself
// has to outlive the guard — if you create it and enter it in one
// expression, the span is a temporary that dies at the end of that
// statement, while the guard is still holding a borrow of it.
//
// This file reproduces that with plain structs (`Span::enter` here stands in
// for tracing's). rustc reports E0716: "temporary value dropped while
// borrowed".
//
// The fix: give the span a name first, so it lives as long as the guard —
//     let span = span("handle_request");
//     let guard = span.enter();

struct Span {
    name: String,
}

struct Entered<'a> {
    span: &'a Span,
}

impl Span {
    fn enter(&self) -> Entered<'_> {
        Entered { span: self }
    }
}

fn span(name: &str) -> Span {
    Span {
        name: name.to_string(),
    }
}

fn main() {
    let guard = span("handle_request").enter();
    println!("inside the span for {}", guard.span.name);
}
```

Pass condition: the student binds the span to a name before entering it.
rustc reports E0716 "temporary value dropped while borrowed" and even
suggests the fix ("consider using a `let` binding to create a longer lived
value") — verified during design against rustc 1.98.1 under the tool's exact
invocation. After the change the file type-checks.

This is the lesson's lifetime payback (L09), as Lesson 24's was the
ownership payback (L07): a span stays open exactly as long as its guard
lives, so the span has to outlive the guard.

## README structure

`lessons/25-observability/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - Events, and why fields beat interpolation
  - Levels and spans
  - Subscribers — what records, and where
  - Testing what you log
  - Metrics — counters you expose
- **Exercises** — four subsections: Warm-up (`record_request`), Main
  (`Metrics::record`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block; the
Metrics subsection carries two (the counter type and the Prometheus
exposition text) and the Compile-fail subsection is prose only, so the
README has 9 code blocks. The "Events" and "Metrics" sections are the
heaviest — they carry the two production skills. The Metrics section states plainly that real services use
the `metrics` crate or OpenTelemetry, and why this lesson hand-rolls
anyway: so the counter's mechanics stay visible — one `AtomicU64`, no extra
dependency.

## Lint expectations

Lesson 25's reference solution is clippy-clean (`clippy::all` +
`clippy::pedantic` denied) with **no `#[allow]` attributes** — verified
during design by building the complete lesson under the repo's exact lint
table:

- `render_prometheus` carries `#[must_use]` (it returns a `String` and has
  no side effect). `record` and `record_request` return `()`, so they do
  not, and `Event::field` does.
- The stub's exercise parameters are `_`-prefixed because the workspace
  denies `unused`; every import is used by the given code.
- `testkit.rs` needs no allow: the `MakeWriter` impl, the `Mutex` unwraps
  and the `expect` calls all pass pedantic. `missing_panics_doc` is already
  allowed workspace-wide, which covers the harness's `expect`s.
- The lesson uses `tracing`'s macros only. `#[instrument]` appears on slides
  and in the README but not in either crate, so nothing depends on the
  `attributes` feature at build time.

If clippy fires on anything unexpected, fix the code rather than adding an
allow, and report it.

## Done criteria

- `lessons/25-observability/` exists with the four-part structure, plus
  `src/testkit.rs` in both crates
- Root `Cargo.toml` `[workspace.dependencies]` gains `serde_json`,
  `tracing` and `tracing-subscriber` (with the `json` feature); both lesson
  `Cargo.toml`s declare all three as workspace dependencies
- Cargo manifests use the package names `observability-exercises` and
  `observability-solutions`
- Both crates' `src/testkit.rs` are byte-identical
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `record_request`, `Metrics`, `Metrics::record` and
  `Metrics::render_prometheus`; the exercise ships `todo!()` bodies for the
  first two, the solution ships real ones
- `cargo test --package observability-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/25-observability/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented`
- `cargo run --package compile-fails -- --expect broken lessons/25-observability`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/25-observability`
  → fails (the file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/25-observability/slides/index.html`
- `dist/index.html` lists lesson 25 as a clickable link (registry slug
  `observability` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy run;
  `https://rust.ristkari.dev/lessons/25-observability/slides/` returns 200

## Open questions

None.
