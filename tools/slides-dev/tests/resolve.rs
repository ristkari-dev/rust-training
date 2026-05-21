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
    assert_eq!(
        resolve("/../etc/passwd", &slides, &shared),
        Resolved::Forbidden
    );
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
