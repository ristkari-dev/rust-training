# Rust Training

A Rust programming course delivered as code + per-lesson reveal.js slide
decks. The arc starts at programming-101 and finishes with concurrency,
systems programming, production services, tooling, and distributed
patterns.

## Prerequisites

- [`rustup`](https://rustup.rs)
- The toolchain pinned in `rust-toolchain.toml` (auto-installed by
  `rustup` the first time you run `cargo` in this directory)
- `make`

## Quick start

```bash
make help                                  # list every available command
make new-lesson NAME=99-demo               # scaffold a sandbox lesson
make slides-dev LESSON=01-hello-rust       # serve a deck on http://localhost:8000
make test                                  # run all tests + compile-fail checks
```

## Repository layout

```
lessons/NN-name/
├── README.md      self-study notes for the lesson
├── slides/        reveal.js deck (index.html + slides.md)
├── exercises/     starter code + failing tests (the spec)
│   └── compile_fails/   optional: standalone .rs files that must NOT compile
└── solutions/     reference implementation

shared/reveal/     vendored reveal.js + Rust theme (do not edit by hand)
templates/         used by `make new-lesson`
tools/             developer tooling (new-lesson, slides-dev, compile-fails)
docs/              design docs and implementation plans
```

## Design

See [`docs/superpowers/specs/2026-05-21-rust-course-design.md`](docs/superpowers/specs/2026-05-21-rust-course-design.md).
