use modules_solutions::{rectangle_area, total_area};

// Warm-up: rectangle_area (leaf module geometry::shapes)

#[test]
fn warmup_area_basic() {
    assert_eq!(rectangle_area(3, 4), 12);
}

#[test]
fn warmup_area_zero() {
    assert_eq!(rectangle_area(0, 5), 0);
}

#[test]
fn warmup_area_square() {
    assert_eq!(rectangle_area(6, 6), 36);
}

#[test]
fn warmup_area_via_full_path() {
    // the full module path also works
    assert_eq!(
        modules_solutions::geometry::shapes::rectangle_area(2, 7),
        14
    );
}

// Main: total_area (parent module geometry)

#[test]
fn main_total_empty() {
    let rects: [(u32, u32); 0] = [];
    assert_eq!(total_area(&rects), 0);
}

#[test]
fn main_total_one() {
    assert_eq!(total_area(&[(3, 4)]), 12);
}

#[test]
fn main_total_many() {
    assert_eq!(total_area(&[(2, 3), (4, 5), (1, 1)]), 27);
}

#[test]
fn main_total_via_module_path() {
    assert_eq!(modules_solutions::geometry::total_area(&[(10, 10)]), 100);
}
