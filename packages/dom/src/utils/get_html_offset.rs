use floating_ui_utils::{Coords, dom::NodeScroll};
use web_sys::Element;

use crate::utils::get_window_scroll_bar_x::get_window_scroll_bar_x;

pub fn get_html_offset(document_element: &Element, scroll: &NodeScroll) -> Coords {
    let html_rect = document_element.get_bounding_client_rect();
    let x = html_rect.left() + scroll.scroll_left
        - get_window_scroll_bar_x(document_element, Some(&html_rect));
    let y = html_rect.top() + scroll.scroll_top;

    Coords { x, y }
}
