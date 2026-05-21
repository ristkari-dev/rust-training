use std::path::PathBuf;

use anyhow::Result;
use build_index::build;
use clap::Parser;

#[derive(Parser)]
#[command(about = "Build static slides site into out/")]
struct Cli {
    /// Directory containing lesson folders
    #[arg(long, default_value = "lessons")]
    lessons: PathBuf,

    /// Directory containing shared reveal.js assets
    #[arg(long, default_value = "shared/reveal")]
    shared: PathBuf,

    /// Output directory
    #[arg(long, default_value = "dist")]
    out: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    build(&cli.lessons, &cli.shared, &cli.out)?;
    println!("built {}", cli.out.display());
    Ok(())
}
