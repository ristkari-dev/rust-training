//! Lesson 21 — reference solutions.

use std::io::{self, Read, Write};

pub fn total_bytes(mut reader: impl Read) -> io::Result<usize> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)
}

pub fn copy_uppercased(mut reader: impl Read, mut writer: impl Write) -> io::Result<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    writer.write_all(input.to_uppercase().as_bytes())?;
    Ok(())
}
