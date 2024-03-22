use floating_ui_utils::{dom::get_document_element, Rect, Strategy};
use web_sys::Element;

pub fn get_viewport_rect(element: &Element, _strategy: Strategy) -> Rect {
    // let window = get_window(Some(element));
    let html = get_document_element(Some(element.into()));
    // TODO
    // let visual_viewport = window.visual_viewport;

    let x = 0.0;
    let y = 0.0;
    let width = html.client_width() as f64;
    let height = html.client_height() as f64;

    // TODO: visual viewport

    Rect {
        x,
        y,
        width,
        height,
    }
}
