//! Geometry helpers (the parent module).

pub mod shapes;

#[must_use]
pub fn total_area(rects: &[(u32, u32)]) -> u32 {
    rects
        .iter()
        .map(|&(w, h)| shapes::rectangle_area(w, h))
        .sum()
}
