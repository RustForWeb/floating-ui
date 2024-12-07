use floating_ui_leptos::{use_floating, Strategy, UseFloatingOptions, UseFloatingReturn};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn ContainingBlock() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (will_change, set_will_change) = signal("transform");
    let (contain, set_contain) = signal("paint");
    let (container_type, set_container_type) = signal::<Option<String>>(None);

    let UseFloatingReturn {
        floating_styles,
        update,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .strategy(Strategy::Absolute.into())
            .while_elements_mounted_auto_update(),
    );

    Effect::new(move || {
        _ = will_change.get();
        _ = contain.get();
        _ = container_type.get();

        update();
    });

    view! {
        <h1>Containing Block</h1>
        <p>The floating element should be correctly positioned.</p>
        <div
            class="container"
            style:will-change=move || match container_type.get() {
                Some(_) => "",
                None => will_change.get()
            }
            style:contain=move || match container_type.get() {
                Some(_) => "",
                None => contain.get()
            }
            style:container-type=move || match container_type.get() {
                Some(container_type) => container_type,
                _ => "".to_owned()
            }
        >
            <div node_ref=reference_ref class="reference">
                Reference
            </div>
            <div node_ref=floating_ref class="floating" style=floating_styles>
                Floating
            </div>
        </div>

        <h2>willChange</h2>
        <div class="controls">
            <For
                each=|| ["transform", "perspective", "transform, perspective", "opacity"]
                key=|local_will_change| format!("{:?}", local_will_change)
                children=move |local_will_change| view! {
                    <button
                        data-testid=format!("willchange-{}", local_will_change)
                        style:background-color=move || if will_change.get() == local_will_change {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_will_change.set(local_will_change)
                    >
                        {local_will_change}
                    </button>
                }
            />
        </div>

        <h2>contain</h2>
        <div class="controls">
            <For
                each=|| ["paint", "layout", "paint, layout", "strict", "content", "size"]
                key=|local_contain| format!("{:?}", local_contain)
                children=move |local_contain| view! {
                    <button
                        data-testid=format!("contain-{}", local_contain)
                        style:background-color=move || if contain.get() == local_contain {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_contain.set(local_contain)
                    >
                        {local_contain}
                    </button>
                }
            />
        </div>

        <h2>containerType</h2>
        <div class="controls">
            <For
                each=|| [None, Some("inline-size"), Some("size")]
                key=|local_container_type| format!("{:?}", local_container_type)
                children=move |local_container_type| view! {
                    <button
                        data-testid=format!("container-type-{}", local_container_type.unwrap_or("normal"))
                        style:background-color=move || if container_type.get() == local_container_type.map(|local_container_type| local_container_type.to_string()) {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_container_type.set(
                            local_container_type.map(|local_container_type| local_container_type.to_string())
                        )
                    >
                        {local_container_type.unwrap_or("normal")}
                    </button>
                }
            />
        </div>
    }
}
