//! Lesson 13 — exercises.
//!
//! Implement `describe_price` (warm-up) and `total_price_dyn` (main) so
//! that `cargo test --manifest-path
//! lessons/13-trait-objects/exercises/Cargo.toml` passes. The trait,
//! structs, and impls are given (they were Lesson 12's exercise) — this
//! lesson is about *using* trait objects. The tests live in
//! `tests/exercise.rs`.

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
        self.cents
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        200 + self.shots * 50
    }
}

#[must_use]
pub fn describe_price(_item: &dyn Priced) -> String {
    todo!("return \"free\" if the item is free, otherwise \"<price> cents\"")
}

#[must_use]
pub fn total_price_dyn(_items: &[Box<dyn Priced>]) -> u32 {
    todo!("sum the price of every boxed trait object in the slice")
}
