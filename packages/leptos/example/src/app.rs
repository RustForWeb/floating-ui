use floating_ui_leptos::{use_floating, UseFloatingOptions, UseFloatingReturn};
use leptos::{
    html::{Div, Span},
    *,
};

#[component]
pub fn App() -> impl IntoView {
    let reference = create_node_ref::<Span>();
    let floating = create_node_ref::<Div>();

    let UseFloatingReturn {
        floating_styles, ..
    } = use_floating(reference, floating, UseFloatingOptions::default().into());

    view! {
        <span _ref=reference>Reference</span>
        <div _ref=floating style=floating_styles>
            Floating
        </div>
    }
}
