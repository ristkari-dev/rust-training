# Lesson 22 — Building a CLI — design

The first lesson of Phase 6 (Production services). `clap` turns raw
command-line arguments (`argv`) into a typed struct via
`#[derive(Parser)]`. The production skill at the center of the lesson:
make a CLI *testable* — parse with `try_parse_from(iter)` (not
`parse()`, which reads the real `argv`) and keep the command logic a
plain function, so tests never spawn a subprocess. Exit codes and
structured output complete the picture. Builds on traits/derive (L12),
`Result` (L14), and iterators (L11).

## Audience and prerequisites

- Has completed Lessons 01-21
- Comfortable with structs + derives (L06/L12), `Result` (L14),
  iterators (L11), and adding a dependency (L14/L18)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Define a CLI's arguments as a struct with `#[derive(Parser)]` and
   `#[arg(...)]` attributes (positional args, `--flags`, defaults)
2. Parse arguments with clap, and explain why `try_parse_from(iter)`
   (returns a `Result`) is the testable counterpart to `parse()` (reads
   real `argv` and exits on error)
3. Test a CLI without spawning a process — drive parsing with an array of
   strings and test the command logic as an ordinary function
4. Produce structured output from parsed arguments
5. Recognize that a CLI signals success/failure to the shell via an
   `ExitCode` from `main`, not just by printing

## Scope

In scope: `clap` derive (`#[derive(Parser)]`); argument attributes
(`#[arg(short, long, default_value_t = ...)]`, positional vs optional,
doc-comment help, required-by-default); parsing with `Cli::parse()`
(real `argv`) vs `Cli::try_parse_from(iter)` (testable, returns
`Result<Cli, clap::Error>`); testing a CLI by parsing an array of `&str`
and testing the command logic directly; producing structured output (a
`String`); exit codes via `std::process::ExitCode` (conceptual — `main`
returns `ExitCode`, `SUCCESS`/`FAILURE`/`from(n)`); stdout-vs-stderr and
machine-readable output (conceptual). The exercises drill a `parse`
wrapper (warm-up) and a `run` output function (main) for a small `greet`
CLI whose `Cli` struct ships complete.

Out of scope (deferred or skipped): subcommands (`#[derive(Subcommand)]`)
beyond a mention; `clap` builder API (derive only); value parsers /
custom validation; environment-variable fallbacks; shell completions;
colored help; arg groups / conflicts; actually reading real `argv` in a
test (we use `try_parse_from`); `anyhow` in `main` (L14 covered errors);
the `ExitCode` plumbing exercised in code (it's conceptual + the
compile-fail). A CLI is introduced as *derive a `Cli`, parse testably,
run a function, signal via `ExitCode`*; subcommands and the builder API
are out of band.

## Slide arc (10 slides)

1. **Title — Building a CLI.** Hook: *"A command-line tool is just a
   function from `argv` to an exit code. `clap` handles the tedious part
   — turning a list of strings into a typed struct, with help and error
   messages for free — so you can focus on what the tool does."*
2. **`argv` and `clap`.** A program receives its arguments as a list of
   strings (`std::env::args()`). Parsing them by hand — flags, defaults,
   `--help` — is tedious and error-prone. `clap` is the standard crate
   that does it: you describe the arguments, it does the parsing.
3. **`#[derive(Parser)]`.**
   ```rust
   use clap::Parser;

   #[derive(Parser)]
   #[command(name = "greet")]
   struct Cli {
       name: String,          // a required positional argument
       count: u8,             // (see next slide for flags/defaults)
   }
   ```
   Describe your arguments as a struct; each field becomes an argument.
   A bare field is a required positional; doc comments become `--help`
   text.
4. **Argument attributes.**
   ```rust
   /// How many times to greet them.
   #[arg(short, long, default_value_t = 1)]
   count: u8,
   ```
   `#[arg(...)]` configures a field: `long` gives `--count`, `short`
   gives `-c`, `default_value_t` makes it optional with a default.
   Without a default, the argument is required and clap errors if it's
   missing.
5. **Parsing.**
   ```rust
   let cli = Cli::parse();                       // in main: reads real argv
   let cli = Cli::try_parse_from(["greet", "Bob", "-c", "3"])?; // testable
   ```
   `Cli::parse()` reads the real `argv` and, on a bad argument, prints an
   error and exits the process. `try_parse_from(iter)` takes the
   arguments explicitly and returns a `Result` instead of exiting — which
   is what makes it testable.
6. **Testing CLIs.** Because `try_parse_from` doesn't touch the real
   `argv` or exit, you can test parsing in-process:
   ```rust
   let cli = Cli::try_parse_from(["greet", "Alice"]).unwrap();
   assert_eq!(cli.name, "Alice");
   ```
   And keep the *work* in a plain function (`run(&cli) -> String`) you can
   call and assert directly — no subprocess, no captured stdout.
