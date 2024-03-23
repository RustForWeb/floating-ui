use floating_ui_dom::{
    compute_position, ComputePositionConfig, ComputePositionReturn, Offset, OffsetOptions,
    Placement,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("Window should exist.");
    let document = window.document().expect("Window should have document.");

    let button = document
        .get_element_by_id("button")
        .expect("Button should exist.");
    let tooltip = document
        .get_element_by_id("tooltip")
        .expect("Tooltip should exist.");

    let ComputePositionReturn { x, y, .. } = compute_position(
        &button,
        &tooltip,
        Some(ComputePositionConfig {
            placement: Some(Placement::Right),
            strategy: None,
            middleware: Some(vec![&Offset::new(OffsetOptions::Value(10.0))]),
        }),
    );

    let style = tooltip.unchecked_into::<HtmlElement>().style();
    style.set_property("left", &format!("{x}px"))?;
    style.set_property("top", &format!("{y}px"))?;

    Ok(())
}
