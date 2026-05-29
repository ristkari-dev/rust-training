// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Every element of an array (or slice, or Vec<T>) must have the SAME
// type T. A `Book` and a `Coffee` are different types, so they cannot
// share an array — even though both implement `Priced`. The compiler
// reports a type mismatch (E0308): it expected the array's element type
// `Book`, then found a `Coffee`.
//
// This is exactly the limitation that trait objects solve. Box each
// value as a `Box<dyn Priced>`: now every element has the SAME type
// (`Box<dyn Priced>`), and the concrete type lives behind the pointer.
//
// The fix:
//
//     let items: Vec<Box<dyn Priced>> = vec![
//         Box::new(Book { cents: 100 }),
//         Box::new(Coffee { shots: 2 }),
//     ];
//
// Hint: change the array to a `Vec<Box<dyn Priced>>` and wrap each value
// in `Box::new(...)`.

trait Priced {
    fn price(&self) -> u32;
}

struct Book {
    cents: u32,
}

struct Coffee {
    shots: u32,
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

fn main() {
    let items = [Book { cents: 100 }, Coffee { shots: 2 }];
    let total: u32 = items.iter().map(|item| item.price()).sum();
    println!("{total}");
}
