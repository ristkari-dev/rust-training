use anyhow::{Result, anyhow};
use clap::{Parser, ValueEnum};
use compile_fails::{Expect, check_one, find_compile_fail_files};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Run rustc on every compile_fails/*.rs file under PATH")]
struct Cli {
    /// What each file should do.
    #[arg(long, value_enum, default_value_t = ExpectArg::Broken)]
    expect: ExpectArg,

    /// Directory to walk (default: lessons)
    #[arg(default_value = "lessons")]
    root: PathBuf,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ExpectArg {
    Broken,
    Compiles,
}

impl From<ExpectArg> for Expect {
    fn from(value: ExpectArg) -> Self {
        match value {
            ExpectArg::Broken => Expect::Broken,
            ExpectArg::Compiles => Expect::Compiles,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let expect: Expect = cli.expect.into();
    let files = find_compile_fail_files(&cli.root)?;

    if files.is_empty() {
        println!("no compile_fails/ files found under {}", cli.root.display());
        return Ok(());
    }

    let mut failed = 0;
    for file in &files {
        match check_one(file, expect) {
            Ok(()) => println!("ok:   {}", file.display()),
            Err(e) => {
                eprintln!("FAIL: {e}");
                failed += 1;
            }
        }
    }

    if failed == 0 {
        let count = files.len();
        println!("\n{count} compile-fail check(s) passed (expect={expect:?}).");
        Ok(())
    } else {
        Err(anyhow!(
            "{failed} compile-fail check(s) did not match expect={expect:?}"
        ))
    }
}
