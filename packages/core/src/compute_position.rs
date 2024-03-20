use floating_ui_utils::{Coords, Placement, Strategy};

use crate::compute_coords_from_placement::compute_coords_from_placement;
use crate::types::{ComputePositionConfig, FloatingElement, GetElementRectsArgs, ReferenceElement};

pub fn compute_position(
    reference: ReferenceElement,
    floating: FloatingElement,
    config: ComputePositionConfig,
) {
    let placement = config.placement.unwrap_or(Placement::Bottom);
    let strategy = config.strategy.unwrap_or(Strategy::Absolute);
    let platform = config.platform;
    let middleware = config.middleware.unwrap_or(vec![]);

    let rtl = platform.is_rtl(floating);

    let rects = platform.get_element_rects(GetElementRectsArgs {
        reference,
        floating,
        strategy,
    });
    let Coords { x, y } = compute_coords_from_placement();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returned_data() {}

    #[test]
    fn test_middleware() {}

    #[test]
    fn test_middleware_data() {}
}
