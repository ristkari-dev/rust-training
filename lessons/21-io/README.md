# Lesson 21 — I/O & the filesystem

Reading and writing data is inherently fallible — the disk fills, the
file's missing, the socket drops. Rust models every I/O operation as a
`Result`, and builds the whole ecosystem on two small traits: `Read` and
`Write`. The trick is to program to those traits, not to `File` — which
also makes your code testable without touching disk. This lesson closes
Phase 5.

## Learning goals

- Explain that I/O is fallible: operations return `io::Result<T>`,
  propagated with `?`
- Use the `Read` trait to consume bytes from any source
- Use the `Write` trait to send bytes to any sink (and why `write_all`,
  not `write`)
- Write functions generic over `impl Read` / `impl Write`, so the same
  code serves files, sockets, and in-memory buffers
- Recognize `File`, buffering (`BufReader`/`BufWriter`), and the rule
  that a trait must be in scope to call its methods

## Self-study notes

### I/O is fallible — `io::Result`

Every read or write can fail, so I/O functions return `io::Result<T>` (an
alias for `Result<T, io::Error>`). Handle it the way you learned in
Lesson 14 — match it, or `?` it up:

```rust
use std::io;

fn read_config() -> io::Result<String> {
    let text = std::fs::read_to_string("config.toml")?;
    Ok(text)
}
```

### The `Read` trait

Anything you can read bytes *from* implements `Read` — files, sockets,
stdin, even `&[u8]`:

```rust
use std::io::Read;

let mut buf = Vec::new();
reader.read_to_end(&mut buf)?;        // read every byte
// or reader.read_to_string(&mut s)?  // for UTF-8 text
```

You must `use std::io::Read;` to call these methods — they live on the
trait.

### The `Write` trait

Anything you can write bytes *to* implements `Write` — files, sockets,
stdout, `Vec<u8>`:

```rust
use std::io::Write;

writer.write_all(b"hello")?;          // write ALL the bytes
writer.flush()?;
```

Prefer `write_all` over `write`: a single `write` may write *fewer* bytes
than you gave it, so you'd have to loop. `write_all` handles that.

### Program to the traits

Write functions generic over `impl Read` / `impl Write`, not over `File`:

```rust
fn copy_uppercased(mut r: impl Read, mut w: impl Write) -> io::Result<()> {
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    w.write_all(s.to_uppercase().as_bytes())?;
    Ok(())
}
```

The same function then works on files, network streams, stdin/stdout —
and in tests, on `&[u8]` and `Vec<u8>`. I/O decoupled from logic.

### Files, buffering, and stdin/stdout

A `File` is just a `Read` + `Write` type, so the generic functions above
work on it unchanged:

```rust
use std::fs::File;

let f = File::open("input.txt")?;             // read; File::create to write
let text = std::fs::read_to_string("in.txt")?; // convenience shortcut
```

Each raw `read`/`write` can be a syscall, so wrap a reader/writer in
`BufReader`/`BufWriter` to batch them (`BufRead::lines()` reads line by
line). The console — `io::stdin()` / `io::stdout()` — is `Read`/`Write`
too.

## Exercises

### Warm-up: `total_bytes`

Implement `total_bytes(reader: impl Read) -> io::Result<usize>` that reads
the whole reader and returns the number of bytes read:

```rust
pub fn total_bytes(mut reader: impl Read) -> io::Result<usize> {
    // let mut buf = Vec::new();
    // reader.read_to_end(&mut buf)
    todo!()
}
```

`read_to_end` returns the byte count as an `io::Result<usize>`, so you can
return it directly. The tests pass a `&[u8]` as the reader — no files
needed.

### Main: `copy_uppercased`

Implement `copy_uppercased(reader: impl Read, writer: impl Write) ->
io::Result<()>`. Read all of `reader` as text, uppercase it, and write the
bytes to `writer`:

```rust
pub fn copy_uppercased(mut reader: impl Read, mut writer: impl Write) -> io::Result<()> {
    // reader.read_to_string(&mut input)?;
    // writer.write_all(input.to_uppercase().as_bytes())?;
    todo!()
}
```

The tests use a `&[u8]` reader and a `Vec<u8>` writer, so the whole thing
runs in memory. Use `write_all`, not `write`.

### Compile-fail

`exercises/compile_fails/21-trait-not-in-scope.rs` calls a `Read` method
on a `&[u8]` without importing the trait, which the compiler rejects
(E0599 — the trait must be in scope). Fix it by adding `use std::io::Read;`.

### Run

```bash
make verify LESSON=21-io
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
