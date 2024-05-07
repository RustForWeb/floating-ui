use floating_ui_utils::dom::get_computed_style;
use web_sys::Element;

pub fn is_static_positioned(element: &Element) -> bool {
    get_computed_style(element)
        .get_property_value("position")
        .expect("Computed style should have position.")
        == "static"
}
