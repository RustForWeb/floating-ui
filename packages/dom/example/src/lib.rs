use std::rc::Rc;

use floating_ui_dom::{
    compute_position, ComputePositionConfig, ComputePositionReturn, Flip, FlipOptions, Offset,
    OffsetOptions, Placement,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("Window should exist.");
    let document = window.document().expect("Window should have document.");

    let button = Rc::new(
        document
            .get_element_by_id("button")
            .expect("Button should exist.")
            .unchecked_into::<HtmlElement>(),
    );
    let tooltip = Rc::new(
        document
            .get_element_by_id("tooltip")
            .expect("Tooltip should exist.")
            .unchecked_into::<HtmlElement>(),
    );

    fn update(button: &HtmlElement, tooltip: &HtmlElement) -> Result<(), JsValue> {
        let ComputePositionReturn { x, y, .. } = compute_position(
            button,
            tooltip,
            Some(ComputePositionConfig {
                placement: Some(Placement::Top),
                strategy: None,
                middleware: Some(vec![
                    &Offset::new(OffsetOptions::Value(6.0)),
                    &Flip::new(FlipOptions::default()),
                ]),
            }),
        );

        let style = tooltip.style();
        style.set_property("left", &format!("{x}px"))?;
        style.set_property("top", &format!("{y}px"))?;

        Ok(())
    }

    {
        let button_clone = button.clone();
        let tooltip_clone = tooltip.clone();

        let show_tooltip = Closure::<dyn Fn()>::new(move || {
            tooltip_clone
                .style()
                .set_property("display", "block")
                .unwrap();
            update(&button_clone, &tooltip_clone).unwrap();
        });

        button.add_event_listener_with_callback(
            "mouseenter",
            show_tooltip.as_ref().unchecked_ref(),
        )?;
        button.add_event_listener_with_callback("focus", show_tooltip.as_ref().unchecked_ref())?;

        show_tooltip.forget();
    }

    {
        let tooltip_clone = tooltip.clone();

        let hide_tooltip = Closure::<dyn Fn()>::new(move || {
            tooltip_clone
                .style()
                .set_property("display", "none")
                .unwrap();
        });

        button.add_event_listener_with_callback(
            "mouseleave",
            hide_tooltip.as_ref().unchecked_ref(),
        )?;
        button.add_event_listener_with_callback("blur", hide_tooltip.as_ref().unchecked_ref())?;

        hide_tooltip.forget();
    }

    Ok(())
}
