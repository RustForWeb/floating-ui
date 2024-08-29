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
    #[cfg(feature = "shift")]
    {
        use crate::shift::ShiftDemo;
        views.push(view! {
            <ShiftDemo />
        });
    }
    #[cfg(feature = "flip")]
    {
        use crate::flip::FlipDemo;
        views.push(view! {
            <FlipDemo />
        });
    }
    #[cfg(feature = "size")]
    {
        use crate::size::SizeDemo;
        views.push(view! {
            <SizeDemo />
        });
    }

    views.into_view()
}
