# Contributing

## Authoring a new lesson

```bash
make new-lesson NAME=07-ownership
```

This creates `lessons/07-ownership/` with the four-part structure: `README.md`,
`slides/`, `exercises/`, `solutions/`. The scaffolded files have placeholder
content — fill them in.

### The four-part structure

Every lesson has exactly these four parts:

1. **`README.md`** — self-study notes that mirror the deck narrative.
   Sections: Learning goals, Self-study notes, Exercises, Solutions.
2. **`slides/`** — the live-lecture deck. `index.html` is reveal.js bootstrap;
   `slides.md` is the markdown content. Use `Note:` blocks for speaker notes.
3. **`exercises/`** — starter code that **compiles** but is incomplete (use
   `todo!()` for unimplemented bodies). The accompanying tests in
   `exercises/tests/` contain **failing tests** that act as the spec.
   Optionally, `exercises/compile_fails/` holds standalone `.rs` files that
   ship in a deliberately broken state — the student fixes them so they
   compile.
4. **`solutions/`** — the same crate shape as `exercises/`, fully implemented.
   The tests in `solutions/` must be identical to those in `exercises/` so
   `cargo test --package <name>-solutions` passes.

### Slide style

- First slide: lesson number, title, one-line learning goal.
- Last slide: pointer to the next lesson.
- Code-heavy slides: limit to ~15 visible lines. Split larger examples and
  use the highlight plugin's `[highlight]` syntax to focus attention.
- Diagrams: SVG. Never images of code.

### Tests as the spec

Exercise tests fail by design until the student completes the lesson.
`make test` runs only the `default-members` of the workspace (tools + solutions),
which keeps CI green while exercises remain unfinished. Students opt in to
their own lesson with `make verify LESSON=NN-name`, which runs that lesson's
exercise tests AND checks that any `compile_fails/` files now compile.

Compile-fail exercises follow a complementary pattern: the file ships in a
state that does **not** compile, the student edits it until it does, and
`make verify` confirms the fix via `cargo run --package compile-fails -- --expect compiles`.
The author/CI check is `--expect broken`, which `make test` runs to guarantee
exercises ship in their intended broken state.

### Crate naming

The scaffolder derives Cargo package names from a lesson's "bare" name (the
part after the `NN-` prefix), so `lessons/01-hello-rust/exercises` becomes
the package `hello-rust-exercises`. This is because the package name shows
up in `use` statements as a Rust identifier (`use hello_rust_exercises::...`),
and Rust identifiers cannot start with a digit. The lesson directory keeps
its `NN-` prefix for ordering.

## Local workflow

```bash
make fmt                            # format
make lint                           # clippy + rustfmt --check
make test                           # all tests except exercises + compile-fail (expect broken)
make verify LESSON=01-hello-rust    # one lesson, both sides
make slides-dev LESSON=01-hello-rust  # serve deck on http://localhost:8000
make slides-build                   # produce static dist/ landing page + decks
make slides-docker                  # build & run the deploy image locally on :8080
```

## Reveal.js

Reveal.js is vendored under `shared/reveal/`. To upgrade:

1. Download a new release tarball from
   <https://github.com/hakimel/reveal.js/releases>.
2. Replace `shared/reveal/dist/` and `shared/reveal/plugin/`.
3. Smoke-test one lesson with `make slides-dev LESSON=01-hello-rust`.

The custom theme lives at `shared/reveal/theme/rust.css` — keep it across
upgrades.

## Toolchain and lints

- The toolchain is pinned in `rust-toolchain.toml` (channel + components).
  Update only when intentionally moving to a newer Rust.
- Workspace lints are deny-by-default at `clippy::all` and `clippy::pedantic`.
  Don't disable these globally; an `#[allow(...)]` on a specific item with a
  brief comment is fine when a lint genuinely fights pedagogy.
- `cargo fmt --check` runs in CI. Run `make fmt` before committing.

## Commit messages

Conventional Commits: `feat:`, `fix:`, `chore:`, `docs:`, `test:`, `build:`,
`ci:`. Scope is optional, e.g. `feat(slides): …` or `feat(lesson-07): …`.

## Reference

- [Course design](docs/superpowers/specs/2026-05-21-rust-course-design.md)
- [Deploy guide](deploy/README.md)
