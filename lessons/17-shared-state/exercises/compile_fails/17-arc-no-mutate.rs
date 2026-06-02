// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// `Arc<T>` gives you SHARED ownership — many handles to the same value —
// but only shared (&T) access. You cannot get a `&mut` through an `Arc`,
// because other threads might be reading the value at the same time.
// So `*counter += 1` is rejected: there's no way to mutate the `i32`
// behind an `Arc<i32>`.
//
// rustc reports E0594: "cannot assign to data in an `Arc<i32>`".
//
// The fix: put a `Mutex` inside the `Arc`. `Arc<Mutex<i32>>` shares
// ownership (the `Arc`) AND allows safe mutation (the `Mutex`): lock the
// mutex to get exclusive access, then mutate through the guard.
//
// Hint: make `counter` an `Arc<Mutex<i32>>` (`Arc::new(Mutex::new(0))`)
// and change `*counter += 1;` to `*counter.lock().unwrap() += 1;`.

use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(0);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            *counter += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("final: {}", *counter);
}
