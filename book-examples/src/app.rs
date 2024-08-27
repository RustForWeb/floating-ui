use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<View> = vec![];

    #[cfg(feature = "placement")]
    {
        use crate::placement::PlacementDemo;
        views.push(view! {
            <PlacementDemo />
        });
    }

    views.into_view()
}
