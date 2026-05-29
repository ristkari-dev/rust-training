//! Lesson 15 — exercises.
//!
//! Implement `rectangle_area` (warm-up, in `geometry/shapes.rs`) and
//! `total_area` (main, in `geometry.rs`) so that `cargo test
//! --manifest-path lessons/15-modules/exercises/Cargo.toml` passes. The
//! module structure and re-exports are given — this lesson is about how
//! code is organized into modules. The tests live in
//! `tests/exercise.rs`.

pub mod geometry;

pub use geometry::shapes::rectangle_area;
pub use geometry::total_area;
