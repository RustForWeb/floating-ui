use floating_ui_utils::Coords;

use crate::{
    types::ElementOrVirtual,
    utils::get_css_dimensions::{CssDimensions, get_css_dimensions},
};

pub fn get_scale(element_or_virtual: ElementOrVirtual) -> Coords {
    let dom_element = element_or_virtual.resolve();

    if let Some(dom_element) = dom_element {
        let rect = dom_element.get_bounding_client_rect();
        let CssDimensions {
            dimensions,
            should_fallback,
        } = get_css_dimensions(&dom_element);
        let mut x = if should_fallback {
            rect.width().round()
        } else {
            rect.width()
        } / dimensions.width;
        let mut y = if should_fallback {
            rect.height().round()
        } else {
            rect.height()
        } / dimensions.height;

        if x == 0.0 || x.is_nan() || x.is_infinite() {
            x = 1.0;
        }

        if y == 0.0 || y.is_nan() || y.is_infinite() {
            y = 1.0;
        }

        Coords { x, y }
    } else {
        Coords::new(1.0)
    }
}
