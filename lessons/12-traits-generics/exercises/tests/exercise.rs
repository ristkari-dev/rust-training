use traits_generics_exercises::{Book, Coffee, Priced, total_price};

// Warm-up: implement Priced for Book and Coffee

#[test]
fn warmup_book_price() {
    assert_eq!(Book { cents: 1299 }.price(), 1299);
}

#[test]
fn warmup_coffee_price() {
    assert_eq!(Coffee { shots: 2 }.price(), 300);
}

#[test]
fn warmup_is_free_true() {
    assert!(Book { cents: 0 }.is_free());
}

#[test]
fn warmup_is_free_false() {
    assert!(!Coffee { shots: 1 }.is_free());
}

// Main: total_price

#[test]
fn main_total_empty() {
    let books: [Book; 0] = [];
    assert_eq!(total_price(&books), 0);
}

#[test]
fn main_total_books() {
    let books = [Book { cents: 100 }, Book { cents: 250 }, Book { cents: 50 }];
    assert_eq!(total_price(&books), 400);
}

#[test]
fn main_total_coffees() {
    let coffees = [Coffee { shots: 1 }, Coffee { shots: 3 }];
    assert_eq!(total_price(&coffees), 600);
}

#[test]
fn main_total_single() {
    let books = [Book { cents: 999 }];
    assert_eq!(total_price(&books), 999);
}
