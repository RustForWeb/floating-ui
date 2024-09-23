use floating_ui_utils::{
    dom::{get_document_element, get_node_scroll},
    Rect,
};
use web_sys::Element;

use crate::platform::is_rtl::is_rtl;

use super::get_window_scroll_bar_x::get_window_scroll_bar_x;

/// Gets the entire size of the scrollable document area, even extending outside of the `<html>` and `<body>` rect bounds if horizontally scrollable.
pub fn get_document_rect(element: &Element) -> Rect {
    let html = get_document_element(Some(element.into()));
    let scroll = get_node_scroll(element.into());
    let body = element
        .owner_document()
        .expect("Element should have owner document.")
        .body()
        .expect("Document should have body.");

    let width = [
        html.scroll_width(),
        html.client_width(),
        body.scroll_width(),
        body.client_width(),
    ]
    .into_iter()
    .max()
    .expect("Iterator is not empty.") as f64;
    let height = [
        html.scroll_height(),
        html.client_height(),
        body.scroll_height(),
        body.client_height(),
    ]
    .into_iter()
    .max()
    .expect("Iterator is not empty.") as f64;

    let mut x = -scroll.scroll_left + get_window_scroll_bar_x(element, None);
    let y = -scroll.scroll_top;

    if is_rtl(&body) {
        x += html.client_width().max(body.client_width()) as f64 - width;
    }

    Rect {
        x,
        y,
        width,
        height,
    }
}
