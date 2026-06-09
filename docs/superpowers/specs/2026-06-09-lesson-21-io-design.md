# Lesson 21 — I/O & the filesystem — design

The third and final lesson of Phase 5 (Systems programming). The
idiomatic Rust approach to I/O: program to the `Read` and `Write`
*traits*, not to a concrete `File`. I/O is fallible — every operation
returns `io::Result` — and is propagated with `?` (Lesson 14). A function
generic over `impl Read` / `impl Write` works on a file, a socket,
stdin/stdout, or an in-memory buffer; that same genericity is what lets
you test it without touching the real filesystem. `File` and buffering
are covered conceptually. Closes Phase 5.

A deliberate constraint: the exercises are **fully hermetic** — generic
over the I/O traits and driven by in-memory buffers (`&[u8]` as a reader,
`Vec<u8>` as a writer), so the tests never touch the real filesystem and
are perfectly deterministic, with no `tempfile`.

## Audience and prerequisites

- Has completed Lessons 01-20
- Comfortable with traits + `impl Trait` (L12), `Result`/`?` (L14),
  `String`/`&str` and `Vec` (L04/L11)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Explain that I/O is fallible: operations return `io::Result<T>`, and
   `?` propagates the error
2. Use the `Read` trait to consume bytes from any source
   (`read_to_end`, `read_to_string`)
3. Use the `Write` trait to send bytes to any sink (`write_all`, and why
   not `write`)
4. Write functions generic over `impl Read` / `impl Write`, so the same
   code serves files, sockets, stdin/stdout, and in-memory buffers (and
   is testable without the filesystem)
5. Recognize `File` (`File::open`/`File::create`, `fs::read_to_string`),
   buffering (`BufReader`/`BufWriter`), and that you must bring the
   `Read`/`Write` trait into scope to call its methods

## Scope

In scope: I/O fallibility and `io::Result` / `?`; the `Read` trait
(`read_to_end`, `read_to_string`); the `Write` trait (`write_all`, and
why `write` can write fewer bytes); writing functions generic over
`impl Read` / `impl Write` (the decoupling that makes I/O testable);
`File` and `std::fs` conveniences conceptually (`File::open`,
`File::create`, `fs::read_to_string`, `fs::write`); buffering
(`BufReader`/`BufWriter`, `BufRead::lines`) conceptually; stdin/stdout
conceptually; the "import the trait to use its methods" rule (the
compile-fail). The exercises drill a `Read`-consuming byte counter
(warm-up) and a `Read`→`Write` uppercase pipeline (main), both tested
with in-memory buffers.

Out of scope (deferred or skipped): touching the real filesystem in the
exercises (no `tempfile`, no temp dirs); `Seek`; `OpenOptions`; directory
traversal / `read_dir` / `walkdir`; paths (`Path`/`PathBuf`) beyond a
mention; permissions/metadata; async I/O (`tokio::io` — that's the async
world of L18); `BufRead::read_line` mechanics in depth; raw file
descriptors; memory-mapped I/O; error-kind matching (`io::ErrorKind`)
beyond a mention. I/O is introduced as *the `Read`/`Write` traits +
`io::Result`*; the filesystem-walking and async-I/O depth is out of band.

## Slide arc (10 slides)

1. **Title — I/O & the filesystem.** Hook: *"Reading and writing data is
   inherently fallible — the disk fills, the file's missing, the socket
   drops. Rust models every I/O operation as a `Result`, and builds the
   whole ecosystem on two small traits: `Read` and `Write`."*
2. **I/O is fallible.** Every read or write can fail, so I/O functions
   return `io::Result<T>` (an alias for `Result<T, io::Error>`). You
   handle it the way you learned in Lesson 14 — match it, or `?` it up:
   ```rust
   use std::io;
   fn read_config() -> io::Result<String> {
       let text = std::fs::read_to_string("config.toml")?;
       Ok(text)
   }
   ```
