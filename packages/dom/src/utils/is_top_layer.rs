use web_sys::Element;

const TOP_LAYER_SELECTORS: [&str; 2] = [":popover-open", ":modal"];

pub fn is_top_layer(floating: &Element) -> bool {
    TOP_LAYER_SELECTORS
        .into_iter()
        .any(|selector| floating.matches(selector).unwrap_or(false))
}
