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

#[must_use]
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
    let rel = if trimmed.is_empty() {
        "index.html"
    } else {
        trimmed
    };
    Resolved::File(slides_dir.join(rel))
}

#[must_use]
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
