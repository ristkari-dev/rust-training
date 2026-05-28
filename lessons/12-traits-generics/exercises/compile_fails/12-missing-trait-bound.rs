// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// A generic type parameter `T` with no bounds could be ANY type. The
// compiler therefore has no idea whether a `T` has a `.price()` method
// — most types don't — so it rejects the call.
//
// rustc will say "no method named `price` found" for `T` (E0599) and
// suggests restricting the type parameter with a trait bound.
//
// The fix: tell the compiler that `T` must implement `Priced`, so every
// `T` is guaranteed to have `.price()`. Change `<T>` to `<T: Priced>`.
//
// Hint: add the bound `T: Priced` to the generic function.

trait Priced {
    fn price(&self) -> u32;
}

struct Book {
    cents: u32,
}

impl Priced for Book {
    fn price(&self) -> u32 {
        self.cents
    }
}

fn total_price<T>(items: &[T]) -> u32 {
    let mut total = 0;
    for item in items {
        total += item.price();
    }
    total
}

fn main() {
    let books = [Book { cents: 100 }, Book { cents: 200 }];
    let _ = total_price(&books);
}
