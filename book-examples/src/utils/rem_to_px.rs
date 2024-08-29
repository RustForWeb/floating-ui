use leptos::{document, window};

pub fn rem_to_px(value: f64) -> f64 {
    document()
        .document_element()
        .map(|document_element| {
            value
                * window()
                    .get_computed_style(&document_element)
                    .expect("Valid element.")
                    .expect("Element should have computed style.")
                    .get_property_value("font-size")
                    .expect("Computed style should have font size.")
                    .replace("px", "")
                    .parse::<f64>()
                    .expect("Font size should be a float.")
        })
        .unwrap_or(value * 16.0)
}
