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
#[must_use]
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
