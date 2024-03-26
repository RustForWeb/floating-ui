use floating_ui_leptos::{
    use_floating, Middleware, Placement, Strategy, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{
    html::{Div, Span},
    *,
};
use web_sys::{Element, Window};

#[component]
pub fn App() -> impl IntoView {
    let open = create_rw_signal(false);

    let reference = create_node_ref::<Span>();
    let floating = create_node_ref::<Div>();

    let UseFloatingReturn {
        floating_styles, ..
    } = use_floating(
        reference,
        floating,
        UseFloatingOptions {
            open: open.into(),
            placement: None::<Placement>.into(),
            strategy: None::<Strategy>.into(),
            middleware: None::<Vec<&dyn Middleware<Element, Window>>>.into(),
            transform: None::<bool>.into(),
            while_elements_mounted: None::<bool>.into(),
        },
    );

    view! {
        <span _ref=reference on:click=move |_| open.update(move |open| *open = !*open)>Reference</span>
        <div _ref=floating style=move || format!("display: {}; {}", match open() {
            true => "block",
            false => "none"
        }, String::from(floating_styles()))>
            Floating
        </div>
    }
}
