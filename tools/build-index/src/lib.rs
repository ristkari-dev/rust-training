//! Static-site generator for the Rust training course.
//!
//! Produces a `dist/` directory containing a landing page that links to each
//! published lesson's reveal.js slide deck, plus copies of every published
//! lesson's `slides/` directory and the shared reveal.js asset tree.

use std::collections::HashSet;
use std::fmt::Write as _;
use std::fs;
use std::hash::BuildHasher;
use std::path::Path;

use anyhow::{Context, Result};
use walkdir::WalkDir;

/// Metadata for a single lesson in the master course list.
#[derive(Debug, Clone, Copy)]
pub struct LessonInfo {
    pub number: &'static str,
    pub slug: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
    pub phase: u8,
}

impl LessonInfo {
    /// Full lesson directory name, e.g. `"01-hello-rust"`.
    #[must_use]
    pub fn dir_name(&self) -> String {
        format!("{}-{}", self.number, self.slug)
    }
}

pub const PHASES: &[(u8, &str)] = &[
    (1, "Programming 101 in Rust"),
    (2, "Ownership deep dive"),
    (3, "Abstraction"),
    (4, "Concurrency"),
    (5, "Systems programming"),
    (6, "Production services"),
    (7, "Tooling & quality"),
    (8, "Distributed patterns"),
];

