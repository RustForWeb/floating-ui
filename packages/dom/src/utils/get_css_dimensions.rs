use floating_ui_utils::{
    dom::{get_computed_style, is_html_element},
    Dimensions,
};
use web_sys::{wasm_bindgen::JsCast, Element, HtmlElement};

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
        .unwrap_or(0.0);
    let height = css
        .get_property_value("height")
        .expect("Computed style should have height.")
        .replace("px", "")
        .parse::<f64>()
        .unwrap_or(0.0);

    let offset_width;
    let offset_height;
    if is_html_element(element) {
        let element = element.unchecked_ref::<HtmlElement>();
        offset_width = element.offset_width() as f64;
        offset_height = element.offset_height() as f64;
    } else {
        offset_width = width;
        offset_height = height;
    }
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
