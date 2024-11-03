use floating_ui_utils::{dom::NodeScroll, Coords};
use web_sys::Element;

use crate::utils::get_window_scroll_bar_x::get_window_scroll_bar_x;

pub fn get_html_offset(
    document_element: &Element,
    scroll: &NodeScroll,
    ignore_scrollbar_x: Option<bool>,
) -> Coords {
    let ignore_scrollbar_x = ignore_scrollbar_x.unwrap_or(false);

    let html_rect = document_element.get_bounding_client_rect();
    let x = html_rect.left() + scroll.scroll_left
        - if ignore_scrollbar_x {
            0.0
        } else {
            // RTL <body> scrollbar.
            get_window_scroll_bar_x(document_element, Some(&html_rect))
        };
    let y = html_rect.top() + scroll.scroll_top;

    Coords { x, y }
}
