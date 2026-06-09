//! Lesson 22 — exercises.
//!
//! Implement `parse` (warm-up) and `run` (main) so that `cargo test
//! --manifest-path lessons/22-cli/exercises/Cargo.toml` passes. The
//! `Cli` struct is given as a worked `clap` example. The tests live in
//! `tests/exercise.rs`.

use clap::Parser;

/// Greet someone a number of times.
#[derive(Parser, Debug, PartialEq, Eq)]
#[command(name = "greet")]
pub struct Cli {
    /// Who to greet.
    pub name: String,

    /// How many times to greet them.
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

pub fn parse(_args: &[&str]) -> Result<Cli, clap::Error> {
    todo!("parse the arguments with `Cli::try_parse_from`")
}

#[must_use]
pub fn run(_cli: &Cli) -> String {
    todo!("return \"Hello, <name>!\" repeated `count` times, joined by newlines")
}
