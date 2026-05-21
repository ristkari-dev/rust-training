# Rust Training — Course Design

A Rust programming course delivered as code + per-lesson reveal.js slide decks.
The arc starts at programming-101 and finishes with concurrency, systems
programming, production services, tooling, and distributed patterns —
mirroring the Go training course's arc, but reshaped around Rust-idiomatic
content and pedagogy.

## Audience and goals

The course is for **total beginners** who advance through the material into
advanced topics. No prior Rust experience and no required prior programming
experience are assumed at lesson 1. Each phase compounds on the previous one,
so the learning curve stays steady rather than spiking at the borrow checker.

Success criteria:

- A learner who finishes all lessons can read and write idiomatic Rust,
  reason about ownership and lifetimes, build a small async HTTP service,
  use the standard testing/linting/profiling tooling, and understand the
  shape of distributed systems built in Rust.
- The course is maintainable: a single author can keep it building, tested,
  and current without heroic effort.

## Curriculum arc

31 lessons across 8 phases. Phase boundaries are pedagogical, not enforced
in the directory layout.

### Phase 1 — Programming 101 in Rust (6 lessons)

1. Hello, Rust — toolchain (`rustup`, `cargo`), the first program, why Rust
2. Variables, types, mutability — `let`/`mut`, shadowing, type inference
3. Control flow & functions — expressions vs statements, returning values
4. Compound types — tuples, arrays, slices, `String` vs `&str`
5. Pattern matching & enums — `match`, `if let`, `Option<T>`
6. Structs & methods — `impl` blocks, associated functions

### Phase 2 — Ownership deep dive (5 lessons)

The Rust-native block. This is where the curriculum diverges most from Go's
arc: ownership, borrowing, and lifetimes get their own focused lessons rather
than being folded into other topics.

7. Ownership & moves
8. References & borrowing
9. Lifetimes
10. Smart pointers — `Box`, `Rc`, `Arc`, `RefCell`
11. Iterators & collections — `Vec`, `HashMap`, iterator chains

### Phase 3 — Abstraction (4 lessons)

12. Traits & generics
13. Trait objects vs static dispatch
14. Error handling — `Result`, `?`, `thiserror`/`anyhow`
15. Modules, crates, workspaces

### Phase 4 — Concurrency (3 lessons)

16. Threads, `Send`/`Sync`, channels
17. Shared state — `Mutex`, `RwLock`, `Arc<Mutex<T>>`
18. Async/await with Tokio

### Phase 5 — Systems programming (3 lessons)

19. Memory, layout, `unsafe`
20. FFI — calling C from Rust and vice versa
21. I/O & filesystem

### Phase 6 — Production services (4 lessons)

22. Building a CLI — `clap`
23. HTTP services with Axum
24. Persistence — `sqlx`, migrations, transactions
25. Observability — `tracing`, structured logs, metrics

### Phase 7 — Tooling & quality (3 lessons)

26. Testing — unit, integration, doc tests, property testing
27. Clippy, rustfmt, miri — the linter ecosystem & CI
28. Benchmarking & profiling — `criterion`, flamegraphs

### Phase 8 — Distributed patterns (3 lessons)

29. Serialization & gRPC — `serde`, `tonic`/`prost`
30. Resilience patterns — retries, timeouts, circuit breakers
31. Capstone — distributed task processor tying everything together

## Lesson structure

Each lesson lives at `lessons/NN-name/` and contains exactly four entries:

```
lessons/NN-name/
├── README.md      self-study notes for the lesson
├── slides/        reveal.js deck (index.html + slides.md)
├── exercises/     starter code + failing tests (the spec)
└── solutions/     reference implementation
```

`exercises/` and `solutions/` are each their own Cargo crate (own
`Cargo.toml`, `src/`, `tests/`) and are registered as workspace members in
the root `Cargo.toml`. They share the workspace `target/` directory, so
incremental builds across the whole course stay fast.

### Exercise failure model

Exercises express their spec in one of two ways:

1. **Failing tests** (the default). `exercises/src/lib.rs` ships stubs that
   call `todo!()`. `exercises/tests/` contains the spec as tests that fail
   loudly until the student fills in the stubs. `cargo test` is the
   green-light signal.

2. **Compile-fail exercises** (used in the ownership deep dive). When a
   lesson teaches a concept that's best expressed as "this code doesn't
   compile until you fix it", the lesson includes an
   `exercises/compile_fails/` directory containing standalone `.rs` files
   with a `// @check-only` marker. Each file is invoked through
   `cargo check` (via a small wrapper in `tools/`); the exercise passes
   when the file compiles cleanly.

`solutions/` always contains a reference implementation. The solution
crate's tests are the same as the exercise's tests, and they must pass.
This is the regression suite for the course.

## Repository layout

```
rust-training/
├── Cargo.toml                workspace manifest (members = exercises + solutions + tools)
├── Cargo.lock
├── Makefile
├── README.md
├── rustfmt.toml
├── clippy.toml
├── rust-toolchain.toml       pinned channel + components
├── .github/workflows/ci.yml
│
├── lessons/NN-name/          (see above)
│
├── shared/
│   └── reveal/               vendored reveal.js + custom Rust theme (do not edit by hand)
│
├── tools/
│   ├── new-lesson/           scaffolds a new lessons/NN-name/ and registers crates
│   └── slides-dev/           serves a deck on localhost with file watching
│
└── docs/superpowers/specs/   design docs and implementation plans
```

