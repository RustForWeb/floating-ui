mod types;

// TODO: consider copying the exported types instead of using star
pub use types::*;

use floating_ui_core::{compute_position as compute_position_core, ComputePositionReturn};
use web_sys::Element;

pub fn compute_position(
    reference: &Element,
    floating: &Element,
    config: ComputePositionConfig,
) -> ComputePositionReturn {
    // TODO: cache
    compute_position_core(reference, floating, config)
}
