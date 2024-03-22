use floating_ui_utils::ClientRectObject;
use web_sys::Element;

pub fn get_client_rects(element: &Element) -> Vec<ClientRectObject> {
    let dom_rect_list = element.get_client_rects();

    (0..dom_rect_list.length())
        .filter_map(|i| {
            dom_rect_list.item(i).map(|dom_rect| ClientRectObject {
                x: dom_rect.x() as isize,
                y: dom_rect.y() as isize,
                width: dom_rect.width() as isize,
                height: dom_rect.height() as isize,
                top: dom_rect.top() as isize,
                right: dom_rect.right() as isize,
                bottom: dom_rect.bottom() as isize,
                left: dom_rect.left() as isize,
            })
        })
        .collect()
}
