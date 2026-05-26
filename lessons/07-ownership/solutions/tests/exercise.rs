use ownership_solutions::{append_excl, swap_and_join};

// Warm-up: append_excl

#[test]
fn warmup_typical() {
    assert_eq!(append_excl(String::from("hello")), "hello!");
}

#[test]
fn warmup_empty() {
    assert_eq!(append_excl(String::new()), "!");
}

#[test]
fn warmup_existing_punctuation() {
    assert_eq!(append_excl(String::from("oh no.")), "oh no.!");
}

#[test]
fn warmup_multibyte_chars() {
    assert_eq!(append_excl(String::from("café")), "café!");
}

// Main: swap_and_join

#[test]
fn main_typical() {
    assert_eq!(
        swap_and_join(String::from("hello"), String::from("world")),
        "world hello"
    );
}

#[test]
fn main_single_chars() {
    assert_eq!(swap_and_join(String::from("a"), String::from("b")), "b a");
}

#[test]
fn main_empty_first() {
    assert_eq!(swap_and_join(String::new(), String::from("hi")), "hi ");
}

#[test]
fn main_empty_second() {
    assert_eq!(swap_and_join(String::from("hi"), String::new()), " hi");
}
