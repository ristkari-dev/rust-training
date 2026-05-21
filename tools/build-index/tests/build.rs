use std::fs;
use std::path::Path;

use build_index::{ALL_LESSONS, build, collect_published, render_index};

fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

#[test]
fn lesson_master_list_has_31_lessons_across_8_phases() {
    assert_eq!(ALL_LESSONS.len(), 31);
    let mut phases: Vec<u8> = ALL_LESSONS.iter().map(|l| l.phase).collect();
    phases.sort_unstable();
    phases.dedup();
    assert_eq!(phases, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn collect_published_finds_lessons_with_slides_subdir() {
    let dir = tempfile::tempdir().unwrap();
    write(
        &dir.path().join("lessons/01-hello-rust/slides/index.html"),
        "",
    );
    write(
        &dir.path().join("lessons/02-variables/README.md"),
        "no slides/",
    );
    fs::create_dir_all(dir.path().join("lessons/junk-no-slides")).unwrap();

    let published = collect_published(&dir.path().join("lessons")).unwrap();
    assert!(published.contains("01-hello-rust"));
    assert!(!published.contains("02-variables"));
    assert!(!published.contains("junk-no-slides"));
}

#[test]
fn collect_published_returns_empty_when_lessons_dir_missing() {
    let dir = tempfile::tempdir().unwrap();
    let published = collect_published(&dir.path().join("nonexistent")).unwrap();
    assert!(published.is_empty());
}

#[test]
fn render_index_marks_published_lessons_as_links() {
    let mut published = std::collections::HashSet::new();
    published.insert("01-hello-rust".to_string());

    let html = render_index(&published);
    // Lesson 01 must be a clickable anchor
    assert!(html.contains(r#"<a class="lesson" href="lessons/01-hello-rust/slides/">"#));
    // Lesson 02 must be a future placeholder div
    assert!(html.contains(r#"<div class="lesson future" aria-disabled="true">"#));
    // Both must mention their titles
    assert!(html.contains("Hello, Rust"));
    assert!(html.contains("Variables, types, mutability"));
    // Phase headers must appear
    assert!(html.contains("Phase 1 · Programming 101 in Rust"));
    assert!(html.contains("Phase 8 · Distributed patterns"));
    // Title and meta
    assert!(html.contains("<title>Rust Training</title>"));
}

#[test]
fn build_produces_index_and_copies_published_lessons() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    // Set up a fake lesson 01 with one slide file
    write(
        &root.join("lessons/01-hello-rust/slides/index.html"),
        "<!doctype html><title>01</title>",
    );
    write(&root.join("lessons/01-hello-rust/slides/slides.md"), "# 01");
    // And a shared/reveal with one file
    write(&root.join("shared/reveal/dist/reveal.css"), "/* fake */");

    let out = root.join("dist");
    build(&root.join("lessons"), &root.join("shared/reveal"), &out).unwrap();

    assert!(out.join("index.html").exists());
    assert!(out.join("lessons/01-hello-rust/slides/index.html").exists());
    assert!(out.join("lessons/01-hello-rust/slides/slides.md").exists());
    assert!(out.join("shared/reveal/dist/reveal.css").exists());

    let index = fs::read_to_string(out.join("index.html")).unwrap();
    assert!(index.contains("<title>Rust Training</title>"));
}

#[test]
fn build_overwrites_existing_out_dir() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(&root.join("lessons/01-hello-rust/slides/index.html"), "v1");
    // Pre-populate with stale content
    write(&root.join("dist/old-stale-file.txt"), "stale");

    build(
        &root.join("lessons"),
        &root.join("shared/reveal"),
        &root.join("dist"),
    )
    .unwrap();

    assert!(
        !root.join("dist/old-stale-file.txt").exists(),
        "stale file should be removed"
    );
    assert!(root.join("dist/index.html").exists());
}
