use convert_case::{Case, Casing};
use floating_ui_leptos::{
    use_floating, Derivable, DerivableFn, IntoReference, MiddlewareState, MiddlewareVec, Offset,
    OffsetOptions, OffsetOptionsValues, Placement, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{html::Div, *};

use crate::utils::all_placements::ALL_PLACEMENTS;

fn values() -> Vec<(
    &'static str,
    Derivable<'static, web_sys::Element, web_sys::Window, OffsetOptions>,
)> {
    vec![
        ("0", OffsetOptions::Value(0.0).into()),
        ("10", OffsetOptions::Value(10.0).into()),
        ("-10", OffsetOptions::Value(-10.0).into()),
        (
            "cA: 10",
            OffsetOptions::Values(OffsetOptionsValues::default().cross_axis(10.0)).into(),
        ),
        (
            "mA: 5, cA: -10",
            OffsetOptions::Values(
                OffsetOptionsValues::default()
                    .main_axis(5.0)
                    .cross_axis(-10.0),
            )
            .into(),
        ),
        (
            "() => -f.height",
            DerivableFn::into(&|MiddlewareState { rects, .. }| {
                OffsetOptions::Value(-rects.floating.height)
            }),
        ),
        (
            "() => cA: -f.width/2",
            DerivableFn::into(&|MiddlewareState { rects, .. }| {
                OffsetOptions::Values(
                    OffsetOptionsValues::default().cross_axis(-rects.floating.width / 2.0),
                )
            }),
        ),
        (
            "aA: 5",
            OffsetOptions::Values(OffsetOptionsValues::default().alignment_axis(5.0)).into(),
        ),
        (
            "aA: -10",
            OffsetOptions::Values(OffsetOptionsValues::default().alignment_axis(-10.0)).into(),
        ),
    ]
}

#[component]
pub fn Offset() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();

    let (rtl, set_rtl) = create_signal(false);
    let (placement, set_placement) = create_signal(Placement::Bottom);
    let (offset_options, set_offset_options) = create_signal("0");

    let UseFloatingReturn {
        floating_styles,
        update,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement.into())
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let options = values()
                    .into_iter()
                    .find_map(|(name, options)| match name == offset_options.get() {
                        true => Some(options),
                        false => None,
                    })
                    .unwrap();

                let middleware: MiddlewareVec = vec![Box::new(Offset::new_derivable(options))];
                Some(middleware)
            })),
    );

    view! {
        <h1>Offset</h1>
        <p></p>
        <div class="container" style:direction=move || match rtl.get() {
            true => "rtl",
            false => "ltr",
        }>
            <div _ref=reference_ref class="reference">
                Reference
            </div>
            <div _ref=floating_ref class="floating" style=floating_styles>
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
                        data-testid=move || format!("offset-{}", name)
                        style:background-color=move || match offset_options.get() == name {
                            true => "black",
                            false => ""
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
