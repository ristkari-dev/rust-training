//! Lesson 13 — reference solutions.

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
pub fn describe_price(item: &dyn Priced) -> String {
    if item.is_free() {
        "free".to_string()
    } else {
        format!("{} cents", item.price())
    }
}

#[must_use]
pub fn total_price_dyn(items: &[Box<dyn Priced>]) -> u32 {
    items.iter().map(|item| item.price()).sum()
}
