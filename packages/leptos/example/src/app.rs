use floating_ui_leptos::{
    use_floating, Arrow, ArrowOptions, MiddlewareVec, Offset, OffsetOptions, Placement,
    UseFloatingOptions, UseFloatingReturn,
};
use leptos::{
    html::{Div, Span},
    *,
};

#[component]
pub fn App() -> impl IntoView {
    let open = create_rw_signal(false);
    let placement = create_rw_signal(Placement::Bottom);

    let reference = create_node_ref::<Span>();
    let floating = create_node_ref::<Div>();
    let floating_arrow = create_node_ref::<Div>();

    let middleware: MiddlewareVec = vec![
        Box::new(Arrow::new(ArrowOptions::new(floating_arrow))),
        Box::new(Offset::new(OffsetOptions::Value(4.0))),
    ];

    let UseFloatingReturn {
        floating_styles, ..
    } = use_floating(
        reference,
        floating,
        UseFloatingOptions::default()
            .open(open.into())
            .placement(placement.into())
            .middleware(middleware.into()),
    );

    view! {
        <span _ref=reference on:click=move |_| open.update(move |open| *open = !*open)>Reference</span>
        <div _ref=floating style=move || format!("display: {}; {}", match open() {
            true => "block",
            false => "none"
        }, String::from(floating_styles()))>
            Floating

            <div _ref=floating_arrow style:position="absolute" style:left="0px" style:top="0px" />
        </div>

        <p>
            <button on:click=move |_| placement.set(Placement::Right)>Change placement</button>
        </p>
    }
}
