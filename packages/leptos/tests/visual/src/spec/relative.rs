use leptos::*;

#[component]
pub fn Relative() -> impl IntoView {
    view! {
        <h1>Relative</h1>
        <p>
            The floating element should be positioned correctly on the bottom when a
            certain parent node has <code>position: relative</code> applied.
        </p>
    }
}
