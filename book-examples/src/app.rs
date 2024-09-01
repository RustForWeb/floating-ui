use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<View> = vec![];

    #[cfg(feature = "placement")]
    {
        use crate::positioning::placement::PlacementDemo;
        views.push(view! {
            <PlacementDemo />
        });
    }
    #[cfg(feature = "shift")]
    {
        use crate::positioning::shift::ShiftDemo;
        views.push(view! {
            <ShiftDemo />
        });
    }
    #[cfg(feature = "flip")]
    {
        use crate::positioning::flip::FlipDemo;
        views.push(view! {
            <FlipDemo />
        });
    }
    #[cfg(feature = "size")]
    {
        use crate::positioning::size::SizeDemo;
        views.push(view! {
            <SizeDemo />
        });
    }

    views.into_view()
}
