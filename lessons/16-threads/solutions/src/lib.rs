//! Lesson 16 — reference solutions.

use std::sync::mpsc;
use std::thread;

#[must_use]
pub fn double_in_thread(n: i32) -> i32 {
    let handle = thread::spawn(move || n * 2);
    handle.join().unwrap()
}

#[must_use]
pub fn parallel_sum_of_squares(values: Vec<i32>) -> i32 {
    let (tx, rx) = mpsc::channel();
    for v in values {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(v * v).unwrap();
        });
    }
    drop(tx);
    rx.iter().sum()
}
