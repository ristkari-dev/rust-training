// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// A thread might run at any time, on any core, so anything you move into
// it must be safe to send across threads — the `Send` marker trait.
// `Rc<T>` is deliberately NOT `Send`: it counts references with a plain
// (non-atomic) integer, so two threads cloning/dropping it at once would
// corrupt the count. The compiler rejects sending an `Rc` to a thread.
//
// rustc reports E0277: "`Rc<i32>` cannot be sent between threads
// safely", and explains the closure isn't `Send` because it captures an
// `Rc`.
//
// The fix: use `Arc<T>` ("atomic Rc"), the thread-safe reference-counted
// pointer. Its count uses atomic operations, so it IS `Send`. We'll use
// `Arc` properly in Lesson 17.
//
// Hint: change `use std::rc::Rc;` to `use std::sync::Arc;` and `Rc::new`
// to `Arc::new`.

use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(42);
    let handle = thread::spawn(move || {
        println!("value is {}", *data);
    });
    handle.join().unwrap();
}