pub const ALL_LESSONS: &[LessonInfo] = &[
    // Phase 1 — Programming 101 in Rust
    LessonInfo {
        number: "01",
        slug: "hello-rust",
        title: "Hello, Rust",
        blurb: "cargo · rustup · println! · fn main()",
        phase: 1,
    },
    LessonInfo {
        number: "02",
        slug: "variables",
        title: "Variables, types, mutability",
        blurb: "let · mut · shadowing · type inference",
        phase: 1,
    },
    LessonInfo {
        number: "03",
        slug: "control-flow",
        title: "Control flow & functions",
        blurb: "if · loop · while · for · expressions",
        phase: 1,
    },
    LessonInfo {
        number: "04",
        slug: "compound-types",
        title: "Compound types",
        blurb: "tuples · arrays · slices · String vs &str",
        phase: 1,
    },
    LessonInfo {
        number: "05",
        slug: "pattern-matching",
        title: "Pattern matching & enums",
        blurb: "match · if let · Option<T>",
        phase: 1,
    },
    LessonInfo {
        number: "06",
        slug: "structs",
        title: "Structs & methods",
        blurb: "struct · impl · associated functions",
        phase: 1,
    },
    // Phase 2 — Ownership deep dive
    LessonInfo {
        number: "07",
        slug: "ownership",
        title: "Ownership & moves",
        blurb: "ownership rules · move semantics · Copy",
        phase: 2,
    },
    LessonInfo {
        number: "08",
        slug: "references",
        title: "References & borrowing",
        blurb: "&T · &mut T · borrowing rules",
        phase: 2,
    },
    LessonInfo {
        number: "09",
        slug: "lifetimes",
        title: "Lifetimes",
        blurb: "'a · lifetime elision",
        phase: 2,
    },
    LessonInfo {
        number: "10",
        slug: "smart-pointers",
        title: "Smart pointers",
        blurb: "Box · Rc · Arc · RefCell",
        phase: 2,
    },
    LessonInfo {
        number: "11",
        slug: "iterators",
        title: "Iterators & collections",
        blurb: "Vec · HashMap · iterator chains",
        phase: 2,
    },
    // Phase 3 — Abstraction
    LessonInfo {
        number: "12",
        slug: "traits-generics",
        title: "Traits & generics",
        blurb: "trait · generic bounds · where",
        phase: 3,
    },
    LessonInfo {
        number: "13",
        slug: "trait-objects",
        title: "Trait objects",
        blurb: "dyn Trait · static vs dynamic dispatch",
        phase: 3,
    },
    LessonInfo {
        number: "14",
        slug: "error-handling",
        title: "Error handling",
        blurb: "Result · ? · thiserror · anyhow",
        phase: 3,
    },
    LessonInfo {
        number: "15",
        slug: "modules",
        title: "Modules, crates, workspaces",
        blurb: "mod · pub · crates · workspace",
        phase: 3,
    },
    // Phase 4 — Concurrency
    LessonInfo {
        number: "16",
        slug: "threads",
        title: "Threads & channels",
        blurb: "thread::spawn · Send · Sync · mpsc",
        phase: 4,
    },
    LessonInfo {
        number: "17",
        slug: "shared-state",
        title: "Shared state",
        blurb: "Mutex · RwLock · Arc<Mutex<T>>",
        phase: 4,
    },
    LessonInfo {
        number: "18",
        slug: "async-tokio",
        title: "Async/await with Tokio",
        blurb: "async · await · Future · Tokio runtime",
        phase: 4,
    },
    // Phase 5 — Systems programming
    LessonInfo {
        number: "19",
        slug: "unsafe-memory",
        title: "Memory, layout, unsafe",
        blurb: "unsafe · raw pointers · #[repr]",
        phase: 5,
    },
    LessonInfo {
        number: "20",
        slug: "ffi",
        title: "FFI",
        blurb: "extern · cbindgen · bindgen",
        phase: 5,
    },
    LessonInfo {
        number: "21",
        slug: "io",
        title: "I/O & filesystem",
        blurb: "std::io · File · Read · Write",
        phase: 5,
    },
    // Phase 6 — Production services
    LessonInfo {
        number: "22",
        slug: "cli",
        title: "Building a CLI",
        blurb: "clap · exit codes · structured output",
        phase: 6,
    },
    LessonInfo {
        number: "23",
        slug: "axum",
        title: "HTTP services with Axum",
        blurb: "axum · extractors · State · handlers",
        phase: 6,
    },
    LessonInfo {
        number: "24",
        slug: "persistence",
        title: "Persistence",
        blurb: "sqlx · migrations · transactions",
        phase: 6,
    },
    LessonInfo {
        number: "25",
        slug: "observability",
        title: "Observability",
        blurb: "tracing · structured logs · metrics",
        phase: 6,
    },
    // Phase 7 — Tooling & quality
    LessonInfo {
        number: "26",
        slug: "testing",
        title: "Testing",
        blurb: "unit · integration · doc tests · proptest",
        phase: 7,
    },
    LessonInfo {
        number: "27",
        slug: "lints",
        title: "Clippy, rustfmt, miri",
        blurb: "lints · rustfmt · miri · CI",
        phase: 7,
    },
    LessonInfo {
        number: "28",
        slug: "benchmarking",
        title: "Benchmarking & profiling",
        blurb: "criterion · flamegraphs · pprof",
        phase: 7,
    },
    // Phase 8 — Distributed patterns
    LessonInfo {
        number: "29",
        slug: "grpc",
        title: "Serialization & gRPC",
        blurb: "serde · tonic · prost",
        phase: 8,
    },
    LessonInfo {
        number: "30",
        slug: "resilience",
        title: "Resilience patterns",
        blurb: "retries · timeouts · circuit breakers",
        phase: 8,
    },
    LessonInfo {
        number: "31",
        slug: "capstone",
        title: "Course capstone",
        blurb: "distributed task processor",
        phase: 8,
    },
];

/// Scan `lessons_dir` for lesson directories that have a `slides/` subdir.
///
/// Returns the set of directory names (e.g. `"01-hello-rust"`). If
/// `lessons_dir` does not exist, returns an empty set rather than erroring —
/// this is a normal state for a fresh checkout.
#[must_use = "the set of published lessons drives the rest of the build"]
pub fn collect_published(lessons_dir: &Path) -> Result<HashSet<String>> {
    let mut out = HashSet::new();
    if !lessons_dir.exists() {
        return Ok(out);
    }

    let entries =
        fs::read_dir(lessons_dir).with_context(|| format!("read_dir {}", lessons_dir.display()))?;
    for entry in entries {
        let entry = entry.with_context(|| format!("iterate {}", lessons_dir.display()))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if !path.join("slides").is_dir() {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            out.insert(name.to_string());
        }
    }
    Ok(out)
}

