use floating_ui_leptos::{use_floating, Placement, UseFloatingOptions, UseFloatingReturn};
use leptos::{html::Div, *};

use crate::utils::all_placements::ALL_PLACEMENTS;

#[component]
pub fn Placement() -> impl IntoView {
    let reference = create_node_ref::<Div>();
    let floating = create_node_ref::<Div>();

    let (rtl, set_rtl) = create_signal(false);
    let (placement, set_placement) = create_signal(Placement::Bottom);
    let UseFloatingReturn {
        floating_styles,
        update,
        ..
    } = use_floating(
        reference,
        floating,
        UseFloatingOptions::default()
            .placement(placement.into())
            .while_elements_mounted_auto_update(),
    );
    let (size, set_size) = create_signal(80);

    _ = watch(
        rtl,
        move |_, _, _| {
            update();
        },
        false,
    );

    view! {
        <h1>Placement</h1>
        <p>
            The floating element should be correctly positioned when given each of the 12 placements.
        </p>
        <div class="container" style=move || match rtl() {
            true => "direction: rtl;",
            false => "direction: ltr;",
        }>
            <div _ref=reference class="reference">
                Reference
            </div>
            <div _ref=floating class="floating" style=move || format!("{} width: {}px; height: {}px;", String::from(floating_styles()), size(), size())>
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
                    set_size(event_target_value(&event).parse().unwrap())
                }
            />
        </div>

        <div class="controls">
            <For
                each=|| ALL_PLACEMENTS
                key=|local_placement| format!("{:?}", local_placement)
                children=move |local_placement| view! {
                    <button
                        data-testid=format!("placement-{:?}", local_placement)
                        style:background-color=move || match placement() == local_placement {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_placement(local_placement)
                    >
                        {format!("{:?}", local_placement)}
                    </button>
                }
            />
        </div>

        <h2>RTL</h2>
        <div class="controls">
                <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("rtl-{}", value)
                        style:background-color=move || match rtl() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_rtl(value)
                    >
                        {format!("{}", value)}
                    </button>
                }
            />
        </div>
    }
}
