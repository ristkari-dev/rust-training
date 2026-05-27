use lifetimes_exercises::{Excerpt, longest};

// Warm-up: longest

#[test]
fn warmup_b_is_longer() {
    assert_eq!(longest("hi", "hello"), "hello");
}

#[test]
fn warmup_a_is_longer() {
    assert_eq!(longest("hello", "hi"), "hello");
}

#[test]
fn warmup_equal_takes_a() {
    assert_eq!(longest("same", "size"), "same");
}

#[test]
fn warmup_empty_a() {
    assert_eq!(longest("", "anything"), "anything");
}

// Main: Excerpt<'a>

#[test]
fn main_text_field_accessible() {
    let e = Excerpt::new("hello");
    assert_eq!(e.text, "hello");
}

#[test]
fn main_length() {
    assert_eq!(Excerpt::new("hello").length(), 5);
}

#[test]
fn main_is_empty_true() {
    assert!(Excerpt::new("").is_empty());
}

#[test]
fn main_is_empty_false() {
    assert!(!Excerpt::new("hi").is_empty());
}