/// Recursively copy `src` into `dst`, creating directories as needed.
pub fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    if !src.exists() {
        return Ok(());
    }
    for entry in WalkDir::new(src) {
        let entry = entry.with_context(|| format!("walk {}", src.display()))?;
        let from = entry.path();
        let rel = from
            .strip_prefix(src)
            .with_context(|| format!("strip_prefix {}", from.display()))?;
        let to = dst.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&to).with_context(|| format!("mkdir -p {}", to.display()))?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("mkdir -p {}", parent.display()))?;
            }
            fs::copy(from, &to)
                .with_context(|| format!("copy {} -> {}", from.display(), to.display()))?;
        }
        // Symlinks and other file types are skipped.
    }
    Ok(())
}

/// Render the static landing page as an HTML string.
#[must_use]
pub fn render_index<S: BuildHasher>(published: &HashSet<String, S>) -> String {
    let mut body = String::new();
    for (phase_num, phase_name) in PHASES {
        writeln!(
            body,
            r#"    <div class="phase">Phase {phase_num} · {name}</div>"#,
            name = escape_html(phase_name)
        )
        .expect("write to String");
        body.push_str("    <div class=\"grid\">\n");
        for lesson in ALL_LESSONS.iter().filter(|l| l.phase == *phase_num) {
            render_lesson(&mut body, lesson, published);
        }
        body.push_str("    </div>\n");
    }

    let mut out = String::with_capacity(INDEX_HEAD.len() + body.len() + INDEX_TAIL.len());
    out.push_str(INDEX_HEAD);
    out.push_str(&body);
    out.push_str(INDEX_TAIL);
    out
}

