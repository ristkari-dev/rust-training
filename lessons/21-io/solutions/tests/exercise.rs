use io_solutions::{copy_uppercased, total_bytes};

// Warm-up: total_bytes (the Read trait)

#[test]
fn warmup_empty() {
    let data: &[u8] = b"";
    assert_eq!(total_bytes(data).unwrap(), 0);
}

#[test]
fn warmup_hello() {
    let data: &[u8] = b"hello";
    assert_eq!(total_bytes(data).unwrap(), 5);
}

#[test]
fn warmup_longer() {
    let data: &[u8] = b"the quick brown fox";
    assert_eq!(total_bytes(data).unwrap(), 19);
}

#[test]
fn warmup_bytes() {
    let data: &[u8] = &[0, 1, 2, 3];
    assert_eq!(total_bytes(data).unwrap(), 4);
}

// Main: copy_uppercased (Read -> transform -> Write)

#[test]
fn main_empty() {
    let input: &[u8] = b"";
    let mut output: Vec<u8> = Vec::new();
    copy_uppercased(input, &mut output).unwrap();
    assert_eq!(output, b"");
}

#[test]
fn main_hello() {
    let input: &[u8] = b"hello";
    let mut output: Vec<u8> = Vec::new();
    copy_uppercased(input, &mut output).unwrap();
    assert_eq!(output, b"HELLO");
}

#[test]
fn main_mixed() {
    let input: &[u8] = b"Hello, World!";
    let mut output: Vec<u8> = Vec::new();
    copy_uppercased(input, &mut output).unwrap();
    assert_eq!(output, b"HELLO, WORLD!");
}

#[test]
fn main_already_upper() {
    let input: &[u8] = b"ABC 123";
    let mut output: Vec<u8> = Vec::new();
    copy_uppercased(input, &mut output).unwrap();
    assert_eq!(output, b"ABC 123");
}
