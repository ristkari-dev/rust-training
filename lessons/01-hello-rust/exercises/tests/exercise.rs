use hello_rust_exercises::greet;

#[test]
fn greets_by_name() {
    assert_eq!(greet("Aki"), "Hello, Aki!");
}

#[test]
fn greets_with_punctuation_intact() {
    assert_eq!(greet("world"), "Hello, world!");
}
