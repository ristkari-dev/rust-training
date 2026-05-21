use anyhow::Result;
use clap::Parser;
use new_lesson::{scaffold, validate_name};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Scaffold a new lesson under lessons/")]
struct Cli {
    /// Lesson name, e.g. "07-ownership"
    name: String,

    /// Repository root (default: current dir)
    #[arg(long, default_value = ".")]
    root: PathBuf,

    /// Templates directory (default: <root>/templates)
    #[arg(long)]
    templates: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    validate_name(&cli.name)?;

    let templates = cli.templates.unwrap_or_else(|| cli.root.join("templates"));
    let target = cli.root.join("lessons").join(&cli.name);

    scaffold(&templates, &target, &cli.name)?;

    println!("scaffolded {}", target.display());
    Ok(())
}
