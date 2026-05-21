# Rust Course Infrastructure — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stand up the Rust training course infrastructure (Cargo workspace, Make-driven UX, three Rust tools, vendored reveal.js, CI) plus one fully written reference lesson (`01-hello-rust`) that exercises both the failing-test and compile-fail exercise modes.

**Architecture:** Cargo workspace at the repo root; every lesson's `exercises/` and `solutions/` are picked up by `members` globs. `default-members` is solutions + tools (so `cargo test` skips exercise stubs that would panic). A `Makefile` wraps Cargo for course-level UX (`make test`, `make verify`, `make new-lesson`, `make slides-dev`). Three small Rust binaries under `tools/` provide course operations: `new-lesson` (scaffold), `slides-dev` (serve reveal.js), and `compile-fails` (run `rustc` against compile-fail exercises in either `--expect broken` mode for CI or `--expect compiles` mode for student verification). Reveal.js is vendored under `shared/reveal/`. GitHub Actions runs `make ci` on stable + beta.

**Tech Stack:** Rust 2024 edition, Cargo workspaces, `clap`, `anyhow`, `toml_edit`, `walkdir`, `tempfile`, `tiny_http`, reveal.js (vendored), GitHub Actions, GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-21-rust-course-design.md`](../specs/2026-05-21-rust-course-design.md). The plan covers the "Initial implementation scope" section of the spec.

**Working directory for all tasks:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Initialize repo and base configuration files

**Files:**
- Create: `.gitignore`
- Create: `rust-toolchain.toml`
- Create: `rustfmt.toml`
- Create: `clippy.toml`
- Create: `Cargo.toml` (workspace root, empty `members`)

- [ ] **Step 1: Initialize git**

```bash
cd /Users/ristkari/code/private/rust-training
git init
```

Expected: `Initialized empty Git repository in .../rust-training/.git/`

- [ ] **Step 2: Create `.gitignore`**

```gitignore
/target
**/*.rs.bk
Cargo.lock
.DS_Store
/tmp
/.idea
/.vscode
```

(Cargo.lock is gitignored for a library/teaching workspace. We do not ship binaries from this repo.)

- [ ] **Step 3: Create `rust-toolchain.toml`**

```toml
[toolchain]
channel = "1.85"
components = ["rustfmt", "clippy", "rust-src"]
profile = "default"
```

- [ ] **Step 4: Create `rustfmt.toml`**

```toml
# Defaults only. No bikeshedding; pedagogy beats personal style.
edition = "2024"
```

- [ ] **Step 5: Create `clippy.toml`**

```toml
# Per-lint configuration goes here. Lints themselves are enforced in
# workspace-level [lints] in Cargo.toml.
msrv = "1.85"
```

- [ ] **Step 6: Create the workspace `Cargo.toml`**

`members` starts empty. Cargo rejects glob patterns that match nothing, so we phase the globs in: Task 3 changes `members` to `["tools/*"]` after the first tool exists, and Task 9 expands to the full glob set after the first lesson is scaffolded.

```toml
[workspace]
resolver = "3"
members = []

[workspace.package]
edition = "2024"
rust-version = "1.85"
license = "MIT OR Apache-2.0"
publish = false

[workspace.lints.rust]
rust_2018_idioms = { level = "deny", priority = -1 }
unused = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
# Lints that fight with teaching ownership/early Rust:
missing_errors_doc = "allow"
missing_panics_doc = "allow"
needless_pass_by_value = "allow"
module_name_repetitions = "allow"

[workspace.dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
toml_edit = "0.22"
walkdir = "2"
tempfile = "3"
tiny_http = "0.12"
```

- [ ] **Step 7: Verify the workspace parses**

```bash
cargo metadata --no-deps --format-version 1 > /dev/null
```

Expected: exit 0, no output.

- [ ] **Step 8: Commit**

```bash
git add .gitignore rust-toolchain.toml rustfmt.toml clippy.toml Cargo.toml
git commit -m "chore: bootstrap Cargo workspace and toolchain config"
```

---

## Task 2: Minimal Makefile with help target

**Files:**
- Create: `Makefile`

- [ ] **Step 1: Create `Makefile` (initial skeleton; later tasks extend it)**

```make
.DEFAULT_GOAL := help
SHELL := /bin/bash

# Cargo binaries inside the workspace; built once, reused everywhere.
NEW_LESSON   := cargo run --quiet --package new-lesson --
SLIDES_DEV   := cargo run --quiet --package slides-dev --
COMPILE_FAIL := cargo run --quiet --package compile-fails --

.PHONY: help
help: ## Show this help.
	@awk 'BEGIN {FS = ":.*##"; printf "Available targets:\n\n"} \
	      /^[a-zA-Z_-]+:.*##/ { printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
```

- [ ] **Step 2: Verify `make help` runs**

```bash
make help
```

Expected output includes the line `help                   Show this help.` (some terminal-color escape codes around it).

- [ ] **Step 3: Commit**

```bash
git add Makefile
git commit -m "chore: add Makefile skeleton with help target"
```

---

## Task 3: `compile-fails` tool — TDD

The tool walks a directory looking for any `.rs` file under a subdirectory literally named `compile_fails`. For each file it invokes `rustc` and checks the exit status against an expectation. The tool runs in two modes:

- `--expect broken` (default, used by `make test`): assert each file **fails** to compile. This is the author/CI check that exercises ship in their intended broken state.
- `--expect compiles`: assert each file **compiles** cleanly. This is the student check (`make verify LESSON=…`) — "did I fix it?".

**Files:**
- Create: `tools/compile-fails/Cargo.toml`
- Create: `tools/compile-fails/src/main.rs`
- Create: `tools/compile-fails/src/lib.rs`
- Create: `tools/compile-fails/tests/runner.rs`
- Modify: root `Cargo.toml` — switch `members = []` to `members = ["tools/*"]` so this and future tool crates are picked up by the glob.

- [ ] **Step 1: Create the crate manifest**

`tools/compile-fails/Cargo.toml`:

```toml
[package]
name = "compile-fails"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true

[dependencies]
anyhow = { workspace = true }
clap = { workspace = true }
walkdir = { workspace = true }
tempfile = { workspace = true }
```

- [ ] **Step 2: Create the library entry point (so tests can call internals)**

`tools/compile-fails/src/lib.rs`:

```rust
//! Walks a directory for `.rs` files under any subdirectory literally named
//! `compile_fails`, invokes `rustc` on each, and checks the exit code
//! against an expectation. Used by `make test` (expect: broken) and
//! `make verify` (expect: compiles).

use anyhow::{Context, Result, anyhow};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// What we expect from each compile-fail file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Expect {
    /// The file is supposed to *fail* to compile. Used by CI / `make test`
    /// to confirm exercises ship in their intended broken state.
    Broken,
    /// The file is supposed to compile cleanly. Used by students via
    /// `make verify LESSON=…` to confirm they fixed the exercise.
    Compiles,
}

/// Find every `.rs` file located inside a directory named `compile_fails`,
/// rooted at `root`. Results are sorted for deterministic output.
pub fn find_compile_fail_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        if path.components().any(|c| c.as_os_str() == "compile_fails") {
            out.push(path.to_path_buf());
        }
    }
    out.sort();
    Ok(out)
}

/// Invoke `rustc` on `path`. Returns `Ok(())` iff the compiler behaved
/// the way `expect` requires.
pub fn check_one(path: &Path, expect: Expect) -> Result<()> {
    let tmp = tempfile::NamedTempFile::new().context("create temp file")?;
    let status = Command::new("rustc")
        .args([
            "--edition=2024",
            "--crate-type=lib",
            "--emit=metadata",
            "-o",
        ])
        .arg(tmp.path())
        .arg(path)
        .status()
        .with_context(|| format!("invoking rustc on {}", path.display()))?;

    match (status.success(), expect) {
        (true,  Expect::Compiles) => Ok(()),
        (false, Expect::Broken)   => Ok(()),
        (true,  Expect::Broken) => Err(anyhow!(
            "file compiled, but was expected to fail: {}",
            path.display()
        )),
        (false, Expect::Compiles) => Err(anyhow!(
            "file did not compile, but was expected to: {}",
            path.display()
        )),
    }
}
```

- [ ] **Step 3: Create the binary entry point**

`tools/compile-fails/src/main.rs`:

```rust
use anyhow::{Result, anyhow};
use clap::{Parser, ValueEnum};
use compile_fails::{Expect, check_one, find_compile_fail_files};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Run rustc on every compile_fails/*.rs file under PATH")]
struct Cli {
    /// What each file should do.
    #[arg(long, value_enum, default_value_t = ExpectArg::Broken)]
    expect: ExpectArg,

    /// Directory to walk (default: lessons)
    #[arg(default_value = "lessons")]
    root: PathBuf,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ExpectArg {
    Broken,
    Compiles,
}

impl From<ExpectArg> for Expect {
    fn from(value: ExpectArg) -> Self {
        match value {
            ExpectArg::Broken => Expect::Broken,
            ExpectArg::Compiles => Expect::Compiles,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let expect: Expect = cli.expect.into();
    let files = find_compile_fail_files(&cli.root)?;

    if files.is_empty() {
        println!("no compile_fails/ files found under {}", cli.root.display());
        return Ok(());
    }

    let mut failed = 0;
    for file in &files {
        match check_one(file, expect) {
            Ok(()) => println!("ok:   {}", file.display()),
            Err(e) => {
                eprintln!("FAIL: {e}");
                failed += 1;
            }
        }
    }

    if failed == 0 {
        println!("\n{} compile-fail check(s) passed (expect={:?}).", files.len(), expect);
        Ok(())
    } else {
        Err(anyhow!("{failed} compile-fail check(s) did not match expect={:?}", expect))
    }
}
```

- [ ] **Step 4: Write the failing integration test**

`tools/compile-fails/tests/runner.rs`:

```rust
use std::fs;
use std::path::Path;

use compile_fails::{Expect, check_one, find_compile_fail_files};

fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

#[test]
fn finds_only_rs_files_under_compile_fails_dirs() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    write(&root.join("lessons/01/exercises/compile_fails/a.rs"), "fn main() {}");
    write(&root.join("lessons/01/exercises/compile_fails/notes.txt"), "ignore me");
    write(&root.join("lessons/01/exercises/src/lib.rs"), "// not in compile_fails/");

    let found = find_compile_fail_files(root).unwrap();
    let names: Vec<_> = found
        .iter()
        .map(|p| p.strip_prefix(root).unwrap().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, vec!["lessons/01/exercises/compile_fails/a.rs"]);
}

#[test]
fn expect_broken_passes_on_a_non_compiling_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("broken.rs");
    fs::write(&path, "fn main() { let x = 1; x = 2; let _ = x; }").unwrap();

    check_one(&path, Expect::Broken).expect("non-compiling file should satisfy Expect::Broken");
}

#[test]
fn expect_broken_fails_when_file_compiles() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("fine.rs");
    fs::write(&path, "pub fn add(a: i32, b: i32) -> i32 { a + b }").unwrap();

    let err = check_one(&path, Expect::Broken).expect_err("compiling file should violate Expect::Broken");
    assert!(err.to_string().contains("expected to fail"));
}

#[test]
fn expect_compiles_passes_on_a_compiling_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("fine.rs");
    fs::write(&path, "pub fn add(a: i32, b: i32) -> i32 { a + b }").unwrap();

    check_one(&path, Expect::Compiles).expect("compiling file should satisfy Expect::Compiles");
}

#[test]
fn expect_compiles_fails_on_a_non_compiling_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("broken.rs");
    fs::write(&path, "fn main() { let x = 1; x = 2; let _ = x; }").unwrap();

    let err = check_one(&path, Expect::Compiles).expect_err("non-compiling file should violate Expect::Compiles");
    assert!(err.to_string().contains("expected to"));
}
```

- [ ] **Step 5: Switch the root `Cargo.toml` to the tools glob**

Now that at least one tool crate exists, replace `members = []` with `members = ["tools/*"]`. This lets future tool crates (Tasks 4 and 5) be picked up automatically.

```toml
[workspace]
resolver = "3"
members = ["tools/*"]
```

Verify the workspace still parses:

```bash
cargo metadata --no-deps --format-version 1 > /dev/null
```

Expected: exit 0.

- [ ] **Step 6: Run the tests and verify they pass**

```bash
cargo test --package compile-fails
```

Expected: all 5 tests pass.

- [ ] **Step 7: Commit**

```bash
git add tools/compile-fails Cargo.toml
git commit -m "feat(tools): add compile-fails runner with broken/compiles modes"
```

---

## Task 4: `new-lesson` tool — TDD

Scaffolds `lessons/NN-name/{README.md, slides/, exercises/, solutions/}` from `templates/`. The root `Cargo.toml` does **not** need to be modified — the workspace `members = ["lessons/*/exercises", "lessons/*/solutions"]` glob picks the new crates up automatically. Templates do not exist yet (Task 8 writes them); for now the tests construct ad-hoc templates in tempdirs.

**Files:**
- Create: `tools/new-lesson/Cargo.toml`
- Create: `tools/new-lesson/src/lib.rs`
- Create: `tools/new-lesson/src/main.rs`
- Create: `tools/new-lesson/tests/scaffold.rs`

- [ ] **Step 1: Create the crate manifest**

`tools/new-lesson/Cargo.toml`:

```toml
[package]
name = "new-lesson"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true

[dependencies]
anyhow = { workspace = true }
clap = { workspace = true }
```

(`toml_edit` is no longer needed because we don't mutate the workspace manifest.)

- [ ] **Step 2: Write the library with scaffolding logic**

`tools/new-lesson/src/lib.rs`:

```rust
//! Scaffolds a new lesson directory by copying a templates tree, with
//! `{{LESSON_NAME}}` substitution and `.tmpl` suffix stripping.

use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;

/// Validate that `name` matches `NN-kebab-case` (two digits, dash, then
/// lowercase letters/digits/dashes, not ending with a dash).
pub fn validate_name(name: &str) -> Result<()> {
    let bytes = name.as_bytes();
    let valid_prefix = bytes.len() >= 4
        && bytes[0].is_ascii_digit()
        && bytes[1].is_ascii_digit()
        && bytes[2] == b'-';
    let valid_rest = bytes[3..]
        .iter()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-');
    let ok = valid_prefix && valid_rest && !name.ends_with('-');
    if !ok {
        bail!("lesson name must match NN-kebab-case (e.g. 07-ownership), got: {name}");
    }
    Ok(())
}

/// Compute the "bare" lesson name (everything after `NN-`) and the Rust
/// identifier form (dashes -> underscores). For `"01-hello-rust"` these
/// are `"hello-rust"` and `"hello_rust"`.
pub fn derive_bare_and_ident(name: &str) -> (String, String) {
    let bare = name.get(3..).unwrap_or(name).to_string(); // strip "NN-"
    let ident = bare.replace('-', "_");
    (bare, ident)
}

/// Copy every file under `templates`, substituting `{{LESSON_NAME}}`,
/// `{{LESSON_BARE}}`, and `{{LESSON_IDENT}}`, into `target`. The `.tmpl`
/// suffix is stripped from filenames.
pub fn scaffold(templates: &Path, target: &Path, name: &str) -> Result<()> {
    if target.exists() {
        bail!("target already exists: {}", target.display());
    }
    let (bare, ident) = derive_bare_and_ident(name);
    copy_dir(templates, target, name, &bare, &ident)
}

fn copy_dir(src: &Path, dst: &Path, name: &str, bare: &str, ident: &str) -> Result<()> {
    fs::create_dir_all(dst).with_context(|| format!("creating {}", dst.display()))?;
    for entry in fs::read_dir(src).with_context(|| format!("reading {}", src.display()))? {
        let entry = entry?;
        let src_path = entry.path();
        let file_name = entry.file_name();
        let stripped = file_name.to_string_lossy();
        let stripped = stripped.strip_suffix(".tmpl").unwrap_or(&stripped);
        let dst_path = dst.join(stripped);

        if entry.file_type()?.is_dir() {
            copy_dir(&src_path, &dst_path, name, bare, ident)?;
        } else {
            let content = fs::read_to_string(&src_path)
                .with_context(|| format!("reading {}", src_path.display()))?;
            let rendered = content
                .replace("{{LESSON_NAME}}", name)
                .replace("{{LESSON_BARE}}", bare)
                .replace("{{LESSON_IDENT}}", ident);
            fs::write(&dst_path, rendered)
                .with_context(|| format!("writing {}", dst_path.display()))?;
        }
    }
    Ok(())
}
```

- [ ] **Step 3: Write the binary entry point**

`tools/new-lesson/src/main.rs`:

```rust
use anyhow::Result;
use clap::Parser;
use new_lesson::{scaffold, validate_name};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Scaffold a new lesson under lessons/")]
struct Cli {
    /// Lesson name, e.g. "07-ownership"
    name: String,

    /// Repository root (default: current dir)
    #[arg(long, default_value = ".")]
    root: PathBuf,

    /// Templates directory (default: <root>/templates)
    #[arg(long)]
    templates: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    validate_name(&cli.name)?;

    let templates = cli.templates.unwrap_or_else(|| cli.root.join("templates"));
    let target = cli.root.join("lessons").join(&cli.name);

    scaffold(&templates, &target, &cli.name)?;

    println!("scaffolded {}", target.display());
    Ok(())
}
```

- [ ] **Step 4: Write the failing tests**

`tools/new-lesson/tests/scaffold.rs`:

```rust
use std::fs;
use std::path::Path;

use new_lesson::{derive_bare_and_ident, scaffold, validate_name};

fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

#[test]
fn validate_name_accepts_valid_kebab() {
    validate_name("01-hello-rust").unwrap();
    validate_name("99-x").unwrap();
}

#[test]
fn validate_name_rejects_invalid() {
    assert!(validate_name("hello").is_err());
    assert!(validate_name("1-x").is_err());      // single digit
    assert!(validate_name("01-").is_err());      // trailing dash
    assert!(validate_name("01-Hello").is_err()); // uppercase
}

#[test]
fn derive_bare_and_ident_strips_prefix_and_converts_dashes() {
    assert_eq!(derive_bare_and_ident("01-hello-rust"), ("hello-rust".to_string(), "hello_rust".to_string()));
    assert_eq!(derive_bare_and_ident("99-x"), ("x".to_string(), "x".to_string()));
}

#[test]
fn scaffold_substitutes_all_three_placeholders_and_strips_tmpl_suffix() {
    let dir = tempfile::tempdir().unwrap();
    let templates = dir.path().join("templates");
    let target = dir.path().join("lessons/01-hello-rust");

    write(&templates.join("README.md.tmpl"), "# {{LESSON_NAME}} ({{LESSON_BARE}})\n");
    write(
        &templates.join("exercises/Cargo.toml.tmpl"),
        "[package]\nname = \"{{LESSON_BARE}}-exercises\"\n",
    );
    write(
        &templates.join("exercises/tests/exercise.rs.tmpl"),
        "use {{LESSON_IDENT}}_exercises::greet;\n",
    );

    scaffold(&templates, &target, "01-hello-rust").unwrap();

    let readme = fs::read_to_string(target.join("README.md")).unwrap();
    assert_eq!(readme, "# 01-hello-rust (hello-rust)\n");
    let cargo = fs::read_to_string(target.join("exercises/Cargo.toml")).unwrap();
    assert!(cargo.contains("name = \"hello-rust-exercises\""));
    let test_file = fs::read_to_string(target.join("exercises/tests/exercise.rs")).unwrap();
    assert_eq!(test_file, "use hello_rust_exercises::greet;\n");
}

#[test]
fn scaffold_refuses_to_overwrite() {
    let dir = tempfile::tempdir().unwrap();
    let templates = dir.path().join("templates");
    let target = dir.path().join("lessons/01-x");
    write(&templates.join("a.tmpl"), "x");
    fs::create_dir_all(&target).unwrap();
    assert!(scaffold(&templates, &target, "01-x").is_err());
}
```

- [ ] **Step 5: Run tests and verify they pass**

```bash
cargo test --package new-lesson
```

Expected: all 5 tests pass. The `tools/*` glob picks the new crate up automatically.

- [ ] **Step 6: Commit**

```bash
git add tools/new-lesson
git commit -m "feat(tools): add new-lesson scaffolding tool with tests"
```

---

## Task 5: `slides-dev` tool — TDD

Tiny static-file server. Maps `/` to `lessons/<NAME>/slides/index.html`, maps `/shared/...` to `shared/...`, and rejects `..` in URLs. Uses `tiny_http` so we don't need a runtime.

**Files:**
- Create: `tools/slides-dev/Cargo.toml`
- Create: `tools/slides-dev/src/lib.rs`
- Create: `tools/slides-dev/src/main.rs`
- Create: `tools/slides-dev/tests/resolve.rs`

> No edit to root `Cargo.toml` needed — the `tools/*` glob picks the
> crate up automatically.

- [ ] **Step 1: Create the crate manifest**

`tools/slides-dev/Cargo.toml`:

```toml
[package]
name = "slides-dev"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true

[dependencies]
anyhow = { workspace = true }
clap = { workspace = true }
tiny_http = { workspace = true }
```

- [ ] **Step 2: Write the library**

`tools/slides-dev/src/lib.rs`:

```rust
//! Resolve incoming URLs to filesystem paths under either the lesson's
//! `slides/` directory or the repo-wide `shared/` directory.

use std::path::{Path, PathBuf};

/// Outcome of attempting to resolve a request URL.
#[derive(Debug, PartialEq, Eq)]
pub enum Resolved {
    /// Serve the file at this absolute path.
    File(PathBuf),
    /// Reject with HTTP 403 (path traversal attempt).
    Forbidden,
}

pub fn resolve(url: &str, slides_dir: &Path, shared_dir: &Path) -> Resolved {
    // Strip query string if any.
    let path_part = url.split('?').next().unwrap_or(url);
    if path_part.contains("..") {
        return Resolved::Forbidden;
    }

    if let Some(rest) = path_part.strip_prefix("/shared/") {
        return Resolved::File(shared_dir.join(rest));
    }

    let trimmed = path_part.trim_start_matches('/');
    let rel = if trimmed.is_empty() { "index.html" } else { trimmed };
    Resolved::File(slides_dir.join(rel))
}

pub fn mime_for(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("md") => "text/markdown; charset=utf-8",
        Some("js") => "application/javascript",
        Some("css") => "text/css",
        Some("json") => "application/json",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    }
}
```

- [ ] **Step 3: Write the binary**

`tools/slides-dev/src/main.rs`:

```rust
use anyhow::{Result, anyhow, bail};
use clap::Parser;
use slides_dev::{Resolved, mime_for, resolve};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Serve a lesson's reveal.js slides on localhost")]
struct Cli {
    /// Lesson name, e.g. "01-hello-rust"
    #[arg(long)]
    lesson: String,

    /// Repository root (default: current dir)
    #[arg(long, default_value = ".")]
    root: PathBuf,

    /// Port (default: 8000)
    #[arg(long, default_value_t = 8000)]
    port: u16,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let slides_dir = cli.root.join("lessons").join(&cli.lesson).join("slides");
    if !slides_dir.is_dir() {
        bail!("not a directory: {}", slides_dir.display());
    }
    let shared_dir = cli.root.join("shared");

    let addr = format!("0.0.0.0:{}", cli.port);
    let server = tiny_http::Server::http(&addr)
        .map_err(|e| anyhow!("failed to bind {addr}: {e}"))?;
    println!("serving {} on http://localhost:{}", slides_dir.display(), cli.port);

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        match resolve(&url, &slides_dir, &shared_dir) {
            Resolved::Forbidden => {
                let _ = request.respond(
                    tiny_http::Response::from_string("forbidden").with_status_code(403),
                );
            }
            Resolved::File(path) => match std::fs::read(&path) {
                Ok(bytes) => {
                    let mime = mime_for(&path);
                    let header = tiny_http::Header::from_bytes(b"Content-Type" as &[u8], mime)
                        .map_err(|()| anyhow!("invalid header"))?;
                    let _ = request.respond(tiny_http::Response::from_data(bytes).with_header(header));
                }
                Err(_) => {
                    let _ = request.respond(
                        tiny_http::Response::from_string("not found").with_status_code(404),
                    );
                }
            },
        }
    }
    Ok(())
}
```

- [ ] **Step 4: Write the failing tests**

`tools/slides-dev/tests/resolve.rs`:

```rust
use std::path::PathBuf;

use slides_dev::{Resolved, mime_for, resolve};

#[test]
fn root_maps_to_index_html() {
    let slides = PathBuf::from("/repo/lessons/01/slides");
    let shared = PathBuf::from("/repo/shared");
    assert_eq!(
        resolve("/", &slides, &shared),
        Resolved::File(slides.join("index.html"))
    );
}

#[test]
fn slides_md_maps_into_slides_dir() {
    let slides = PathBuf::from("/repo/lessons/01/slides");
    let shared = PathBuf::from("/repo/shared");
    assert_eq!(
        resolve("/slides.md", &slides, &shared),
        Resolved::File(slides.join("slides.md"))
    );
}

#[test]
fn shared_prefix_maps_into_shared_dir() {
    let slides = PathBuf::from("/repo/lessons/01/slides");
    let shared = PathBuf::from("/repo/shared");
    assert_eq!(
        resolve("/shared/reveal/reveal.js", &slides, &shared),
        Resolved::File(shared.join("reveal/reveal.js"))
    );
}

#[test]
fn dotdot_is_rejected() {
    let slides = PathBuf::from("/repo/lessons/01/slides");
    let shared = PathBuf::from("/repo/shared");
    assert_eq!(resolve("/../etc/passwd", &slides, &shared), Resolved::Forbidden);
    assert_eq!(
        resolve("/shared/../../etc/passwd", &slides, &shared),
        Resolved::Forbidden
    );
}

#[test]
fn query_strings_are_stripped() {
    let slides = PathBuf::from("/repo/lessons/01/slides");
    let shared = PathBuf::from("/repo/shared");
    assert_eq!(
        resolve("/slides.md?v=2", &slides, &shared),
        Resolved::File(slides.join("slides.md"))
    );
}

#[test]
fn mime_for_known_extensions() {
    use std::path::Path;
    assert_eq!(mime_for(Path::new("a.html")), "text/html; charset=utf-8");
    assert_eq!(mime_for(Path::new("a.css")), "text/css");
    assert_eq!(mime_for(Path::new("a.js")), "application/javascript");
    assert_eq!(mime_for(Path::new("a.unknown")), "application/octet-stream");
}
```

- [ ] **Step 5: Run tests and verify they pass**

```bash
cargo test --package slides-dev
```

Expected: all 6 tests pass.

- [ ] **Step 6: Run the whole workspace**

```bash
cargo test --workspace
```

Expected: 16 tests pass across the three tools (5 compile-fails + 5 new-lesson + 6 slides-dev).

- [ ] **Step 7: Commit**

```bash
git add tools/slides-dev
git commit -m "feat(tools): add slides-dev static server with URL resolution"
```

---

## Task 6: Vendor reveal.js and create the Rust theme

We vendor a pinned reveal.js release rather than depending on a CDN, so the course works fully offline.

**Files:**
- Create: `shared/reveal/README.md`
- Create: `shared/reveal/dist/reveal.css` (from upstream)
- Create: `shared/reveal/dist/reveal.js` (from upstream)
- Create: `shared/reveal/dist/theme/black.css` (from upstream — fallback base)
- Create: `shared/reveal/plugin/...` (highlight, markdown, notes, search — from upstream)
- Create: `shared/reveal/theme/rust.css` (custom)

- [ ] **Step 1: Download a pinned reveal.js release**

```bash
mkdir -p shared/reveal
curl -L -o /tmp/reveal.tar.gz https://github.com/hakimel/reveal.js/archive/refs/tags/5.1.0.tar.gz
tar -xzf /tmp/reveal.tar.gz -C /tmp
cp -R /tmp/reveal.js-5.1.0/dist shared/reveal/dist
cp -R /tmp/reveal.js-5.1.0/plugin shared/reveal/plugin
rm -rf /tmp/reveal.js-5.1.0 /tmp/reveal.tar.gz
```

Expected: `shared/reveal/dist/reveal.js` and `shared/reveal/plugin/highlight/highlight.js` exist.

- [ ] **Step 2: Create the Rust theme**

`shared/reveal/theme/rust.css`:

```css
/* Rust-flavored reveal.js theme. Builds on the black theme by overriding
   accent colors. Do not edit shared/reveal/dist/ directly; edit this file. */
