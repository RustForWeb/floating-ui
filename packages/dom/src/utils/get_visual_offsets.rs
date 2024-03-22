use floating_ui_utils::{
    dom::{get_window, DomElementOrWindow},
    Coords,
};
use web_sys::Element;

pub fn get_visual_offsets(_element: Option<&Element>) -> Coords {
    // TODO: web-sys does not support VisualViewport

    // let window = get_window(element.map(|element| element.as_ref()));

    // if !is_web_kit() || !window.visual_viewport {
    //     Coords::new(0.0)
    // } else {
    //     Coords {
    //         x: todo!(),
    //         y: todo!(),
    //     }
    // }

    Coords::new(0.0)
}

pub fn should_add_visual_offsets(
    element: Option<&Element>,
    is_fixed: bool,
    floating_offset_parent: Option<DomElementOrWindow>,
) -> bool {
    match floating_offset_parent {
        Some(DomElementOrWindow::Window(floating_offset_parent)) => {
            if is_fixed
                && *floating_offset_parent != get_window(element.map(|element| element.as_ref()))
            {
                false
            } else {
                is_fixed
            }
        }
        _ => false,
    }
}
