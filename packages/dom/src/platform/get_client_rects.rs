use floating_ui_utils::ClientRectObject;
use web_sys::Element;

use crate::utils::get_bounding_client_rect::dom_rect_to_client_rect_object;

pub fn get_client_rects(element: &Element) -> Vec<ClientRectObject> {
    let dom_rect_list = element.get_client_rects();

    (0..dom_rect_list.length())
        .filter_map(|i| dom_rect_list.item(i).map(dom_rect_to_client_rect_object))
        .collect()
}