`tools/new-lesson` and `tools/slides-dev` are implemented as Rust binaries
(workspace members). This dogfoods the language being taught and keeps the
course self-contained — no Python, Node, or Ruby in the dev workflow.

## Tooling and developer UX

The Make targets mirror the Go course's UX, so the cognitive load of moving
between the two courses is minimal:

- `make help` — list every target with one-line descriptions
- `make new-lesson NAME=NN-name` — scaffold a new lesson from templates and
  register the new `exercises/` and `solutions/` crates in the root
  `Cargo.toml` workspace `members` list
- `make slides-dev LESSON=NN-name` — serve the reveal.js deck on
  `http://localhost:8000` with live reload on slide changes
- `make test` — `cargo test --workspace` across every lesson, plus the
  compile-fail wrapper across every `compile_fails/` directory
- `make lint` — `cargo clippy --workspace --all-targets -- -D warnings`
  followed by `cargo fmt --all --check`
- `make fmt` — `cargo fmt --all`
- `make ci` — equivalent to `make lint test`; this is what GitHub Actions runs

### `make test` semantics

`cargo test --workspace` covers every lesson's `exercises/` and `solutions/`
crate in one pass. Exercises whose stubs still call `todo!()` will panic
inside the tests, producing loud, locatable failures — that's the intended
"this isn't done yet" signal.

For lessons with `compile_fails/` directories, the test runner additionally
invokes `cargo check` on each `.rs` file in that directory and asserts the
exit status (zero = passing, the file compiles; non-zero = the student
still has work to do). A small wrapper binary in `tools/` handles this.

## Toolchain, edition, lints

- **Edition: 2024.** Current as of this course's authoring; default
  behaviors are friendlier for beginners (e.g., disjoint captures in
  closures).
- **MSRV: pinned via `rust-toolchain.toml`.** Channel: a recent stable
  (e.g., `1.85`) chosen at course-authoring time. Components: `rustfmt`,
  `clippy`, `rust-src`. Every learner uses the same toolchain via
  `rustup`.
- **`rustfmt.toml`:** defaults. No bikeshedding; pedagogy beats personal
  style.
- **`clippy.toml` + workspace lints in `Cargo.toml`:**
  - Deny: `clippy::all`, `clippy::pedantic`, `rust_2018_idioms`
  - Allow (workspace-wide, to avoid lints that fight with teaching):
    `clippy::missing_errors_doc`, `clippy::needless_pass_by_value`
    (especially in the ownership phase). Individual lessons can override
    further if needed.
- **CI:** GitHub Actions runs `make ci` on stable and beta. Caches
  `~/.cargo` and `target/` between runs to keep wall-clock time low.

## Slides

The reveal.js setup matches the Go course's: a vendored copy under
`shared/reveal/`, a thin `index.html` per lesson that loads `slides.md`,
and the same plugins (`highlight`, `notes`, `search`). The custom theme is
restyled with a Rust-flavored accent palette (orange/rust) replacing Go's
blue, but the markup structure is otherwise unchanged. Rust syntax
highlighting comes from highlight.js's bundled `rust` mode.

`make slides-dev LESSON=NN-name` runs the `tools/slides-dev` binary, which
serves the deck and watches the directory for changes.

## Initial implementation scope

The first implementation plan covers the **course infrastructure**, not the
full lesson catalog. Authoring 31 lessons is an ongoing effort that follows
its own cadence; the implementation plan derived from this spec is "done"
when:

- The Cargo workspace, Makefile, `rust-toolchain.toml`, `rustfmt.toml`,
  `clippy.toml`, and CI workflow exist and pass.
- `tools/new-lesson` and `tools/slides-dev` are implemented, tested, and
  documented.
- `shared/reveal/` contains the vendored reveal.js bundle and the Rust
  theme.
- **One reference lesson** (`lessons/01-hello-rust/`) is fully written
  end-to-end — README, slides, exercises (with both a passing-test exercise
  and a `compile_fails/` exercise as proof of mechanism), and solutions —
  so the templates and tooling are validated against a real lesson.
- `make help`, `make new-lesson`, `make slides-dev`, `make test`,
  `make lint`, `make fmt`, and `make ci` all work locally and in CI.

Subsequent lessons are authored one at a time outside this design's
implementation plan, each as its own small piece of work.

## Out of scope

The following are explicitly excluded from this design so the scope stays
implementable by a single author:

- A web-hosted student portal, progress tracking, or grading. Self-study
  only.
- Video recordings, screencasts, or live-coaching infrastructure.
- Translations / non-English content.
- Embedded/no_std, WASM, and game-dev tracks. These are valid Rust niches
  but each could fill its own course; the distributed-patterns phase
  intentionally stays in the same conceptual territory as the Go course's
  final phase rather than pivoting into one of these.

## References

- Go training course design (mirrored arc):
  `../../docs/superpowers/specs/2026-05-05-go-course-design.md` (in the
  sibling `go-training` repository, not vendored here).
