# Lesson 22 — Building a CLI

A command-line tool is just a function from `argv` to an exit code.
`clap` handles the tedious part — turning a list of strings into a typed
struct, with `--help` and error messages for free — so you can focus on
what the tool does. The production skill: keep it *testable*. This lesson
opens Phase 6.

## Learning goals

- Define a CLI's arguments as a struct with `#[derive(Parser)]` and
  `#[arg(...)]` attributes
- Parse with clap, and explain why `try_parse_from(iter)` is the testable
  counterpart to `parse()`
- Test a CLI without spawning a process — drive parsing with an array of
  strings and test the logic as an ordinary function
- Produce structured output from parsed arguments
- Recognize that `main` returns an `ExitCode` to signal success/failure

## Self-study notes

### `#[derive(Parser)]` and `#[arg(...)]`

Describe your arguments as a struct; each field becomes an argument:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "greet")]
struct Cli {
    /// Who to greet.            (doc comments become --help text)
    name: String,               // required positional argument

    /// How many times.
    #[arg(short, long, default_value_t = 1)]
    count: u8,                  // optional: --count / -c, defaults to 1
}
```

A bare field is a required positional; `#[arg(long)]` gives `--count`,
`short` gives `-c`, and `default_value_t` makes it optional.

### Parsing — `parse()` vs `try_parse_from()`

```rust
let cli = Cli::parse();                               // main: reads real argv
let cli = Cli::try_parse_from(["greet", "Bob"])?;     // testable
```

`Cli::parse()` reads the real `argv` and, on a bad argument, prints an
error and *exits the process*. `try_parse_from(iter)` takes the arguments
explicitly and returns a `Result` instead of exiting — which is what makes
it testable.

### Testing a CLI

Because `try_parse_from` doesn't touch `argv` or exit, you can test
parsing in-process, and keep the work in a plain function:

```rust
let cli = Cli::try_parse_from(["greet", "Alice"]).unwrap();
assert_eq!(cli.name, "Alice");
// then assert on run(&cli) directly — no subprocess
```

(The first element is the program name, just like real `argv`.)

### Exit codes — `ExitCode`

A CLI tells the shell whether it succeeded through its *exit code*, not
its output. `main` can return one:

```rust
use std::process::ExitCode;

fn main() -> ExitCode {
    ExitCode::SUCCESS          // 0; ExitCode::FAILURE or ::from(2) otherwise
}
```

`0` means success, non-zero means failure — scripts and `&&`/`||` chains
depend on it.

### Structured output (stdout/stderr, machine-readable)

Print results to **stdout** and errors/logs to **stderr**, so a user can
pipe the results without the noise (`tool > out.txt`). For output other
programs consume, offer a machine-readable format (e.g. a `--json` flag)
alongside the human-friendly default.

## Exercises

### Warm-up: `parse`

Implement `parse(args: &[&str]) -> Result<Cli, clap::Error>` that parses
the given arguments with clap's testable entry point:

```rust
pub fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
    // Cli::try_parse_from(args)
    todo!()
}
```

Use `try_parse_from` (not `parse()`), so the tests can pass arguments
explicitly. The first element is the program name (e.g. `"greet"`).

### Main: `run`

Implement `run(cli: &Cli) -> String` that returns the greeting —
`"Hello, {name}!"` — repeated `count` times, joined by newlines:

```rust
pub fn run(cli: &Cli) -> String {
    // (0..cli.count).map(|_| format!("Hello, {}!", cli.name)).collect::<Vec<_>>().join("\n")
    todo!()
}
```

Returning a `String` (instead of printing) keeps it directly testable —
the real `main` would `println!` it.

### Compile-fail

`exercises/compile_fails/22-exitcode-mismatch.rs` declares `main` to
return `ExitCode` but returns a bare `0`, which the compiler rejects
(E0308 — an integer is not an `ExitCode`). Fix it by returning
`ExitCode::SUCCESS`.

### Run

```bash
make verify LESSON=22-cli
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
