use floating_ui_utils::{
    dom::{
        get_document_element, get_node_name, get_node_scroll, is_overflow_element, ElementOrWindow,
        NodeScroll,
    },
    Coords, Rect, Strategy,
};

use crate::{
    utils::{
        get_bounding_client_rect::get_bounding_client_rect,
        get_window_scroll_bar_x::get_window_scroll_bar_x,
    },
    ElementOrVirtual,
};

pub fn get_rect_relative_to_offset_parent(
    element_or_virtual: ElementOrVirtual,
    offset_parent: ElementOrWindow,
    strategy: Strategy,
) -> Rect {
    let is_offset_parent_an_element = matches!(offset_parent, ElementOrWindow::Element(_));
    let document_element = get_document_element(Some((&offset_parent).into()));
    let is_fixed = strategy == Strategy::Fixed;
    let rect = get_bounding_client_rect(
        element_or_virtual,
        true,
        is_fixed,
        Some(offset_parent.clone()),
    );

    let mut scroll = NodeScroll::new(0.0);
    let mut offsets = Coords::new(0.0);

    #[allow(clippy::nonminimal_bool)]
    if is_offset_parent_an_element || (!is_offset_parent_an_element && !is_fixed) {
        if get_node_name((&offset_parent).into()) != "body"
            || is_overflow_element(&document_element)
        {
            scroll = get_node_scroll(offset_parent.clone());
        }

        match offset_parent {
            ElementOrWindow::Element(offset_parent) => {
                let offset_rect = get_bounding_client_rect(
                    offset_parent.into(),
                    true,
                    is_fixed,
                    Some(offset_parent.into()),
                );
                offsets.x = offset_rect.x + offset_parent.client_left() as f64;
                offsets.y = offset_rect.y + offset_parent.client_top() as f64;
            }
            ElementOrWindow::Window(_) => {
                offsets.x = get_window_scroll_bar_x(&document_element);
            }
        }
    }

    let x = rect.left + scroll.scroll_left - offsets.x;
    let y = rect.top + scroll.scroll_top - offsets.y;

    Rect {
        x,
        y,
        width: rect.width,
        height: rect.height,
    }
}