@import url("../dist/theme/black.css");

:root {
  --r-link-color: #d28445;
  --r-link-color-hover: #f1a76b;
  --r-selection-background-color: #b7410e;
  --r-heading-color: #f4e4c1;
  --r-main-color: #f4e4c1;
}

.reveal a {
  color: var(--r-link-color);
}

.reveal pre code {
  background: #1a1a1a;
  border-left: 4px solid #b7410e;
  padding: 0.6em 0.8em;
}

.reveal .progress {
  color: #b7410e;
}
```

- [ ] **Step 3: Document the vendoring**

`shared/reveal/README.md`:

```markdown
# Vendored reveal.js

This directory is a verbatim copy of [reveal.js](https://github.com/hakimel/reveal.js)
release `5.1.0`, plus a custom theme at `theme/rust.css`.

**Do not edit `dist/` or `plugin/` by hand.** To upgrade reveal.js, replace
the contents of those directories from a fresh release tarball; the custom
theme lives outside them and survives.
```

- [ ] **Step 4: Commit**

```bash
git add shared/
git commit -m "chore: vendor reveal.js 5.1.0 and add rust theme"
```

---

## Task 7: Extend the Makefile with operational targets

`make test` is the author-and-CI command: it must be green at all times. It builds everything (so exercises stay compileable), runs the `default-members` tests (tools + solutions, *not* exercises whose stubs panic), and asserts the compile-fail exercises ship in their intended broken state.

`make verify LESSON=…` is the student command: it runs the exercise crate's tests and checks that any compile-fail files now compile.

**Files:**
- Modify: `Makefile`

- [ ] **Step 1: Add the remaining targets to `Makefile`**

Append after the `help` target:

```make
.PHONY: build
build: ## Compile every crate in the workspace, including exercise stubs.
	cargo build --workspace --all-targets

.PHONY: test
test: build ## CI test: tools + solutions pass; exercises ship broken.
	cargo test
	$(COMPILE_FAIL) --expect broken lessons

.PHONY: lint
lint: ## Run clippy and rustfmt --check across the workspace.
	cargo clippy --workspace --all-targets -- -D warnings
	cargo fmt --all --check

.PHONY: fmt
fmt: ## Format the workspace.
	cargo fmt --all

.PHONY: ci
ci: lint test ## Run the full local CI sequence.

.PHONY: new-lesson
new-lesson: ## Scaffold a new lesson. Usage: make new-lesson NAME=NN-name
	@if [ -z "$(NAME)" ]; then echo "NAME is required (e.g. make new-lesson NAME=07-ownership)"; exit 2; fi
	$(NEW_LESSON) $(NAME)

.PHONY: slides-dev
slides-dev: ## Serve a lesson's slides on http://localhost:8000. Usage: make slides-dev LESSON=NN-name
	@if [ -z "$(LESSON)" ]; then echo "LESSON is required (e.g. make slides-dev LESSON=01-hello-rust)"; exit 2; fi
	$(SLIDES_DEV) --lesson $(LESSON)

.PHONY: verify
verify: ## Student check: run a lesson's exercise tests + compile-fail compiles. Usage: make verify LESSON=NN-name
	@if [ -z "$(LESSON)" ]; then echo "LESSON is required (e.g. make verify LESSON=01-hello-rust)"; exit 2; fi
	cargo test --manifest-path lessons/$(LESSON)/exercises/Cargo.toml
	@if [ -d "lessons/$(LESSON)/exercises/compile_fails" ]; then \
		$(COMPILE_FAIL) --expect compiles lessons/$(LESSON); \
	fi
```

- [ ] **Step 2: Verify lint runs cleanly on the current workspace**

```bash
make lint
```

Expected: exit 0.

- [ ] **Step 3: Verify test target works (no lessons yet, so compile-fail says "no files found")**

```bash
make test
```

Expected: all tests pass; final line from compile-fails says `no compile_fails/ files found under lessons`.

- [ ] **Step 4: Verify the NAME-required guard works**

```bash
make new-lesson
```

Expected: exit code 2, message `NAME is required (...)`.

- [ ] **Step 5: Commit**

```bash
git add Makefile
git commit -m "chore: wire up make build/test/lint/fmt/ci/verify/new-lesson/slides-dev"
```

---

## Task 8: Lesson templates

The `new-lesson` tool reads from these. Each `.tmpl` substitutes three placeholders and has its `.tmpl` suffix stripped on copy:

- `{{LESSON_NAME}}` — full lesson name including the `NN-` prefix, e.g. `01-hello-rust`. Used in directory paths, document titles.
- `{{LESSON_BARE}}` — name without the `NN-` prefix, e.g. `hello-rust`. Used in **Cargo package names** (which can include digits but it's still clearer not to).
- `{{LESSON_IDENT}}` — Rust identifier form (dashes → underscores), e.g. `hello_rust`. Used in `use` statements (Rust identifiers cannot start with a digit nor contain dashes).

**Files:**
- Create: `templates/README.md.tmpl`
- Create: `templates/slides/index.html.tmpl`
- Create: `templates/slides/slides.md.tmpl`
- Create: `templates/exercises/Cargo.toml.tmpl`
- Create: `templates/exercises/src/lib.rs.tmpl`
- Create: `templates/exercises/tests/exercise.rs.tmpl`
- Create: `templates/solutions/Cargo.toml.tmpl`
- Create: `templates/solutions/src/lib.rs.tmpl`
- Create: `templates/solutions/tests/exercise.rs.tmpl`

- [ ] **Step 1: `templates/README.md.tmpl`**

```markdown
# Lesson {{LESSON_NAME}}

> One-sentence description of what this lesson teaches.

## Learning goals

- Goal 1
- Goal 2

## Self-study notes

Replace this paragraph with the lesson's prose. Cover the concepts at
roughly the depth of a 30-minute read. Code samples are encouraged.

## Exercises

Open `exercises/src/lib.rs` and make the tests pass. From the repo root:

```bash
make verify LESSON={{LESSON_NAME}}
```

If this lesson has compile-fail exercises, they live in
`exercises/compile_fails/`. Each `.rs` file ships in a state that does
*not* compile; your job is to make it compile. `make verify` runs both
the test suite and the compile-fail checks.

## Solutions

See `solutions/src/lib.rs` for the reference implementation. Try the
exercises before peeking.
```

- [ ] **Step 2: `templates/slides/index.html.tmpl`**

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>{{LESSON_NAME}}</title>
    <link rel="stylesheet" href="/shared/reveal/dist/reveal.css" />
    <link rel="stylesheet" href="/shared/reveal/theme/rust.css" />
    <link rel="stylesheet" href="/shared/reveal/plugin/highlight/monokai.css" />
  </head>
  <body>
    <div class="reveal">
      <div class="slides">
        <section data-markdown="slides.md" data-separator="^\n---\n$" data-separator-vertical="^\n--\n$"></section>
      </div>
    </div>
    <script src="/shared/reveal/dist/reveal.js"></script>
    <script src="/shared/reveal/plugin/markdown/markdown.js"></script>
    <script src="/shared/reveal/plugin/highlight/highlight.js"></script>
    <script src="/shared/reveal/plugin/notes/notes.js"></script>
    <script>
      Reveal.initialize({
        hash: true,
        slideNumber: "c/t",
        plugins: [RevealMarkdown, RevealHighlight, RevealNotes],
      });
    </script>
  </body>
</html>
```

- [ ] **Step 3: `templates/slides/slides.md.tmpl`**

```markdown
# {{LESSON_NAME}}

One-sentence hook.

---

## Goal

What this lesson achieves.

---

## Concept

Explain the core concept here.

```rust
fn example() {
    println!("replace me");
}
```

---

## Wrap

- Key takeaway 1
- Key takeaway 2
- Next: lesson NN+1
```

- [ ] **Step 4: `templates/exercises/Cargo.toml.tmpl`**

```toml
[package]
name = "{{LESSON_BARE}}-exercises"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true
```

- [ ] **Step 5: `templates/exercises/src/lib.rs.tmpl`**

```rust
//! Exercises for lesson {{LESSON_NAME}}.
//! Make the tests in `tests/exercise.rs` pass.

#[must_use]
pub fn add(_a: i32, _b: i32) -> i32 {
    todo!("implement add")
}
```

(The `#[must_use]` attribute is required to satisfy the workspace's `clippy::pedantic` policy when `make lint` lints exercise crates via `cargo clippy --workspace`.)

- [ ] **Step 6: `templates/exercises/tests/exercise.rs.tmpl`**

```rust
use {{LESSON_IDENT}}_exercises::add;

#[test]
fn adds_positives() {
    assert_eq!(add(2, 3), 5);
}
```

- [ ] **Step 7: `templates/solutions/Cargo.toml.tmpl`**

```toml
[package]
name = "{{LESSON_BARE}}-solutions"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true
```

- [ ] **Step 8: `templates/solutions/src/lib.rs.tmpl`**

```rust
//! Reference solution for lesson {{LESSON_NAME}}.

#[must_use]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- [ ] **Step 9: `templates/solutions/tests/exercise.rs.tmpl`**

```rust
use {{LESSON_IDENT}}_solutions::add;

#[test]
fn adds_positives() {
    assert_eq!(add(2, 3), 5);
}
```

- [ ] **Step 10: Commit**

```bash
git add templates/
git commit -m "feat: add lesson templates for new-lesson scaffolding"
```

---

## Task 9: Scaffold the reference lesson `01-hello-rust`

Use the tool we just built to generate the lesson skeleton, then expand the workspace `members` to include lesson globs (now that at least one lesson exists). Task 10 then replaces the placeholder content with real lesson 01 material.

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=01-hello-rust
```

Expected: `scaffolded lessons/01-hello-rust` printed; `lessons/01-hello-rust/{README.md, slides/, exercises/, solutions/}` created. The root `Cargo.toml` is unchanged at this step.

- [ ] **Step 2: Expand the root `Cargo.toml` to use lesson globs and `default-members`**

Now that `lessons/01-hello-rust/exercises` and `lessons/01-hello-rust/solutions` exist (each with a `Cargo.toml` from the templates), the lesson globs have something to match. Update the root `Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = ["tools/*", "lessons/*/exercises", "lessons/*/solutions"]
default-members = ["tools/*", "lessons/*/solutions"]
```

`default-members` selects what `cargo test` (without `--workspace`) operates on: tools + solutions only. Exercises stay out of the default set so their `todo!()`-shaped stubs don't panic during `make test`.

- [ ] **Step 3: Verify the workspace picked up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"[^"]*"' | sort -u
```

Expected output includes both `"name":"hello-rust-exercises"` and `"name":"hello-rust-solutions"`.

- [ ] **Step 4: Verify workspace builds**

```bash
cargo build --workspace
```

Expected: warning-free build. Templates produce valid Rust because the stub uses `todo!()` and the parameters use `_`-prefix underscores.

- [ ] **Step 5: Commit the scaffold and the workspace expansion**

```bash
git add lessons/01-hello-rust Cargo.toml
git commit -m "chore: scaffold lessons/01-hello-rust and expand workspace globs"
```

---

## Task 10: Write lesson 01 content

Replace the scaffolded placeholders with real lesson material. The lesson teaches Cargo basics, `fn main`, `println!`, and as a deliberate beginner exposure: one immutable-binding compile-fail exercise.

**Files (overwrite):**
- `lessons/01-hello-rust/README.md`
- `lessons/01-hello-rust/slides/slides.md`
- `lessons/01-hello-rust/exercises/src/lib.rs`
- `lessons/01-hello-rust/exercises/tests/exercise.rs`
- `lessons/01-hello-rust/solutions/src/lib.rs`
- `lessons/01-hello-rust/solutions/tests/exercise.rs`
- Create: `lessons/01-hello-rust/exercises/compile_fails/01-immutable-binding.rs`

> Cargo manifests (`exercises/Cargo.toml` and `solutions/Cargo.toml`) are
> already correct from the scaffold — `hello-rust-exercises` and
> `hello-rust-solutions` — and do not need to be edited.

- [ ] **Step 1: Overwrite `lessons/01-hello-rust/README.md`**

```markdown
# Lesson 01 — Hello, Rust

Your first Rust program. By the end you will have installed the
toolchain, run a binary built by `cargo`, and met your first compiler
error.

## Learning goals

- Install `rustup` and verify `cargo`, `rustc`, and `rustfmt` work
- Read a minimal `fn main()` and understand the `println!` macro
- Write and call a small library function (`greet`)
- Experience a deliberate compile error and read the diagnostic

## Self-study notes

### The toolchain

Rust ships through `rustup`, a version manager that installs `cargo`
(build tool), `rustc` (compiler), `rustfmt` (formatter), and `clippy`
(linter). Install it from <https://rustup.rs> and then run:

```bash
rustc --version
cargo --version
```

You should see version `1.85` or newer.

### The shape of a program

A Rust binary's entry point is `fn main()`. A Rust library exposes
functions other crates can call. This lesson's exercise asks you to write
a library function called `greet`. The tests already know how `greet`
should behave; your job is to make them stop failing.

### Macros vs functions

`println!` is a macro, not a function. The `!` is the giveaway. For now
treat it as "print this and a newline." We will explore macros properly
later in the course.

## Exercises

### Failing tests

Open `exercises/src/lib.rs` and implement `greet` so that the tests pass:

```bash
cargo test --package hello-rust-exercises
```

### Compile-fail exercise

Open `exercises/compile_fails/01-immutable-binding.rs`. The file does
**not** compile — read the comment, then fix it. The check passes once
the file no longer compiles cleanly *and* it makes the change the comment
asks for (which is the same thing here: an immutable binding cannot be
reassigned).

Verify:

```bash
cargo run --package compile-fails -- lessons/01-hello-rust
```

The runner reports `ok` when the file still fails to compile, and
`FAIL` when it compiles cleanly.

## Solutions

See `solutions/src/lib.rs` for the reference implementation. Try first.
```

- [ ] **Step 2: Overwrite `lessons/01-hello-rust/slides/slides.md`**

```markdown
# Hello, Rust

Your first compile error is a feature, not a bug.

---

## Why Rust

- Memory safety without a garbage collector
- Fearless concurrency
- A compiler that teaches you

---

## The toolchain

`rustup` installs:

- `cargo` — build tool and package manager
- `rustc` — the compiler
- `rustfmt` — formatter (defaults are correct)
- `clippy` — linter that catches common mistakes

```bash
rustup show
cargo --version
```

---

## Your first program

```rust
fn main() {
    println!("Hello, Rust!");
}
```

- `fn main()` is the entry point
- `println!` is a macro (note the `!`)

---

## Your first library function

```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

- `pub` makes it visible outside the crate
- `&str` is a borrowed string slice (more soon)
- `String` is an owned, heap-allocated string

---

## Compile errors are pedagogy

```rust
let x = 1;
x = 2; // ERROR: cannot assign twice to immutable variable
```

Read the error. The compiler usually tells you the fix.

---

## Wrap

- Installed the toolchain, ran `cargo`
- Wrote a function, made tests pass
- Met our first borrow-checker-adjacent error
- Next: variables, mutability, and shadowing
```

- [ ] **Step 3: Overwrite `lessons/01-hello-rust/exercises/src/lib.rs`**

```rust
//! Lesson 01 — exercises.
//!
//! Implement `greet` so that `cargo test --package hello-rust-exercises`
//! passes. The tests live in `tests/exercise.rs`.

#[must_use]
pub fn greet(_name: &str) -> String {
    todo!("return a greeting like \"Hello, <name>!\"")
}
```

- [ ] **Step 4: Overwrite `lessons/01-hello-rust/exercises/tests/exercise.rs`**

```rust
use hello_rust_exercises::greet;

#[test]
fn greets_by_name() {
    assert_eq!(greet("Aki"), "Hello, Aki!");
}

#[test]
fn greets_with_punctuation_intact() {
    assert_eq!(greet("world"), "Hello, world!");
}
```

- [ ] **Step 5: Overwrite `lessons/01-hello-rust/solutions/src/lib.rs`**

```rust
//! Lesson 01 — reference solution.

#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

- [ ] **Step 6: Overwrite `lessons/01-hello-rust/solutions/tests/exercise.rs`**

```rust
use hello_rust_solutions::greet;

#[test]
fn greets_by_name() {
    assert_eq!(greet("Aki"), "Hello, Aki!");
}

#[test]
fn greets_with_punctuation_intact() {
    assert_eq!(greet("world"), "Hello, world!");
}
```

- [ ] **Step 7: Create `lessons/01-hello-rust/exercises/compile_fails/01-immutable-binding.rs`**

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Task: make this file fail to compile because the code mutates an
// immutable binding. (When you "fix" the program, the fix is to make `x`
// mutable. But the goal of this exercise is to leave it broken and
// understand WHY it's broken. Read the error rustc gives you.)

fn main() {
    let x = 1;
    x = 2;
    println!("{x}");
}
```

- [ ] **Step 8: Verify failing tests fail (the exercise is undone)**

```bash
cargo test --manifest-path lessons/01-hello-rust/exercises/Cargo.toml
```

Expected: tests fail with a `todo!` panic. This is the intentional state — students fix it.

- [ ] **Step 9: Verify solutions tests pass**

```bash
cargo test --package hello-rust-solutions
```

Expected: 2 tests pass.

- [ ] **Step 10: Verify compile-fail exercise ships broken (the author/CI check)**

```bash
cargo run --package compile-fails -- --expect broken lessons/01-hello-rust
```

Expected: `ok: lessons/01-hello-rust/exercises/compile_fails/01-immutable-binding.rs` and exit 0.

- [ ] **Step 11: Verify `make verify` would catch a student who hasn't fixed it yet**

```bash
cargo run --package compile-fails -- --expect compiles lessons/01-hello-rust
```

Expected: non-zero exit and `FAIL: ... expected to: lessons/01-hello-rust/...`. (This confirms the student-mode check is wired correctly; we leave the exercise broken since that's its shipped state.)

- [ ] **Step 12: Verify `make test` is green now that lesson 01 exists**

```bash
make test
```

Expected: exit 0. Tools' tests pass, solutions' tests pass (4 tests across both tools and lesson 01 solutions), compile-fails reports `ok` for the one shipped-broken file.

- [ ] **Step 13: Verify lint passes**

```bash
make lint
```

Expected: exit 0.

- [ ] **Step 14: Commit**

```bash
git add lessons/01-hello-rust
git commit -m "feat(lesson-01): write hello-rust content with compile-fail exercise"
```

---

## Task 11: GitHub Actions CI

**Files:**
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: Create the workflow**

`.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -D warnings

jobs:
  ci:
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        toolchain: [stable, beta]
    steps:
      - uses: actions/checkout@v4

      - name: Install toolchain (${{ matrix.toolchain }})
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ matrix.toolchain }}
          components: clippy, rustfmt

      - name: Cache cargo registry, index, and target
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ matrix.toolchain }}-${{ hashFiles('**/Cargo.toml') }}
          restore-keys: |
            ${{ runner.os }}-cargo-${{ matrix.toolchain }}-

      - name: make ci
        run: make ci
```

> **Note for the implementer:** the workflow pins to `dtolnay/rust-toolchain@stable` (the action's tag) but then asks it to install the toolchain from the matrix value. The `rust-toolchain.toml` file in the repo will further pin to channel `1.85` for *local* development; CI deliberately ignores that file and runs against `stable` + `beta` to catch upcoming breakage. If `rust-toolchain.toml` interferes, add `RUSTUP_TOOLCHAIN: ${{ matrix.toolchain }}` to `env`.

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add GitHub Actions workflow running make ci on stable + beta"
```

---

## Task 12: Rewrite the top-level README

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Overwrite `README.md`**

```markdown
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
```

- [ ] **Step 2: Commit**

```bash
git add README.md
git commit -m "docs: rewrite README for Rust course"
```

---

## Task 13: End-to-end verification

Wipe build artifacts, exercise the full developer loop, then clean up.

- [ ] **Step 1: Clean and run full CI locally**

```bash
cargo clean
make ci
```

Expected: green. Total time depends on your machine; expect 30–90 seconds on cold cache.

- [ ] **Step 2: Scaffold a throwaway lesson and verify it builds**

```bash
make new-lesson NAME=99-demo
cargo build --workspace
```

Expected: workspace compiles; `lessons/99-demo/{README.md, slides/, exercises/, solutions/}` exist; root `Cargo.toml` lists both new crates.

- [ ] **Step 3: Verify slides-dev binds and serves**

```bash
( make slides-dev LESSON=99-demo & ) && sleep 1 && curl -sS -o /dev/null -w "%{http_code}\n" http://localhost:8000/ && pkill -f "slides-dev"
```

Expected: `200`.

- [ ] **Step 4: Remove the throwaway lesson**

```bash
rm -rf lessons/99-demo
```

No `Cargo.toml` edit needed — the glob will simply stop matching the deleted directory.

Verify:

```bash
cargo metadata --no-deps --format-version 1 > /dev/null
make ci
```

Expected: both succeed.

- [ ] **Step 5: Final commit (if any uncommitted changes from cleanup)**

```bash
git status
# If clean, no commit needed.
# If not clean (e.g. Cargo.lock changes), commit:
git add -A
git commit -m "chore: end-to-end verification cleanup"
```

---

## Done criteria

- `make help`, `make build`, `make test`, `make lint`, `make fmt`, `make ci`, `make new-lesson`, `make slides-dev`, and `make verify` all work locally.
- `make test` is green on a fresh clone (16 tool tests + 2 solution tests + compile-fail `expect=broken` succeeds for lesson 01).
- `cargo run --package compile-fails -- --expect compiles lessons/01-hello-rust` returns non-zero (confirms the student-mode check fails when the student hasn't done the work yet).
- GitHub Actions `ci` workflow runs `make ci` on stable + beta.
- Lesson 01 is fully written with README, slides, exercises (failing-test + compile-fail), and solutions.
- Top-level `README.md` reflects the Rust course.
