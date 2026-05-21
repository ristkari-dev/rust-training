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
// The match below has four explicit arms (one per (success, expect)
// combination) for pedagogical clarity — collapsing the two `Ok(())`
// arms with `|` would obscure the symmetry.
#[allow(clippy::match_same_arms)]
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
        (true, Expect::Compiles) => Ok(()),
        (false, Expect::Broken) => Ok(()),
        (true, Expect::Broken) => Err(anyhow!(
            "file compiled, but was expected to fail: {}",
            path.display()
        )),
        (false, Expect::Compiles) => Err(anyhow!(
            "file did not compile, but was expected to: {}",
            path.display()
        )),
    }
}