fn render_lesson<S: BuildHasher>(
    buf: &mut String,
    lesson: &LessonInfo,
    published: &HashSet<String, S>,
) {
    let dir_name = lesson.dir_name();
    let title = escape_html(lesson.title);
    let blurb = escape_html(lesson.blurb);
    let number = lesson.number; // always digits, safe

    if published.contains(&dir_name) {
        writeln!(
            buf,
            r#"      <a class="lesson" href="lessons/{dir_name}/slides/">"#
        )
        .expect("write to String");
        writeln!(buf, r#"        <div class="num">{number}</div>"#).expect("write to String");
        writeln!(buf, r#"        <div class="title">{title}</div>"#).expect("write to String");
        writeln!(buf, r#"        <div class="blurb">{blurb}</div>"#).expect("write to String");
        buf.push_str("      </a>\n");
    } else {
        buf.push_str("      <div class=\"lesson future\" aria-disabled=\"true\">\n");
        writeln!(buf, r#"        <div class="num">{number}</div>"#).expect("write to String");
        writeln!(buf, r#"        <div class="title">{title}</div>"#).expect("write to String");
        writeln!(buf, r#"        <div class="blurb">{blurb}</div>"#).expect("write to String");
        buf.push_str("      </div>\n");
    }
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
    out
}

/// End-to-end build: clear `out_dir`, copy each published lesson's slides into
/// `out_dir/lessons/<name>/slides/`, copy `shared_dir` into
/// `out_dir/shared/reveal/`, and write the landing page.
pub fn build(lessons_dir: &Path, shared_dir: &Path, out_dir: &Path) -> Result<()> {
    if out_dir.exists() {
        fs::remove_dir_all(out_dir).with_context(|| format!("rm -rf {}", out_dir.display()))?;
    }
    fs::create_dir_all(out_dir).with_context(|| format!("mkdir -p {}", out_dir.display()))?;

    let published = collect_published(lessons_dir)?;
    for name in &published {
        let from = lessons_dir.join(name).join("slides");
        let to = out_dir.join("lessons").join(name).join("slides");
        copy_tree(&from, &to).with_context(|| format!("copy slides for {name}"))?;
    }

    if shared_dir.exists() {
        let to = out_dir.join("shared").join("reveal");
        copy_tree(shared_dir, &to)
            .with_context(|| format!("copy shared assets from {}", shared_dir.display()))?;
    }

    let index = render_index(&published);
    let index_path = out_dir.join("index.html");
    fs::write(&index_path, index).with_context(|| format!("write {}", index_path.display()))?;

    Ok(())
}

const INDEX_HEAD: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Rust Training</title>
  <style>
    :root {
      --bg: #1a1410;
      --surface: #25201a;
      --border: rgba(210, 132, 69, 0.22);

      --fg: #f4e4c1;
      --fg-muted: rgba(244, 228, 193, 0.65);
      --fg-subtle: rgba(244, 228, 193, 0.42);

      --accent: #d28445;
      --accent-strong: #b7410e;
      --accent-soft: rgba(210, 132, 69, 0.10);

      --font-sans: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      --font-mono: "JetBrains Mono", "SF Mono", "Source Code Pro", Menlo, Consolas, monospace;
    }

    * { box-sizing: border-box; }
    html, body { margin: 0; padding: 0; background: var(--bg); color: var(--fg); }
    body { font-family: var(--font-sans); font-size: 16px; line-height: 1.55; }

    main { max-width: 960px; margin: 0 auto; padding: 60px 28px 100px; }

    h1 {
      font-size: 2.6rem;
      font-weight: 700;
      color: var(--accent);
      letter-spacing: -0.025em;
      margin: 0 0 0.3rem;
    }
    p.lead {
      font-size: 1.1rem;
      color: var(--fg-muted);
      margin: 0 0 2.5rem;
      max-width: 640px;
    }

    .phase {
      font-family: var(--font-mono);
      font-size: 0.75rem;
      font-weight: 600;
      letter-spacing: 0.12em;
      text-transform: uppercase;
      color: var(--accent);
      margin: 2.5rem 0 1rem;
      display: flex;
      align-items: center;
      gap: 0.7rem;
    }
    .phase::after {
      content: "";
      flex: 1;
      height: 1px;
      background: var(--border);
    }

    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
      gap: 0.7rem;
    }

    .lesson {
      background: var(--accent-soft);
      border: 1px solid var(--border);
      border-radius: 10px;
      padding: 0.95rem 1rem 1rem;
      text-decoration: none;
      color: inherit;
      transition: transform 150ms ease, border-color 150ms ease, background 150ms ease;
      display: block;
    }
    .lesson:hover {
      transform: translateY(-2px);
      border-color: var(--accent);
      background: rgba(210, 132, 69, 0.16);
    }
    .lesson:focus-visible {
      outline: 2px solid var(--accent);
      outline-offset: 3px;
    }
    .lesson .num {
      font-family: var(--font-mono);
      font-size: 0.7rem;
      font-weight: 600;
      color: var(--accent);
      letter-spacing: 0.05em;
    }
    .lesson .title {
      font-size: 1.05rem;
      font-weight: 600;
      color: var(--fg);
      letter-spacing: -0.01em;
      margin-top: 0.25rem;
      line-height: 1.25;
    }
    .lesson .blurb {
      font-family: var(--font-mono);
      font-size: 0.78rem;
      color: var(--fg-muted);
      margin-top: 0.45rem;
      line-height: 1.4;
    }

    .lesson.future {
      background: rgba(255, 255, 255, 0.02);
      border: 1px dashed rgba(255, 255, 255, 0.08);
      opacity: 0.42;
      cursor: default;
    }
    .lesson.future:hover {
      transform: none;
      background: rgba(255, 255, 255, 0.02);
      border-color: rgba(255, 255, 255, 0.08);
    }
    .lesson.future .num,
    .lesson.future .title { color: var(--fg-muted); }
    .lesson.future .blurb { color: var(--fg-subtle); }

    footer {
      margin-top: 4rem;
      padding-top: 1.5rem;
      border-top: 1px solid rgba(255,255,255,0.06);
      font-size: 0.9rem;
      color: var(--fg-subtle);
    }
    footer a { color: var(--accent); text-decoration: none; }
    footer a:hover { text-decoration: underline; }
  </style>
</head>
<body>
  <main>
    <h1>Rust Training</h1>
    <p class="lead">A Rust programming course delivered as code + per-lesson reveal.js slide decks. Starts at programming-101 and finishes with concurrency, systems programming, production services, and distributed patterns.</p>

"#;

const INDEX_TAIL: &str = r#"
    <footer>
      Source: <a href="https://github.com/ristkari-dev/rust-training">github.com/ristkari-dev/rust-training</a>
    </footer>
  </main>
</body>
</html>
"#;
