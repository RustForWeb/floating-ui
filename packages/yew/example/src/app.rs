use floating_ui_yew::{
    use_auto_update, use_floating, Arrow, ArrowData, ArrowOptions, DetectOverflowOptions, Flip,
    FlipOptions, MiddlewareVec, Offset, OffsetOptions, Padding, Placement, Shift, ShiftOptions,
    Side, UseFloatingOptions, UseFloatingReturn, ARROW_NAME,
};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let reference_ref = use_node_ref();
    let floating_ref = use_node_ref();
    let arrow_ref = use_node_ref();

    let open = use_state_eq(|| false);

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(6.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default().detect_overflow(
            DetectOverflowOptions::default().padding(Padding::All(5.0)),
        ))),
        Box::new(Arrow::new(ArrowOptions::new(arrow_ref.clone()))),
    ];

    let auto_update = use_auto_update();

    let UseFloatingReturn {
        placement,
        floating_styles,
        middleware_data,
        ..
    } = use_floating(
        reference_ref.clone().into(),
        floating_ref.clone(),
        UseFloatingOptions::default()
            .open(*open)
            .placement(Placement::Top)
            .middleware(middleware)
            .while_elements_mounted((*auto_update).clone()),
    );

    let static_side = placement.side().opposite();
    let arrow_data = use_memo(
        middleware_data,
        move |middleware_data| -> Option<ArrowData> { middleware_data.get_as(ARROW_NAME) },
    );
    let arrow_x = use_memo(arrow_data.clone(), |arrow_data| {
        arrow_data
            .as_ref()
            .clone()
            .and_then(|arrow_data| arrow_data.x.map(|x| format!("{}px", x)))
    });
    let arrow_y = use_memo(arrow_data.clone(), |arrow_data| {
        arrow_data
            .as_ref()
            .clone()
            .and_then(|arrow_data| arrow_data.y.map(|y| format!("{}px", y)))
    });

    let onmouseenter = use_callback(open.clone(), |_, open| open.set(true));
    let onmouseleave = use_callback(open.clone(), |_, open| open.set(false));
    let onfocus = use_callback(open.clone(), |_, open| open.set(true));
    let onblur = use_callback(open.clone(), |_, open| open.set(false));

    html! {
        <>
            <button
                ref={reference_ref}
                id="button"
                aria-describedby="tooltip"
                onmouseenter={onmouseenter}
                onmouseleave={onmouseleave}
                onfocus={onfocus}
                onblur={onblur}
            >
                {"My button"}
            </button>

            <div
                ref={floating_ref}
                id="tooltip"
                role="tooltip"
                style={format!(
                    "display: {}; {}",
                    match *open {
                        true => "block",
                        false => "none",
                    },
                    *floating_styles
                )}
            >
                {"My tooltip with more content"}
                <div
                    ref={arrow_ref}
                    id="arrow"
                    style={format!(
                        "{}{}{}{}",
                        (match static_side {
                            Side::Left => Some("-4px".into()),
                            _ => (*arrow_x).clone()
                        }).map(|value| format!("left: {value};")).unwrap_or("".into()),
                        (match static_side {
                            Side::Top => Some("-4px".into()),
                            _ => (*arrow_y).clone()
                        }).map(|value| format!("top: {value};")).unwrap_or("".into()),
                        (match static_side {
                            Side::Right => Some("-4px"),
                            _ => None
                        }).map(|value| format!("right: {value};")).unwrap_or("".into()),
                        (match static_side {
                            Side::Bottom => Some("-4px"),
                            _ => None
                        }).map(|value| format!("left: {value};")).unwrap_or("".into()),
                    )}
                />
            </div>
        </>
    }
}