3. **The `Read` trait.** Anything you can read bytes *from* implements
   `Read` — files, sockets, stdin, even `&[u8]`:
   ```rust
   use std::io::Read;
   let mut buf = Vec::new();
   reader.read_to_end(&mut buf)?;        // read every byte
   // or read_to_string(&mut s)? for UTF-8 text
   ```
   You must `use std::io::Read;` to call these methods.
4. **The `Write` trait.** Anything you can write bytes *to* implements
   `Write` — files, sockets, stdout, `Vec<u8>`:
   ```rust
   use std::io::Write;
   writer.write_all(b"hello")?;          // write ALL the bytes
   writer.flush()?;
   ```
   Prefer `write_all` over `write`: a single `write` may write *fewer*
   bytes than you gave it, and you'd have to loop. `write_all` handles
   that for you.
5. **Program to the traits.**
   ```rust
   fn copy_uppercased(mut r: impl Read, mut w: impl Write) -> io::Result<()> {
       let mut s = String::new();
       r.read_to_string(&mut s)?;
       w.write_all(s.to_uppercase().as_bytes())?;
       Ok(())
   }
   ```
   Write functions generic over `impl Read` / `impl Write`, not over
   `File`. The same function then works on files, network streams,
   stdin/stdout — and in tests, on `&[u8]` and `Vec<u8>`. I/O decoupled
   from logic.
6. **Files.** A `File` is just a `Read` + `Write` type:
   ```rust
   use std::fs::File;
   let f = File::open("input.txt")?;     // read; File::create for write
   let text = std::fs::read_to_string("input.txt")?;  // shortcut
   std::fs::write("out.txt", b"data")?;               // shortcut
   ```
   Because `File` implements the traits, the generic functions from the
   previous slide work on it unchanged.
7. **Buffering.** Each raw `read`/`write` can be a syscall. Wrap a reader
   or writer in `BufReader`/`BufWriter` to batch them:
   ```rust
   use std::io::{BufReader, BufRead};
   let reader = BufReader::new(file);
   for line in reader.lines() {
       let line = line?;
   }
   ```
   `BufRead::lines()` is the easy way to read a file line by line.
8. **stdin / stdout.** The console is `Read`/`Write` too:
   ```rust
   use std::io::Write;
   let mut out = std::io::stdout();
   out.write_all(b"hello\n")?;
   ```
   `io::stdin()` reads; `io::stdout()`/`stderr()` write. Same traits, same
   methods.
9. **Putting it together.** Walk through the exercises: `total_bytes`
   takes `impl Read` and returns how many bytes it read (warm-up — the
   `Read` trait); `copy_uppercased` takes `impl Read` + `impl Write`,
   reads the text, uppercases it, and writes it back (main — the
   read→transform→write pipeline). Both are tested with in-memory buffers
   (`&[u8]`, `Vec<u8>`) — no files needed. The compile-fail calls a `Read`
   method without importing the trait.
10. **Wrap — Phase 5 complete.** Five takeaways: I/O returns
    `io::Result`, propagated with `?`; `Read` consumes bytes, `Write`
    emits them (`write_all`, not `write`); writing generic over
    `impl Read`/`impl Write` decouples logic from the source/sink and
    makes it testable; `File` implements both traits; you must import the
    trait to call its methods. Next: **Phase 6 — Lesson 22, Building a
    CLI**.

## Exercise spec

`lessons/21-io/` follows the standard four-part lesson shape:

```
21-io/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/21-trait-not-in-scope.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `io-exercises` and `io-solutions` (the lesson's
"bare" name is `io`; the import idents are `io_exercises` /
`io_solutions`). This matches the build-index master registry slug `io`,
so the landing page links it without any change. No external
dependencies; the tests use only in-memory `&[u8]` / `Vec<u8>`.

### Exercise stub (`exercises/src/lib.rs`)

The stub ships both signatures with `todo!()` bodies and uses `_reader` /
`_writer` (no `mut`) so the unused params don't trip `unused`/`unused_mut`
(the solution adds `mut`). The `use std::io::{self, Read, Write};` import
is fully used by the signatures (`io::Result`, `impl Read`, `impl
Write`), so there's no unused-import warning. The crate and its tests
compile; the tests fail at runtime with the `todo!()` panic.

```rust
//! Lesson 21 — exercises.
//!
//! Implement `total_bytes` (warm-up) and `copy_uppercased` (main) so that
//! `cargo test --manifest-path lessons/21-io/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

