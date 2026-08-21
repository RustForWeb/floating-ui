use floating_ui_utils::{
    Coords,
    dom::{DomElementOrWindow, get_window, is_web_kit},
};
use web_sys::Element;

pub fn get_visual_offsets(element: Option<&Element>) -> Coords {
    let window = get_window(element.map(|element| element.as_ref()));

    if is_web_kit()
        && let Some(visual_viewport) = window.visual_viewport()
    {
        Coords {
            x: visual_viewport.offset_left(),
            y: visual_viewport.offset_top(),
        }
    } else {
        Coords::new(0.0)
    }
}

pub fn should_add_visual_offsets(
    element: Option<&Element>,
    is_fixed: bool,
    floating_offset_parent: Option<DomElementOrWindow>,
) -> bool {
    match floating_offset_parent {
        Some(DomElementOrWindow::Window(floating_offset_parent)) => {
            is_fixed
                && *floating_offset_parent == get_window(element.map(|element| element.as_ref()))
        }
        _ => false,
    }
}
