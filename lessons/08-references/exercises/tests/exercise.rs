use references_exercises::{merge_into, wrap_in_quotes};

// Warm-up: wrap_in_quotes

#[test]
fn warmup_typical() {
    assert_eq!(wrap_in_quotes("hello"), "\"hello\"");
}

#[test]
fn warmup_empty() {
    assert_eq!(wrap_in_quotes(""), "\"\"");
}

#[test]
fn warmup_with_spaces() {
    assert_eq!(wrap_in_quotes("hello world"), "\"hello world\"");
}

#[test]
fn warmup_with_inner_quote() {
    assert_eq!(wrap_in_quotes("she said \"hi\""), "\"she said \"hi\"\"");
}

// Main: merge_into

#[test]
fn main_typical() {
    let mut target = String::from("items: ");
    merge_into(&mut target, &["a", "b", "c"], "-");
    assert_eq!(target, "items: a-b-c");
}

#[test]
fn main_empty_parts_no_change() {
    let mut target = String::from("unchanged");
    merge_into(&mut target, &[], "-");
    assert_eq!(target, "unchanged");
}

#[test]
fn main_single_part_no_separator() {
    let mut target = String::new();
    merge_into(&mut target, &["solo"], "-");
    assert_eq!(target, "solo");
}

#[test]
fn main_multi_char_separator() {
    let mut target = String::new();
    merge_into(&mut target, &["one", "two", "three"], ", ");
    assert_eq!(target, "one, two, three");
}
