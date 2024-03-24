use std::rc::Rc;

use floating_ui_dom::{
    compute_position, get_opposite_side, get_side, Arrow, ArrowData, ArrowOptions,
    ComputePositionConfig, ComputePositionReturn, DetectOverflowOptions, Flip, FlipOptions, Offset,
    OffsetOptions, Padding, Placement, Shift, ShiftOptions, Side, ARROW_NAME,
};
use log::Level;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).expect("Console logger should be available.");

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
    let arrow = Rc::new(
        document
            .get_element_by_id("arrow")
            .expect("Arrow should exist.")
            .unchecked_into::<HtmlElement>(),
    );

    fn update(
        button: &HtmlElement,
        tooltip: &HtmlElement,
        arrow: &HtmlElement,
    ) -> Result<(), JsValue> {
        let ComputePositionReturn {
            x,
            y,
            placement,
            middleware_data,
            ..
        } = compute_position(
            button,
            tooltip,
            Some(ComputePositionConfig {
                placement: Some(Placement::Top),
                strategy: None,
                middleware: Some(vec![
                    &Offset::new(OffsetOptions::Value(6.0)),
                    &Flip::new(FlipOptions::default()),
                    &Shift::new(ShiftOptions {
                        detect_overflow: Some(DetectOverflowOptions {
                            boundary: None,
                            root_boundary: None,
                            element_context: None,
                            alt_boundary: None,
                            padding: Some(Padding::All(5.0)),
                        }),
                        main_axis: None,
                        cross_axis: None,
                        limiter: None,
                    }),
                    &Arrow::new(ArrowOptions {
                        element: arrow,
                        padding: None,
                    }),
                ]),
            }),
        );

        let arrow_data: Option<ArrowData> = middleware_data.get_as(ARROW_NAME);
        if let Some(arrow_data) = arrow_data {
            let static_side = get_opposite_side(get_side(placement));

            let arrow_x = arrow_data.x.map_or(String::new(), |x| format!("{x}px"));
            let arrow_y = arrow_data.y.map_or(String::new(), |y| format!("{y}px"));

            let style = arrow.style();
            style.set_property(
                "left",
                match static_side {
                    Side::Left => "-4px",
                    _ => &arrow_x,
                },
            )?;
            style.set_property(
                "top",
                match static_side {
                    Side::Top => "-4px",
                    _ => &arrow_y,
                },
            )?;
            style.set_property(
                "right",
                match static_side {
                    Side::Right => "-4px",
                    _ => "",
                },
            )?;
            style.set_property(
                "bottom",
                match static_side {
                    Side::Bottom => "-4px",
                    _ => "",
                },
            )?;
        }

        let style = tooltip.style();
        style.set_property("left", &format!("{x}px"))?;
        style.set_property("top", &format!("{y}px"))?;

        Ok(())
    }

    {
        let button_clone = button.clone();
        let tooltip_clone = tooltip.clone();
        let arrow_clone = arrow.clone();

        let show_tooltip = Closure::<dyn Fn()>::new(move || {
            tooltip_clone
                .style()
                .set_property("display", "block")
                .unwrap();
            update(&button_clone, &tooltip_clone, &arrow_clone).unwrap();
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
