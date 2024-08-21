use floating_ui_dom::dom::get_window;
use web_sys::Element;

pub fn get_dpr(element: &Element) -> f64 {
    get_window(Some(element)).device_pixel_ratio()
}
