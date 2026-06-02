//! Lesson 17 — reference solutions.

use std::sync::{Arc, Mutex};
use std::thread;

#[must_use]
pub fn locked_increment(m: &Mutex<i32>, by: i32) -> i32 {
    let mut guard = m.lock().unwrap();
    *guard += by;
    *guard
}

#[must_use]
pub fn concurrent_counter(threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();
    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..per_thread {
                let mut count = counter.lock().unwrap();
                *count += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    *counter.lock().unwrap()
}
