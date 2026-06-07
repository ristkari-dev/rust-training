use ffi_solutions::add;

#[test]
fn adds_positives() {
    assert_eq!(add(2, 3), 5);
}
