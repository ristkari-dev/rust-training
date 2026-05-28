//! Lesson 12 — exercises.
//!
//! Fill in the `price` bodies for `Book` and `Coffee` (warm-up) and the
//! `total_price` body (main) so that `cargo test --manifest-path
//! lessons/12-traits-generics/exercises/Cargo.toml` passes. The tests
//! live in `tests/exercise.rs`.

pub trait Priced {
    fn price(&self) -> u32;

    fn is_free(&self) -> bool {
        self.price() == 0
    }
}

pub struct Book {
    pub cents: u32,
}

pub struct Coffee {
    pub shots: u32,
}

impl Priced for Book {
    fn price(&self) -> u32 {
        todo!("return this book's price in cents")
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        todo!("compute this coffee's price: 200 plus 50 per shot")
    }
}

#[must_use]
pub fn total_price<T: Priced>(_items: &[T]) -> u32 {
    todo!("sum the price of every item in the slice")
}
