use web_sys::Element;

use crate::utils::get_dpr::get_dpr;

pub fn round_by_dpr(element: &Element, value: f64) -> f64 {
    let dpr = get_dpr(element);
    (value * dpr).round() / dpr
}
