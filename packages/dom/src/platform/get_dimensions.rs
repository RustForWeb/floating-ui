use floating_ui_utils::Dimensions;
use web_sys::Element;

use crate::utils::get_css_dimensions::{CssDimensions, get_css_dimensions};

pub fn get_dimensions(element: &Element) -> Dimensions {
    let CssDimensions { dimensions, .. } = get_css_dimensions(element);
    dimensions
}
