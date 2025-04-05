use floating_ui_leptos::{
    ARROW_NAME, Arrow, ArrowData, ArrowOptions, DetectOverflowOptions, Flip, FlipOptions,
    MiddlewareVec, Offset, OffsetOptions, Padding, Placement, Shift, ShiftOptions, Side,
    UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

#[component]
pub fn App() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();
    let arrow_ref = AnyNodeRef::new();

    let (open, set_open) = signal(false);

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(6.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default().detect_overflow(
            DetectOverflowOptions::default().padding(Padding::All(5.0)),
        ))),
        Box::new(Arrow::new(ArrowOptions::new(arrow_ref))),
    ];

    let UseFloatingReturn {
        placement,
        floating_styles,
        middleware_data,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .open(open.into())
            .placement(Placement::Top.into())
            .middleware(SendWrapper::new(middleware).into())
            .while_elements_mounted_auto_update(),
    );

    let static_side = Signal::derive(move || placement.get().side().opposite());
    let arrow_data =
        Signal::derive(move || -> Option<ArrowData> { middleware_data.get().get_as(ARROW_NAME) });
    let arrow_x = Signal::derive(move || {
        arrow_data
            .get()
            .and_then(|arrow_data| arrow_data.x.map(|x| format!("{}px", x)))
    });
    let arrow_y = Signal::derive(move || {
        arrow_data
            .get()
            .and_then(|arrow_data| arrow_data.y.map(|y| format!("{}px", y)))
    });

    view! {
        <button
            node_ref=reference_ref
            id="button"
            aria-describedby="tooltip"
            on:mouseenter=move |_| set_open.set(true)
            on:mouseleave=move |_| set_open.set(false)
            on:focus=move |_| set_open.set(true)
            on:blur=move |_| set_open.set(false)
        >
            My button
        </button>

        <div
            node_ref=floating_ref
            id="tooltip"
            role="tooltip"
            style:display=move || if open.get() {
                "block"
            } else {
                "none"
            }
            style:position=move || floating_styles.get().style_position()
            style:top=move || floating_styles.get().style_top()
            style:left=move || floating_styles.get().style_left()
            style:transform=move || floating_styles.get().style_transform().unwrap_or_default()
            style:will-change=move || floating_styles.get().style_will_change().unwrap_or_default()
        >
            My tooltip with more content
            <div
                node_ref=arrow_ref
                id="arrow"
                style:left=move || match static_side.get() {
                    Side::Left => Some("-4px".to_owned()),
                    _ => arrow_x.get()
                }.unwrap_or_default()
                style:top=move || match static_side.get() {
                    Side::Top => Some("-4px".to_owned()),
                    _ => arrow_y.get()
                }.unwrap_or_default()
                style:right=move || match static_side.get() {
                    Side::Right => Some("-4px"),
                    _ => None
                }.unwrap_or_default()
                style:bottom=move || match static_side.get() {
                    Side::Bottom => Some("-4px"),
                    _ => None
                }.unwrap_or_default()
            />
        </div>
    }
}
