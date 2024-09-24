use convert_case::{Case, Casing};
use floating_ui_leptos::{
    use_floating, IntoReference, Placement, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{html::Div, *};

use crate::utils::{all_placements::ALL_PLACEMENTS, use_size::use_size};

#[component]
pub fn Placement() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();

    let (rtl, set_rtl) = create_signal(false);
    let (placement, set_placement) = create_signal(Placement::Bottom);

    let UseFloatingReturn {
        floating_styles,
        update,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement.into())
            .while_elements_mounted_auto_update(),
    );

    let (size, set_size) = use_size(None, None);

    view! {
        <h1>Placement</h1>
        <p>
            The floating element should be correctly positioned when given each of the 12 placements.
        </p>
        <div class="container" style:direction=move || match rtl.get() {
            true => "rtl",
            false => "ltr",
        }>
            <div _ref=reference_ref class="reference">
                Reference
            </div>
            <div _ref=floating_ref class="floating" style=move || format!("{} width: {}px; height: {}px;", floating_styles.get(), size.get(), size.get())>
                Floating
            </div>
        </div>

        <div class="controls">
            <label for="size">Size</label>
            <input
                id="size"
                type="range"
                min="1"
                max="200"
                prop:value=size
                on:input=move |event| {
                    set_size.set(event_target_value(&event).parse().unwrap())
                }
            />
        </div>

        <div class="controls">
            <For
                each=|| ALL_PLACEMENTS
                key=|local_placement| format!("{:?}", local_placement)
                children=move |local_placement| view! {
                    <button
                        data-testid=format!("Placement{:?}", local_placement).to_case(Case::Kebab)
                        style:background-color=move || match placement.get() == local_placement {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_placement.set(local_placement)
                    >
                        {format!("{:?}", local_placement).to_case(Case::Kebab)}
                    </button>
                }
            />
        </div>

        <h2>RTL</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| {
                    let rtl_update = update.clone();

                    view! {
                        <button
                            data-testid=format!("rtl-{}", value)
                            style:background-color=move || match rtl.get() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_rtl.set(value);
                                rtl_update();
                            }
                        >
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>
    }
}
