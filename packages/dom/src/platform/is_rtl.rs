use floating_ui_utils::dom::get_computed_style;
use web_sys::Element;

pub fn is_rtl(element: &Element) -> bool {
    get_computed_style(element)
        .get_property_value("direction")
        .unwrap_or("".into())
        == "rtl"
}
