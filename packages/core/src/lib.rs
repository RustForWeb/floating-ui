//! This is the platform-agnostic core of Floating UI, exposing the main [`compute_position`][`crate::compute_position::compute_position()`] function but no platform interface logic.

mod compute_coords_from_placement;
mod compute_position;
mod detect_overflow;
pub mod middleware;
mod types;

#[cfg(test)]
mod test_utils;

pub use compute_coords_from_placement::*;
pub use compute_position::*;
pub use detect_overflow::*;
pub use types::*;
