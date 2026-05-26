// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Rust's `match` expression is **exhaustive**: every possible variant
// of the matched type must have an arm. If you forget one, the compiler
// refuses to compile. This is one of Rust's signature safety features —
// you can't accidentally not-handle a case.
//
// The function below matches on a `Direction` enum but forgets one of
// the four variants. rustc will tell you exactly which one is missing.
//
// Hint: read the rustc error. It will say "non-exhaustive patterns:
// `<Variant>` not covered". Add an arm that maps the missing variant to
// its opposite.

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        // West is missing!
    }
}

fn main() {
    let d = opposite(Direction::West);
    println!("{d:?}");
}
