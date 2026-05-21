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
    assert!(validate_name("1-x").is_err()); // single digit
    assert!(validate_name("01-").is_err()); // trailing dash
    assert!(validate_name("01-Hello").is_err()); // uppercase
}

#[test]
fn derive_bare_and_ident_strips_prefix_and_converts_dashes() {
    assert_eq!(
        derive_bare_and_ident("01-hello-rust"),
        ("hello-rust".to_string(), "hello_rust".to_string())
    );
    assert_eq!(
        derive_bare_and_ident("99-x"),
        ("x".to_string(), "x".to_string())
    );
}

#[test]
fn scaffold_substitutes_all_three_placeholders_and_strips_tmpl_suffix() {
    let dir = tempfile::tempdir().unwrap();
    let templates = dir.path().join("templates");
    let target = dir.path().join("lessons/01-hello-rust");

    write(
        &templates.join("README.md.tmpl"),
        "# {{LESSON_NAME}} ({{LESSON_BARE}})\n",
    );
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
