use floating_ui_utils::{
    Rect, Strategy,
    dom::{get_computed_style, get_document_element, get_window, is_web_kit},
};
use web_sys::Element;

use crate::utils::get_window_scroll_bar_x::get_window_scroll_bar_x;

// Safety check: ensure the scrollbar space is reasonable in case this calculation is affected by unusual styles.
// Most scrollbars leave 15-18px of space.
const SCROLLBAR_MAX: f64 = 25.0;

#[derive(Clone, Debug, PartialEq)]
pub enum ViewportRootBoundary {
    Viewport,
    LayoutViewport,
}

pub fn get_viewport_rect(
    element: &Element,
    strategy: Strategy,
    root_boundary: ViewportRootBoundary,
) -> Rect {
    let is_layout_viewport = root_boundary == ViewportRootBoundary::LayoutViewport;
    let window = get_window(Some(element));
    let html = get_document_element(Some(element.into()));
    let visual_viewport = window.visual_viewport();

    let mut x = 0.0;
    let mut y = 0.0;
    let mut width = html.client_width() as f64;
    let mut height = html.client_height() as f64;

    if let Some(visual_viewport) = visual_viewport {
        // Client coordinates are relative to the layout viewport, except in WebKit with an `absolute` strategy,
        // where they are relative to the visual viewport.
        let layout_relative_client_coords = !is_web_kit() || strategy == Strategy::Fixed;

        if is_layout_viewport {
            if !layout_relative_client_coords {
                x = -visual_viewport.offset_left();
                y = -visual_viewport.offset_top();
            }
        } else {
            width = visual_viewport.width();
            height = visual_viewport.height();

            if layout_relative_client_coords {
                x = visual_viewport.offset_left();
                y = visual_viewport.offset_top();
            }
        }
    }

    let window_scrollbar_x = get_window_scroll_bar_x(&html, None);
    // `scrollbar-gutter: stable` on the <html> reserves gutter space that shrinks
    // the visual width but isn't reflected in `html.clientWidth`, so subtract it.
    // Only the inline-end (right) gutter can hold the scrollbar; `both-edges` also
    // reserves an empty inline-start gutter that clips nothing, so exclude just
    // the one scrollbar-side gutter — halve the measured (two-gutter) total. A
    // left-side scrollbar (`window_scroll_bar_x > 0`) is already handled by
    // `get_html_offset`/`visual_viewport.width`; skip it here.
    if window_scrollbar_x <= 0.0 {
        let doc = html
            .owner_document()
            .expect("Element should have owner document.");
        let body = doc.body().expect("Document should have body.");
        let body_styles = get_computed_style(&body);
        let body_margin_inline = if doc.compat_mode() == "CSS1Compat" {
            body_styles
                .get_property_value("margin-left")
                .expect("Computed style should have margin left.")
                .parse::<f64>()
                .unwrap_or(0.0)
                + body_styles
                    .get_property_value("margin-right")
                    .expect("Computed style should have margin right.")
                    .parse::<f64>()
                    .unwrap_or(0.0)
        } else {
            0.0
        };
        let reserved_width =
            ((html.client_width() as f64) - (body.client_width() as f64) - body_margin_inline)
                .abs();
        let gutter = if get_computed_style(&html)
            .get_property_value("scrollbar-gutter")
            .ok()
            .as_deref()
            == Some("stable both-edges")
        {
            reserved_width / 2.0
        } else {
            reserved_width
        };

        if gutter <= SCROLLBAR_MAX {
            width -= gutter;
        }
    }

    Rect {
        x,
        y,
        width,
        height,
    }
}
