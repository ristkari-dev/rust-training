# Building a CLI

> A command-line tool is just a function from `argv` to an exit code. `clap` handles the tedious part — turning a list of strings into a typed struct, with help and error messages for free — so you can focus on what the tool does.

---

## `argv` and `clap`

A program receives its arguments as a list of strings (`std::env::args()`). Parsing them by hand — flags, defaults, `--help` — is tedious and error-prone.

`clap` is the standard crate that does it: you describe the arguments, it does the parsing.

---

## `#[derive(Parser)]`

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "greet")]
struct Cli {
    name: String,          // a required positional argument
    count: u8,             // (see next slide for flags/defaults)
}
```

Describe your arguments as a struct; each field becomes an argument. A bare field is a required positional; doc comments become `--help` text.

---

## Argument attributes

```rust
/// How many times to greet them.
#[arg(short, long, default_value_t = 1)]
count: u8,
```

`#[arg(...)]` configures a field: `long` gives `--count`, `short` gives `-c`, `default_value_t` makes it optional with a default. Without a default, the argument is required and clap errors if it's missing.

---

## Parsing

```rust
let cli = Cli::parse();                       // in main: reads real argv
let cli = Cli::try_parse_from(["greet", "Bob", "-c", "3"])?; // testable
```

`Cli::parse()` reads the real `argv` and, on a bad argument, prints an error and exits the process. `try_parse_from(iter)` takes the arguments explicitly and returns a `Result` instead of exiting — which is what makes it testable.

---

## Testing CLIs

```rust
let cli = Cli::try_parse_from(["greet", "Alice"]).unwrap();
assert_eq!(cli.name, "Alice");
```

Because `try_parse_from` doesn't touch the real `argv` or exit, you can test parsing in-process. And keep the *work* in a plain function (`run(&cli) -> String`) you can call and assert directly — no subprocess, no captured stdout.

---

## Exit codes

```rust
use std::process::ExitCode;

fn main() -> ExitCode {
    if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
```

A CLI tells the shell whether it succeeded through its *exit code*, not its output. `0` means success; non-zero means failure. `ExitCode::from(2)` sets a specific code. Scripts and `&&`/`||` chains depend on it.

---

## Structured output

Print results to **stdout** and errors/logs to **stderr**, so a user can pipe the results without the noise:

```text
tool > out.txt      # results only; errors still show on the terminal
```

For output other programs consume, offer a machine-readable format (e.g. a `--json` flag) alongside the human-friendly default.

---

## Putting it together

Today's exercises (a `greet` CLI; the `Cli` struct is given):

- **Warm-up** `parse` — wrap `Cli::try_parse_from` so parsing is testable
- **Main** `run` — turn a parsed `Cli` into the greeting output

The compile-fail returns a bare integer from a `main` declared to return `ExitCode`.

---

## Wrap — Phase 6 begins

- `#[derive(Parser)]` turns a struct into a CLI
- `#[arg(...)]` configures each argument
- `try_parse_from` is the testable parse (vs `parse()`)
- keep the work in a function you can call directly
- `main` returns an `ExitCode` to signal success or failure

Next: **Lesson 23 — HTTP services with Axum**.