use std::io::{self, Read, Write};

pub fn total_bytes(_reader: impl Read) -> io::Result<usize> {
    todo!("read the whole reader and return the number of bytes read")
}

pub fn copy_uppercased(_reader: impl Read, _writer: impl Write) -> io::Result<()> {
    todo!("read all of `reader` as text, uppercase it, and write the bytes to `writer`")
}
```

### Warm-up: `total_bytes`

Reference solution:

```rust
pub fn total_bytes(mut reader: impl Read) -> io::Result<usize> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)
}
```

Pedagogical packing: the `Read` trait in its simplest use. `read_to_end`
drains the reader into a `Vec<u8>` and *returns* the number of bytes read
(an `io::Result<usize>`), so the function returns that directly — no
explicit `Ok`/`?` needed (the result is already the right type). Taking
`impl Read` by value (with `mut`) means any reader works: a file, a
socket, or — in the tests — a `&[u8]`. No `#[must_use]` (it returns a
`Result`, already `#[must_use]`).

Four tests (all use `&[u8]` as the reader):

```rust
#[test]
fn warmup_empty() {
    let data: &[u8] = b"";
    assert_eq!(total_bytes(data).unwrap(), 0);
}

#[test]
fn warmup_hello() {
    let data: &[u8] = b"hello";
    assert_eq!(total_bytes(data).unwrap(), 5);
}

#[test]
fn warmup_longer() {
    let data: &[u8] = b"the quick brown fox";
    assert_eq!(total_bytes(data).unwrap(), 19);
}

#[test]
fn warmup_bytes() {
    let data: &[u8] = &[0, 1, 2, 3];
    assert_eq!(total_bytes(data).unwrap(), 4);
}
```

### Main: `copy_uppercased`

Reference solution:

```rust
pub fn copy_uppercased(mut reader: impl Read, mut writer: impl Write) -> io::Result<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    writer.write_all(input.to_uppercase().as_bytes())?;
    Ok(())
}
```

Pedagogical packing: the read→transform→write pipeline, generic over both
traits. `read_to_string` reads the reader's bytes as UTF-8 text (the
`?` propagates any I/O or UTF-8 error and discards the byte count);
`to_uppercase()` transforms; `write_all` writes *every* byte to the
writer (using `write_all`, not `write`, so partial writes are handled).
Returns `io::Result<()>`. In the tests the reader is a `&[u8]` and the
writer is a `&mut Vec<u8>`, so the whole thing runs in memory.

Four tests (reader `&[u8]`, writer `Vec<u8>`):

```rust
#[test]
fn main_empty() {
    let input: &[u8] = b"";
    let mut output: Vec<u8> = Vec::new();
    copy_uppercased(input, &mut output).unwrap();
    assert_eq!(output, b"");
}

#[test]
fn main_hello() {
    let input: &[u8] = b"hello";
    let mut output: Vec<u8> = Vec::new();
    copy_uppercased(input, &mut output).unwrap();
    assert_eq!(output, b"HELLO");
}

#[test]
fn main_mixed() {
    let input: &[u8] = b"Hello, World!";
    let mut output: Vec<u8> = Vec::new();
    copy_uppercased(input, &mut output).unwrap();
    assert_eq!(output, b"HELLO, WORLD!");
}

#[test]
fn main_already_upper() {
    let input: &[u8] = b"ABC 123";
    let mut output: Vec<u8> = Vec::new();
    copy_uppercased(input, &mut output).unwrap();
    assert_eq!(output, b"ABC 123");
}
```

