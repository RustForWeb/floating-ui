use std::rc::Rc;

use dioxus::prelude::*;
use floating_ui_dioxus::{
    ARROW_NAME, Arrow, ArrowData, ArrowOptions, DetectOverflowOptions, Flip, FlipOptions,
    MiddlewareVec, Offset, OffsetOptions, Padding, Placement, Shift, ShiftOptions, Side,
    UseFloatingOptions, UseFloatingReturn, use_auto_update, use_floating,
};

#[component]
pub fn App() -> Element {
    let mut reference_ref = use_signal(|| None::<Rc<MountedData>>);
    let mut floating_ref = use_signal(|| None::<Rc<MountedData>>);
    let mut arrow_ref = use_signal(|| None::<Rc<MountedData>>);

    let mut open = use_signal(|| false);

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(6.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default().detect_overflow(
            DetectOverflowOptions::default().padding(Padding::All(5.0)),
        ))),
        Box::new(Arrow::new(ArrowOptions::new(arrow_ref))),
    ];

    let auto_update = use_auto_update();

    let UseFloatingReturn {
        placement,
        floating_styles,
        middleware_data,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .open(open())
            .placement(Placement::Top)
            .middleware(middleware)
            .while_elements_mounted((*(auto_update())).clone()),
    );

    let static_side = use_memo(move || placement().side().opposite());
    let arrow_data =
        use_memo(move || -> Option<ArrowData> { middleware_data().get_as(ARROW_NAME) });
    let arrow_x = use_memo(move || {
        arrow_data().and_then(|arrow_data| arrow_data.x.map(|x| format!("{x}px")))
    });
    let arrow_y = use_memo(move || {
        arrow_data().and_then(|arrow_data| arrow_data.y.map(|y| format!("{y}px")))
    });

    rsx! {
        button {
            id: "button",
            aria_describedby: "tooltip",
            onmounted: move |event| {
                reference_ref.set(Some(event.data()));
            },
            onmouseenter: move |_| open.set(true),
            onmouseleave: move |_| open.set(false),
            onfocus: move |_| open.set(true),
            onblur: move |_| open.set(false),

            "My button"
        }

        div {
            id: "tooltip",
            role: "tooltip",
            style: format!(
                "display: {}; {}",
                if open() {
                    "block"
                } else {
                    "none"
                },
                floating_styles()
            ),
            onmounted: move |event| {
                floating_ref.set(Some(event.data()));
            },

            "My tooltip with more content"
            div {
                id: "arrow",
                style: format!(
                    "{}{}{}{}",
                    (match static_side() {
                        Side::Left => Some("-4px".to_owned()),
                        _ => arrow_x().clone()
                    }).map(|value| format!("left: {value};")).unwrap_or_default(),
                    (match static_side() {
                        Side::Top => Some("-4px".to_owned()),
                        _ => arrow_y().clone()
                    }).map(|value| format!("top: {value};")).unwrap_or_default(),
                    (match static_side() {
                        Side::Right => Some("-4px"),
                        _ => None
                    }).map(|value| format!("right: {value};")).unwrap_or_default(),
                    (match static_side() {
                        Side::Bottom => Some("-4px"),
                        _ => None
                    }).map(|value| format!("left: {value};")).unwrap_or_default(),
                ),
                onmounted: move |event| {
                    arrow_ref.set(Some(event.data()));
                },
            }
        }
    }
}
