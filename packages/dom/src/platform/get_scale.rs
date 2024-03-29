use floating_ui_utils::Coords;

use crate::{
    types::ElementOrVirtual,
    utils::get_css_dimensions::{get_css_dimensions, CssDimensions},
};

pub fn get_scale(element_or_virtual: ElementOrVirtual) -> Coords {
    let dom_element = element_or_virtual.resolve();

    if let Some(dom_element) = dom_element {
        let rect = dom_element.get_bounding_client_rect();
        let CssDimensions {
            dimensions,
            should_fallback,
        } = get_css_dimensions(&dom_element);
        let x = match should_fallback {
            true => rect.width().round(),
            false => rect.width(),
        } / dimensions.width;
        let y = match should_fallback {
            true => rect.height().round(),
            false => rect.height(),
        } / dimensions.height;

        // TODO: fallback?

        Coords { x, y }
    } else {
        Coords::new(1.0)
    }
}
