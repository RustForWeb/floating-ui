use floating_ui_utils::{
    dom::{
        get_document_element, get_node_name, get_node_scroll, is_overflow_element,
        DomElementOrWindow, NodeScroll,
    },
    Coords, Rect, Strategy,
};

use crate::{
    types::ElementOrVirtual,
    utils::{
        get_bounding_client_rect::get_bounding_client_rect, get_html_offset::get_html_offset,
        get_window_scroll_bar_x::get_window_scroll_bar_x,
    },
};

pub fn get_rect_relative_to_offset_parent(
    element_or_virtual: ElementOrVirtual,
    offset_parent: DomElementOrWindow,
    strategy: Strategy,
) -> Rect {
    let is_offset_parent_an_element = matches!(offset_parent, DomElementOrWindow::Element(_));
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
            DomElementOrWindow::Element(offset_parent) => {
                let offset_rect = get_bounding_client_rect(
                    offset_parent.into(),
                    true,
                    is_fixed,
                    Some(offset_parent.into()),
                );
                offsets.x = offset_rect.x + offset_parent.client_left() as f64;
                offsets.y = offset_rect.y + offset_parent.client_top() as f64;
            }
            DomElementOrWindow::Window(_) => {
                // If the <body> scrollbar appears on the left (e.g. RTL systems).
                // Use Firefox with layout.scrollbar.side = 3 in about:config to test this.
                offsets.x = get_window_scroll_bar_x(&document_element, None);
            }
        }
    }

    let html_offset = if !is_offset_parent_an_element && !is_fixed {
        get_html_offset(&document_element, &scroll, None)
    } else {
        Coords::new(0.0)
    };

    let x = rect.left + scroll.scroll_left - offsets.x - html_offset.x;
    let y = rect.top + scroll.scroll_top - offsets.y - html_offset.y;

    Rect {
        x,
        y,
        width: rect.width,
        height: rect.height,
    }
}
