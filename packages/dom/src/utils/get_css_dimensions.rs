use floating_ui_utils::{dom::get_computed_style, Dimensions};
use web_sys::Element;

#[derive(Clone, Debug)]
pub struct CssDimensions {
    pub dimensions: Dimensions,
    pub should_fallback: bool,
}

pub fn get_css_dimensions(element: &Element) -> CssDimensions {
    let css = get_computed_style(element);

    let width = css
        .get_property_value("width")
        .expect("Computed style should have width.")
        .replace("px", "")
        .parse::<f64>()
        .expect("Width should be a number.");
    let height = css
        .get_property_value("width")
        .expect("Computed style should have height.")
        .replace("px", "")
        .parse::<f64>()
        .expect("Height should be a number.");

    // TODO: check if element is HtmlElement?
    let offset_width = width;
    let offset_height = height;
    let should_fallback = width.round() != offset_width || height.round() != offset_height;

    CssDimensions {
        dimensions: match should_fallback {
            true => Dimensions {
                width: offset_width,
                height: offset_height,
            },
            false => Dimensions { width, height },
        },
        should_fallback,
    }
}
