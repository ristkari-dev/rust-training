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
