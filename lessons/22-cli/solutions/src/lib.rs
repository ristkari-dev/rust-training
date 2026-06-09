//! Lesson 22 — reference solutions.

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

pub fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}

#[must_use]
pub fn run(cli: &Cli) -> String {
    (0..cli.count)
        .map(|_| format!("Hello, {}!", cli.name))
        .collect::<Vec<_>>()
        .join("\n")
}
