// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Items in a module are PRIVATE by default — visible only inside that
// module (and its descendants). The `rectangle_area` function below has
// no `pub`, so `main`, which is outside the `geometry` module, cannot
// call it. The compiler reports E0603: "function `rectangle_area` is
// private".
//
// The fix: mark the function `pub` so it becomes part of the module's
// public surface. (If the module itself were nested and private, it
// would need `pub` too — the whole path must be public.)
//
// Hint: add `pub` in front of `fn rectangle_area`.

mod geometry {
    fn rectangle_area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn main() {
    let area = geometry::rectangle_area(3, 4);
    println!("{area}");
}
