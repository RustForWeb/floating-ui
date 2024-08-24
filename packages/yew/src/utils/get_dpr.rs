use floating_ui_dom::dom::get_window;
use web_sys::Node;

pub fn get_dpr(element: &Node) -> f64 {
    get_window(Some(element)).device_pixel_ratio()
}
