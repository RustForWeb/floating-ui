use floating_ui_utils::{Coords, ElementRects, Placement};

pub fn compute_coords_from_placement(
    ElementRects {
        reference,
        floating,
    }: ElementRects,
    placement: Placement,
    rtl: Option<bool>,
) -> Coords {
    Coords { x: 0, y: 0 }
}
