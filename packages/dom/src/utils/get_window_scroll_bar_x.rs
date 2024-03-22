use floating_ui_utils::dom::{get_document_element, get_node_scroll};
use web_sys::Element;

use crate::utils::get_bounding_client_rect::get_bounding_client_rect;

pub fn get_window_scroll_bar_x(element: &Element) -> f64 {
    get_bounding_client_rect(
        (&get_document_element(element.into())).into(),
        false,
        false,
        None,
    )
    .left
        + get_node_scroll(element.into()).scroll_left
}
