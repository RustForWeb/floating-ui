use web_sys::Node;

use crate::utils::get_dpr::get_dpr;

pub fn round_by_dpr(element: &Node, value: f64) -> f64 {
    let dpr = get_dpr(element);
    (value * dpr).round() / dpr
}
