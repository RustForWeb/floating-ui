//! Middleware implementations for [`compute_position`][`crate::compute_position::compute_position`].
//!
//! See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/index.html) for more documentation.

mod arrow;
mod auto_placement;
mod flip;
mod hide;
mod inline;
mod offset;
mod shift;
mod size;

pub use arrow::*;
pub use auto_placement::*;
pub use flip::*;
pub use hide::*;
pub use inline::*;
pub use offset::*;
pub use shift::*;
pub use size::*;
