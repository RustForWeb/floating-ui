use floating_ui_leptos::{
    use_floating, IntoReference, Strategy, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{html::Div, *};

#[component]
pub fn ContainingBlock() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();

    let (will_change, set_will_change) = create_signal("transform");
    let (contain, set_contain) = create_signal("paint");
    let (container_type, set_container_type) = create_signal::<Option<String>>(None);

    let UseFloatingReturn {
        floating_styles,
        update,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .strategy(Strategy::Absolute.into())
            .while_elements_mounted_auto_update(),
    );

    let will_change_update = update.clone();
    let contain_update = update.clone();
    let container_type_update = update.clone();

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
                _ => "".into()
            }
        >
            <div _ref=reference_ref class="reference">
                Reference
            </div>
            <div _ref=floating_ref class="floating" style=floating_styles>
                Floating
            </div>
        </div>

        <h2>willChange</h2>
        <div class="controls">
            <For
                each=|| ["transform", "perspective", "transform, perspective", "opacity"]
                key=|local_will_change| format!("{:?}", local_will_change)
                children=move |local_will_change| {
                    let will_change_update = will_change_update.clone();
                    view! {
                        <button
                            data-testid=format!("willchange-{}", local_will_change)
                            style:background-color=move || match will_change.get() == local_will_change {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_will_change.set(local_will_change);
                                will_change_update();
                            }
                        >
                            {local_will_change}
                        </button>
                    }
                }
            />
        </div>

        <h2>contain</h2>
        <div class="controls">
            <For
                each=|| ["paint", "layout", "paint, layout", "strict", "content", "size"]
                key=|local_contain| format!("{:?}", local_contain)
                children=move |local_contain| {
                    let contain_update = contain_update.clone();
                    view! {
                        <button
                            data-testid=format!("contain-{}", local_contain)
                            style:background-color=move || match contain.get() == local_contain {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_contain.set(local_contain);
                                contain_update();
                            }
                        >
                            {local_contain}
                        </button>
                    }
                }
            />
        </div>

        <h2>containerType</h2>
        <div class="controls">
            <For
                each=|| [None, Some("inline-size"), Some("size")]
                key=|local_container_type| format!("{:?}", local_container_type)
                children=move |local_container_type| {
                    let container_type_update = container_type_update.clone();
                    view! {
                        <button
                            data-testid=format!("container-type-{}", local_container_type.unwrap_or("normal"))
                            style:background-color=move || match container_type.get() == local_container_type.map(|local_container_type| local_container_type.to_string()) {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_container_type.set(local_container_type.map(|local_container_type| local_container_type.to_string()));
                                container_type_update();
                            }
                        >
                            {local_container_type.unwrap_or("normal")}
                        </button>
                    }
                }
            />
        </div>
    }
}
