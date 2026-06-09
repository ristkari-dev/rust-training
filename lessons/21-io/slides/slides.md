# I/O & the filesystem

> Reading and writing data is inherently fallible — the disk fills, the file's missing, the socket drops. Rust models every I/O operation as a `Result`, and builds the whole ecosystem on two small traits: `Read` and `Write`.

---

## I/O is fallible

Every read or write can fail, so I/O functions return `io::Result<T>` (an alias for `Result<T, io::Error>`).

```rust
use std::io;

fn read_config() -> io::Result<String> {
    let text = std::fs::read_to_string("config.toml")?;
    Ok(text)
}
```

Handle it the way you learned in Lesson 14 — match it, or `?` it up.

---

## The `Read` trait

Anything you can read bytes *from* implements `Read` — files, sockets, stdin, even `&[u8]`:

```rust
use std::io::Read;

let mut buf = Vec::new();
reader.read_to_end(&mut buf)?;        // read every byte
// or reader.read_to_string(&mut s)?  // for UTF-8 text
```

You must `use std::io::Read;` to call these methods — they live on the trait.

---

## The `Write` trait

Anything you can write bytes *to* implements `Write` — files, sockets, stdout, `Vec<u8>`:

```rust
use std::io::Write;

writer.write_all(b"hello")?;          // write ALL the bytes
writer.flush()?;
```

Prefer `write_all` over `write`: a single `write` may write *fewer* bytes than you gave it, and you'd have to loop. `write_all` handles that.

---

## Program to the traits

```rust
fn copy_uppercased(mut r: impl Read, mut w: impl Write) -> io::Result<()> {
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    w.write_all(s.to_uppercase().as_bytes())?;
    Ok(())
}
```

Write functions generic over `impl Read` / `impl Write`, not over `File`. The same function works on files, network streams, stdin/stdout — and in tests, on `&[u8]` and `Vec<u8>`. I/O decoupled from logic.

---

## Files

```rust
use std::fs::File;

let f = File::open("input.txt")?;     // read; File::create for write
let text = std::fs::read_to_string("input.txt")?;  // shortcut
std::fs::write("out.txt", b"data")?;               // shortcut
```

A `File` implements `Read` + `Write`, so the generic functions from the previous slide work on it unchanged.

---

## Buffering

```rust
use std::io::{BufReader, BufRead};

let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;
}
```

Each raw `read`/`write` can be a syscall. `BufReader`/`BufWriter` batch them. `BufRead::lines()` is the easy way to read a file line by line.

---

## stdin / stdout

```rust
use std::io::Write;

let mut out = std::io::stdout();
out.write_all(b"hello\n")?;
```

The console is `Read`/`Write` too: `io::stdin()` reads; `io::stdout()` / `stderr()` write. Same traits, same methods.

---

## Putting it together

Today's exercises:

- **Warm-up** `total_bytes(impl Read)` — read the whole reader, return the byte count
- **Main** `copy_uppercased(impl Read, impl Write)` — read text, uppercase it, write it back

Both are tested with in-memory buffers (`&[u8]`, `Vec<u8>`) — no files needed. The compile-fail calls a `Read` method without importing the trait.

---

## Wrap — Phase 5 complete

- I/O returns `io::Result`, propagated with `?`
- `Read` consumes bytes; `Write` emits them (`write_all`, not `write`)
- generic over `impl Read` / `impl Write` decouples logic from source/sink — and makes it testable
- `File` implements both traits
- you must import the trait to call its methods

Next: **Phase 6 — Lesson 22, Building a CLI**.
