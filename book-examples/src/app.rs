use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<AnyView> = vec![];

    #[cfg(feature = "placement")]
    {
        use crate::positioning::placement::PlacementDemo;
        views.push(
            view! {
                <PlacementDemo />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "shift")]
    {
        use crate::positioning::shift::ShiftDemo;
        views.push(
            view! {
                <ShiftDemo />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "flip")]
    {
        use crate::positioning::flip::FlipDemo;
        views.push(
            view! {
                <FlipDemo />
            }
            .into_any(),
        );
    }
    #[cfg(feature = "size")]
    {
        use crate::positioning::size::SizeDemo;
        views.push(
            view! {
                <SizeDemo />
            }
            .into_any(),
        );
    }

    views.into_view()
}
