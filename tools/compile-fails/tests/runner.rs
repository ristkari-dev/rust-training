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

    write(
        &root.join("lessons/01/exercises/compile_fails/a.rs"),
        "fn main() {}",
    );
    write(
        &root.join("lessons/01/exercises/compile_fails/notes.txt"),
        "ignore me",
    );
    write(
        &root.join("lessons/01/exercises/src/lib.rs"),
        "// not in compile_fails/",
    );

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

    let err =
        check_one(&path, Expect::Broken).expect_err("compiling file should violate Expect::Broken");
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

    let err = check_one(&path, Expect::Compiles)
        .expect_err("non-compiling file should violate Expect::Compiles");
    assert!(err.to_string().contains("expected to"));
}