7. **Exit codes.** A CLI tells the shell whether it succeeded through its
   *exit code*, not its output. `main` can return one:
   ```rust
   use std::process::ExitCode;
   fn main() -> ExitCode {
       if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
   }
   ```
   `0` means success; non-zero means failure. `ExitCode::from(2)` sets a
   specific code. Scripts and `&&`/`||` chains depend on it.
8. **Structured output.** Print results to **stdout** and errors/logs to
   **stderr**, so a user can pipe the results without the noise
   (`tool > out.txt`). For output other programs consume, offer a
   machine-readable format (e.g. a `--json` flag) alongside the
   human-friendly default.
9. **Putting it together.** Walk through the exercises: `parse` wraps
   `Cli::try_parse_from` so the parsing is testable (warm-up), and `run`
   turns a parsed `Cli` into the greeting output — `"Hello, {name}!"`
   repeated `count` times (main). The `Cli` struct (a `greet` CLI) is
   given. The compile-fail returns a bare integer from a `main` declared
   to return `ExitCode`.
10. **Wrap — Phase 6 begins.** Five takeaways: `#[derive(Parser)]`
    turns a struct into a CLI; `#[arg(...)]` configures each argument;
    `try_parse_from` is the testable parse; keep the work in a function
    you can call directly; `main` returns an `ExitCode` to signal
    success or failure. Next: **Lesson 23 — HTTP services with Axum**.

## Exercise spec

`lessons/22-cli/` follows the standard four-part lesson shape, plus a
dependency in each crate's `Cargo.toml`:

```
22-cli/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml          # adds clap = { workspace = true }
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/22-exitcode-mismatch.rs
└── solutions/
    ├── Cargo.toml          # adds clap = { workspace = true }
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `cli-exercises` and `cli-solutions` (the lesson's
"bare" name is `cli`; the import idents are `cli_exercises` /
`cli_solutions`). This matches the build-index master registry slug
`cli`, so the landing page links it without any change.

### Cargo.toml dependency

`clap` is **already** a `[workspace.dependencies]` entry
(`clap = { version = "4", features = ["derive"] }`, used by the
`tools/`), so no root change is needed. Both lesson crates simply add,
after the `[lints]` section:

```toml
[dependencies]
clap = { workspace = true }
```

### Exercise stub (`exercises/src/lib.rs`)

The `Cli` struct ships **complete** (its `#[derive(Parser)]` + `#[arg]`
attributes are the worked clap example), and `parse`/`run` ship with
`todo!()` bodies. The `use clap::Parser;` import is used by the derive,
and `clap::Error` appears in `parse`'s signature, so there are no
unused-import warnings. The crate and its tests compile; the tests fail
at runtime with the `todo!()` panic.

```rust
//! Lesson 22 — exercises.
//!
//! Implement `parse` (warm-up) and `run` (main) so that `cargo test
//! --manifest-path lessons/22-cli/exercises/Cargo.toml` passes. The
//! `Cli` struct is given as a worked `clap` example. The tests live in
//! `tests/exercise.rs`.

use clap::Parser;

/// Greet someone a number of times.
#[derive(Parser, Debug, PartialEq, Eq)]
#[command(name = "greet")]
pub struct Cli {
    /// Who to greet.
    pub name: String,

    /// How many times to greet them.
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

pub fn parse(_args: &[&str]) -> Result<Cli, clap::Error> {
    todo!("parse the arguments with `Cli::try_parse_from`")
}

#[must_use]
pub fn run(_cli: &Cli) -> String {
    todo!("return \"Hello, <name>!\" repeated `count` times, joined by newlines")
}
```

### Warm-up: `parse`

Reference solution:

```rust
pub fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}
```

Pedagogical packing: the testable parsing seam. `try_parse_from(args)`
takes the arguments explicitly (the slice's first element is the program
name, as in real `argv`) and returns a `Result<Cli, clap::Error>` instead
of reading the real `argv` and exiting the process — which is exactly
what lets the tests run it in-process. (`&[&str]` works directly as the
argument iterator.) No `#[must_use]` (it returns a `Result`, already
`#[must_use]`).

Four tests:

```rust
#[test]
fn warmup_name_only() {
    let cli = parse(&["greet", "Alice"]).unwrap();
    assert_eq!(cli.name, "Alice");
    assert_eq!(cli.count, 1);
}

#[test]
fn warmup_with_count() {
    let cli = parse(&["greet", "Bob", "--count", "3"]).unwrap();
    assert_eq!(cli.count, 3);
}

#[test]
fn warmup_short_flag() {
    let cli = parse(&["greet", "Cara", "-c", "2"]).unwrap();
    assert_eq!(cli.count, 2);
}

#[test]
fn warmup_missing_name_errors() {
    assert!(parse(&["greet"]).is_err());
}
```

