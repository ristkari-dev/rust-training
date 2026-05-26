// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Method receivers come in three kinds:
//   - `&self`     — borrow for reading
//   - `&mut self` — borrow for mutation
//   - `self`      — take ownership
//
// The `increment` method below tries to modify `self.count`, but its
// receiver is `&self` — a read-only borrow. The compiler refuses.
//
// Hint: read the rustc error. It will mention "cannot assign" and
// "behind a `&` reference". The fix is to change the receiver from
// `&self` to `&mut self`.

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&self) {
        self.count += 1;
    }

    fn value(&self) -> u32 {
        self.count
    }
}

fn main() {
    let mut c = Counter::new();
    c.increment();
    println!("{}", c.value());
}