**Eight tests total** (four warm-up + four main). They are fully
deterministic and hermetic — every reader is a `&[u8]` literal and every
writer is an in-memory `Vec<u8>`; nothing touches the filesystem. The
`copy_uppercased` assertions compare the output `Vec<u8>` against a byte
string literal.

### Compile-fail: `21-trait-not-in-scope.rs`

Path: `exercises/compile_fails/21-trait-not-in-scope.rs`. A
self-contained file that calls a `Read` method on a `&[u8]` *without*
`use std::io::Read;` in scope. Ships broken; the student adds the import.
The `compile-fails` tool type-checks with `rustc --crate-type=lib
--emit=metadata` (no linking, no filesystem), so the E0599 fires at
type-check.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// The `Read` and `Write` methods (`read_to_end`, `read_to_string`,
// `write_all`, ...) are defined on TRAITS. To call a trait's methods,
// the trait must be IN SCOPE — you have to `use` it. Here `&[u8]` does
// implement `Read`, but without `use std::io::Read;` the method
// `read_to_end` isn't visible, so the call fails.
//
// rustc reports E0599 ("no method named `read_to_end` ...") and helpfully
// notes that the `Read` trait is implemented but not in scope, suggesting
// the exact import.
//
// The fix: bring the trait into scope.
//
// Hint: add `use std::io::Read;` at the top of the file.

fn main() {
    let data: &[u8] = b"hello";
    let mut buf = Vec::new();
    let n = data.read_to_end(&mut buf);
    println!("{n:?}");
}
```

Pass condition: the student adds `use std::io::Read;`. rustc reports
E0599 with the "trait `Read` ... is implemented but not in scope; perhaps
you want to import it: `use std::io::Read;`" suggestion — verified during
design. After adding the import the file type-checks.

This is the lesson's centerpiece for the trait-based I/O design: the
`Read`/`Write` methods only exist when their trait is imported — the
single most common stumbling block for I/O newcomers.

## README structure

`lessons/21-io/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - I/O is fallible — `io::Result`
  - The `Read` trait
  - The `Write` trait
  - Program to the traits
  - Files, buffering, and stdin/stdout
- **Exercises** — four subsections: Warm-up (`total_bytes`), Main
  (`copy_uppercased`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`Read` trait" / "`Write` trait" and "Program to the traits" sections are
the heaviest — they carry the lesson's core idea.

## Lint expectations

Lesson 21's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design:

- `total_bytes` returns `read_to_end(...)` directly (it already yields
  `io::Result<usize>`), so there's no `let`-and-return or unused-count
  issue.
- `copy_uppercased` uses `write_all` (not `write`), so
  `clippy::unused_io_amount` does not fire; `read_to_string`'s count is
  discarded via `?`, which is fine.
- Neither function carries `#[must_use]` — both return `io::Result`,
  already `#[must_use]` (adding it would trip `double_must_use`).
- `impl Read`/`impl Write` taken by value with `mut`:
  `clippy::needless_pass_by_value` is allowed in the workspace, and the
  reader/writer are consumed by the function.
- The **exercise stub** uses `_reader`/`_writer` (no `mut`) so the unused
  params and bindings lint clean; the `use std::io::{self, Read, Write};`
  import is used by the signatures (verified).

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/21-io/` exists with the four-part structure
- Cargo manifests use the correct package names (`io-exercises`,
  `io-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `total_bytes` / `copy_uppercased` signatures; the exercise ships
  `todo!()` bodies, the solution ships real bodies
- `cargo test --package io-solutions` → 8 tests pass (in-memory, no
  filesystem)
- `cargo test --manifest-path lessons/21-io/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/21-io`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/21-io`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/21-io/slides/index.html`
- `dist/index.html` lists lesson 21 as a clickable link (registry slug
  `io` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/21-io/slides/` returns 200

## Open questions

None.
