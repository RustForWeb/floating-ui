use convert_case::{Case, Casing};
use floating_ui_leptos::{
    Derivable, DerivableFn, MiddlewareState, MiddlewareVec, Offset, OffsetOptions,
    OffsetOptionsValues, Placement, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::utils::all_placements::ALL_PLACEMENTS;

type Value = SendWrapper<Derivable<'static, web_sys::Element, web_sys::Window, OffsetOptions>>;

fn values() -> Vec<(&'static str, Value)> {
    vec![
        ("0", SendWrapper::new(OffsetOptions::Value(0.0).into())),
        ("10", SendWrapper::new(OffsetOptions::Value(10.0).into())),
        ("-10", SendWrapper::new(OffsetOptions::Value(-10.0).into())),
        (
            "cA: 10",
            SendWrapper::new(
                OffsetOptions::Values(OffsetOptionsValues::default().cross_axis(10.0)).into(),
            ),
        ),
        (
            "mA: 5, cA: -10",
            SendWrapper::new(
                OffsetOptions::Values(
                    OffsetOptionsValues::default()
                        .main_axis(5.0)
                        .cross_axis(-10.0),
                )
                .into(),
            ),
        ),
        (
            "() => -f.height",
            SendWrapper::new(DerivableFn::into(&|MiddlewareState { rects, .. }| {
                OffsetOptions::Value(-rects.floating.height)
            })),
        ),
        (
            "() => cA: -f.width/2",
            SendWrapper::new(DerivableFn::into(&|MiddlewareState { rects, .. }| {
                OffsetOptions::Values(
                    OffsetOptionsValues::default().cross_axis(-rects.floating.width / 2.0),
                )
            })),
        ),
        (
            "aA: 5",
            SendWrapper::new(
                OffsetOptions::Values(OffsetOptionsValues::default().alignment_axis(5.0)).into(),
            ),
        ),
        (
            "aA: -10",
            SendWrapper::new(
                OffsetOptions::Values(OffsetOptionsValues::default().alignment_axis(-10.0)).into(),
            ),
        ),
    ]
}

#[component]
pub fn Offset() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (rtl, set_rtl) = signal(false);
    let (placement, set_placement) = signal(Placement::Bottom);
    let (offset_options, set_offset_options) = signal("0");

    let UseFloatingReturn {
        floating_styles,
        update,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement)
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let options = values()
                    .into_iter()
                    .find_map(|(name, options)| {
                        (name == offset_options.get()).then(|| options.take())
                    })
                    .unwrap();

                let middleware: MiddlewareVec = vec![Box::new(Offset::new_derivable(options))];

                Some(SendWrapper::new(middleware))
            })),
    );

    Effect::new(move || {
        _ = rtl.get();
        update();
    });

    view! {
        <h1>Offset</h1>
        <p></p>
        <div class="container" style:direction=move || if rtl.get() {
            "rtl"
        } else {
            "ltr"
        }>
            <div node_ref=reference_ref class="reference">
                Reference
            </div>
            <div node_ref=floating_ref class="floating" style=floating_styles>
                Floating
            </div>
        </div>

        <h2>alignment</h2>
        <div class="controls">
            <For
                each=values
                key=|(name, _)| name.to_string()
                children=move |(name, _)| view! {
                    <button
                        data-testid=move || format!("offset-{name}")
                        style:background-color=move || if offset_options.get() == name {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_offset_options.set(name)
                    >
                        {name}
                    </button>
                }
            />
        </div>

        <h2>Placement</h2>
        <div class="controls">
            <For
                each=|| ALL_PLACEMENTS
                key=|local_placement| format!("{local_placement:?}")
                children=move |local_placement| view! {
                    <button
                        data-testid=format!("Placement{local_placement:?}").to_case(Case::Kebab)
                        style:background-color=move || if placement.get() == local_placement {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_placement.set(local_placement)
                    >
                        {format!("{local_placement:?}").to_case(Case::Kebab)}
                    </button>
                }
            />
        </div>

        <h2>RTL</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("rtl-{}", value)
                        style:background-color=move || if rtl.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_rtl.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>
    }
}