### Main: `run`

Reference solution:

```rust
#[must_use]
pub fn run(cli: &Cli) -> String {
    (0..cli.count)
        .map(|_| format!("Hello, {}!", cli.name))
        .collect::<Vec<_>>()
        .join("\n")
}
```

Pedagogical packing: the command logic as a pure function — read the
parsed fields, produce structured output. Greeting `count` times, joined
by newlines (reuses iterators from L11). Returning a `String` (rather
than printing) is what makes it directly testable; the real `main` would
`println!` it. Returns `String`, so `#[must_use]`.

Four tests (some build `Cli` directly — its fields are `pub` — and one
goes through `parse`):

```rust
#[test]
fn main_run_once() {
    let cli = Cli {
        name: "Alice".to_string(),
        count: 1,
    };
    assert_eq!(run(&cli), "Hello, Alice!");
}

#[test]
fn main_run_thrice() {
    let cli = Cli {
        name: "Bob".to_string(),
        count: 3,
    };
    assert_eq!(run(&cli), "Hello, Bob!\nHello, Bob!\nHello, Bob!");
}

#[test]
fn main_run_zero() {
    let cli = Cli {
        name: "X".to_string(),
        count: 0,
    };
    assert_eq!(run(&cli), "");
}

#[test]
fn main_run_via_parse() {
    let cli = parse(&["greet", "Dora", "-c", "2"]).unwrap();
    assert_eq!(run(&cli), "Hello, Dora!\nHello, Dora!");
}
```

**Eight tests total** (four warm-up + four main). The warm-up tests
exercise clap parsing via `try_parse_from` (program name + args); the
main tests assert the exact output string. Everything runs in-process —
no subprocess, no real `argv`. `Cli` derives `PartialEq, Eq` so it's
easy to assert on (and to satisfy `clippy::derive_partial_eq_without_eq`).

### Compile-fail: `22-exitcode-mismatch.rs`

Path: `exercises/compile_fails/22-exitcode-mismatch.rs`. A self-contained,
std-only file (the `compile-fails` tool type-checks with `rustc
--crate-type=lib --emit=metadata`, which can't link `clap`, so the
compile-fail uses only `std`). It declares `main` to return `ExitCode`
but returns a bare integer.

```rust
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
```

Pass condition: the student returns an `ExitCode` (e.g.
`ExitCode::SUCCESS`). rustc reports E0308 "expected `ExitCode`, found
integer" — verified during design. After the fix the file type-checks.

This is the lesson's centerpiece for exit codes: a CLI's `main` returns
an `ExitCode`, the typed way to tell the shell how the program ended.

## README structure

`lessons/22-cli/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - `#[derive(Parser)]` and `#[arg(...)]`
  - Parsing — `parse()` vs `try_parse_from()`
  - Testing a CLI
  - Exit codes — `ExitCode`
  - Structured output (stdout/stderr, machine-readable)
- **Exercises** — four subsections: Warm-up (`parse`), Main (`run`),
  Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`#[derive(Parser)]`" and "Parsing / Testing" sections are the heaviest —
they carry the testable-CLI idea.

## Lint expectations

Lesson 22's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design:

- `Cli` derives `Parser, Debug, PartialEq, Eq`. Deriving `Eq` alongside
  `PartialEq` satisfies `clippy::derive_partial_eq_without_eq` (pedantic).
- `parse` returns `Result<Cli, clap::Error>` directly via
  `Cli::try_parse_from(args)` — no `#[must_use]` (it returns a `Result`;
  `missing_errors_doc` is allowed in the workspace, so no `# Errors` doc
  is required).
- `run` returns `String`, so it carries `#[must_use]`; the
  `iter`/`map`/`collect`/`join` chain is clippy-clean.
- The **exercise stub** keeps `use clap::Parser;` (used by the derive)
  and references `clap::Error` in `parse`'s signature, so no unused
  imports; the `todo!()` bodies lint clean (verified).
- `clap` is added to the lesson crates as `clap = { workspace = true }`;
  it is already a workspace dependency with the `derive` feature.

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/22-cli/` exists with the four-part structure
- Both lesson `Cargo.toml`s declare `clap = { workspace = true }` (no
  root `Cargo.toml` change needed — `clap` is already a workspace dep)
- Cargo manifests use the correct package names (`cli-exercises`,
  `cli-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same `Cli`
  struct and `parse` / `run` signatures; the exercise ships `todo!()`
  bodies, the solution ships real bodies
- `cargo test --package cli-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/22-cli/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/22-cli`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/22-cli`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/22-cli/slides/index.html`
- `dist/index.html` lists lesson 22 as a clickable link (registry slug
  `cli` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/22-cli/slides/` returns 200

## Open questions

None.
