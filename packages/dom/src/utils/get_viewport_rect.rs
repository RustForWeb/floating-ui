use floating_ui_utils::{
    Rect, Strategy,
    dom::{get_computed_style, get_document_element, get_window, is_web_kit},
};
use web_sys::Element;

use crate::utils::get_window_scroll_bar_x::get_window_scroll_bar_x;

// Safety check: ensure the scrollbar space is reasonable in case this calculation is affected by unusual styles.
// Most scrollbars leave 15-18px of space.
const SCROLLBAR_MAX: f64 = 25.0;

pub fn get_viewport_rect(element: &Element, strategy: Strategy) -> Rect {
    let window = get_window(Some(element));
    let html = get_document_element(Some(element.into()));
    let visual_viewport = window.visual_viewport();

    let mut x = 0.0;
    let mut y = 0.0;
    let mut width = html.client_width() as f64;
    let mut height = html.client_height() as f64;

    if let Some(visual_viewport) = visual_viewport {
        width = visual_viewport.width();
        height = visual_viewport.height();

        let visual_viewport_based = is_web_kit();
        if !visual_viewport_based || strategy == Strategy::Fixed {
            x = visual_viewport.offset_left();
            y = visual_viewport.offset_top();
        }
    }

    let window_scrollbar_x = get_window_scroll_bar_x(&html, None);
    // <html> `overflow: hidden` + `scrollbar-gutter: stable` reduces the visual width of the <html>,
    // but this is not considered in the size of `html.client_width`.
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
        let clipping_stable_scrollbar_width =
            ((html.client_width() as f64) - (body.client_width() as f64) - body_margin_inline)
                .abs();

        if clipping_stable_scrollbar_width <= SCROLLBAR_MAX {
            width -= clipping_stable_scrollbar_width;
        }
    } else if window_scrollbar_x <= SCROLLBAR_MAX {
        // If the <body> scrollbar is on the left, the width needs to be extended
        // by the scrollbar amount so there isn't extra space on the right.
        width += window_scrollbar_x;
    }

    Rect {
        x,
        y,
        width,
        height,
    }
}
