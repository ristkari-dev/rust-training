use cli_exercises::{Cli, parse, run};

// Warm-up: parse (testable clap parsing via try_parse_from)

#[test]
fn warmup_name_only() {
    let cli = parse(&["greet", "Alice"]).unwrap();
    assert_eq!(cli.name, "Alice");
    assert_eq!(cli.count, 1);
}

#[test]
fn warmup_with_count() {
    let cli = parse(&["greet", "Bob", "--count", "3"]).unwrap();
    assert_eq!(cli.count, 3);
}

#[test]
fn warmup_short_flag() {
    let cli = parse(&["greet", "Cara", "-c", "2"]).unwrap();
    assert_eq!(cli.count, 2);
}

#[test]
fn warmup_missing_name_errors() {
    assert!(parse(&["greet"]).is_err());
}

// Main: run (the command logic → structured output)

#[test]
fn main_run_once() {
    let cli = Cli {
        name: "Alice".to_string(),
        count: 1,
    };
    assert_eq!(run(&cli), "Hello, Alice!");
}

#[test]
fn main_run_thrice() {
    let cli = Cli {
        name: "Bob".to_string(),
        count: 3,
    };
    assert_eq!(run(&cli), "Hello, Bob!\nHello, Bob!\nHello, Bob!");
}

#[test]
fn main_run_zero() {
    let cli = Cli {
        name: "X".to_string(),
        count: 0,
    };
    assert_eq!(run(&cli), "");
}

#[test]
fn main_run_via_parse() {
    let cli = parse(&["greet", "Dora", "-c", "2"]).unwrap();
    assert_eq!(run(&cli), "Hello, Dora!\nHello, Dora!");
}
