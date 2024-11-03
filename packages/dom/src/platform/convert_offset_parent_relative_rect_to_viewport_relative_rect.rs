use floating_ui_core::ConvertOffsetParentRelativeRectToViewportRelativeRectArgs;
use floating_ui_utils::{
    dom::{
        get_document_element, get_node_name, get_node_scroll, is_overflow_element, is_top_layer,
        NodeScroll,
    },
    Coords, ElementOrWindow, Rect, Strategy,
};
use web_sys::{Element, Window};

use crate::{
    platform::get_scale::get_scale,
    utils::{get_bounding_client_rect::get_bounding_client_rect, get_html_offset::get_html_offset},
};

pub fn convert_offset_parent_relative_rect_to_viewport_relative_rect(
    ConvertOffsetParentRelativeRectToViewportRelativeRectArgs {
        elements,
        rect,
        offset_parent,
        strategy,
    }: ConvertOffsetParentRelativeRectToViewportRelativeRectArgs<Element, Window>,
) -> Rect {
    let is_fixed = strategy == Strategy::Fixed;
    let document_element = get_document_element(
        offset_parent
            .as_ref()
            .map(|offset_parent| offset_parent.into()),
    );
    let top_layer = elements.map_or(false, |elements| is_top_layer(elements.floating));

    if offset_parent
        .as_ref()
        .is_some_and(|offset_parent| match offset_parent {
            ElementOrWindow::Element(element) => *element == &document_element,
            ElementOrWindow::Window(_) => false,
        })
        || (top_layer && is_fixed)
    {
        return rect;
    }

    let mut scroll = NodeScroll::new(0.0);
    let mut scale = Coords::new(1.0);
    let mut offsets = Coords::new(0.0);
    let is_offset_parent_an_element =
        offset_parent
            .as_ref()
            .is_some_and(|offset_parent| match offset_parent {
                ElementOrWindow::Element(_) => true,
                ElementOrWindow::Window(_) => false,
            });

    #[allow(clippy::nonminimal_bool)]
    if is_offset_parent_an_element || (!is_offset_parent_an_element && !is_fixed) {
        if let Some(offset_parent) = offset_parent.as_ref() {
            if get_node_name(offset_parent.into()) != "body"
                || is_overflow_element(&document_element)
            {
                scroll = get_node_scroll(offset_parent.into());
            }
        }

        if let Some(ElementOrWindow::Element(offset_parent)) = offset_parent {
            let offset_rect = get_bounding_client_rect(offset_parent.into(), false, false, None);
            scale = get_scale(offset_parent.into());
            offsets.x = offset_rect.x + offset_parent.client_left() as f64;
            offsets.y = offset_rect.y + offset_parent.client_top() as f64;
        }
    }

    let html_offset = if !is_offset_parent_an_element && !is_fixed {
        get_html_offset(&document_element, &scroll, Some(true))
    } else {
        Coords::new(0.0)
    };

    Rect {
        x: rect.x * scale.x - scroll.scroll_left * scale.x + offsets.x + html_offset.x,
        y: rect.y * scale.y - scroll.scroll_top * scale.y + offsets.y + html_offset.y,
        width: rect.width * scale.x,
        height: rect.height * scale.y,
    }
}
