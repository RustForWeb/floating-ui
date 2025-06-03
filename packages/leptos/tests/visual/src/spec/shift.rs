use convert_case::{Case, Casing};
use floating_ui_leptos::{
    Derivable, DerivableFn, LimitShift, LimitShiftOffset, LimitShiftOffsetValues,
    LimitShiftOptions, MiddlewareState, MiddlewareVec, Offset, OffsetOptions, Placement, Shift,
    ShiftOptions, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_scroll::{UseScrollOptions, UseScrollReturn, use_scroll},
};

type Value = SendWrapper<Derivable<'static, web_sys::Element, web_sys::Window, LimitShiftOffset>>;

fn values() -> Vec<(&'static str, Value)> {
    vec![
        ("0", SendWrapper::new(LimitShiftOffset::Value(0.0).into())),
        ("50", SendWrapper::new(LimitShiftOffset::Value(50.0).into())),
        (
            "-50",
            SendWrapper::new(LimitShiftOffset::Value(-50.0).into()),
        ),
        (
            "mA: 50",
            SendWrapper::new(
                LimitShiftOffset::Values(LimitShiftOffsetValues::default().main_axis(50.0)).into(),
            ),
        ),
        (
            "cA: 50",
            SendWrapper::new(
                LimitShiftOffset::Values(LimitShiftOffsetValues::default().cross_axis(50.0)).into(),
            ),
        ),
        (
            "fn => r.width/2",
            SendWrapper::new(DerivableFn::into(&|MiddlewareState { rects, .. }| {
                LimitShiftOffset::Value(rects.reference.width)
            })),
        ),
        (
            "fn => cA: f.width/2",
            SendWrapper::new(DerivableFn::into(&|MiddlewareState { rects, .. }| {
                LimitShiftOffset::Values(
                    // According to the name this should be `rects.floating / 2.0`, but the React unit test uses `rects.reference` instead.
                    LimitShiftOffsetValues::default().cross_axis(rects.reference.width),
                )
            })),
        ),
    ]
}

#[component]
pub fn Shift() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (placement, set_placement) = signal(Placement::Bottom);
    let (main_axis, set_main_axis) = signal(true);
    let (cross_axis, set_cross_axis) = signal(false);
    let (limit_shift, set_limit_shift) = signal(false);
    let (limit_shift_main_axis, set_limit_shift_main_axis) = signal(true);
    let (limit_shift_cross_axis, set_limit_shift_cross_axis) = signal(true);
    let (limit_shift_offset, set_limit_shift_offset) = signal("0");
    let (offset_value, set_offset_value) = signal(0);

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement)
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let limit_shift_offset = values()
                    .into_iter()
                    .find_map(|(name, options)| {
                        (name == limit_shift_offset.get()).then(|| options.take())
                    })
                    .unwrap();

                let mut shift_options = ShiftOptions::default()
                    .main_axis(main_axis.get())
                    .cross_axis(cross_axis.get());

                if limit_shift.get() {
                    shift_options = shift_options.limiter(Box::new(LimitShift::new(
                        LimitShiftOptions::default()
                            .main_axis(limit_shift_main_axis.get())
                            .cross_axis(limit_shift_cross_axis.get())
                            .offset_derivable(limit_shift_offset),
                    )))
                }

                let middleware: MiddlewareVec = vec![
                    Box::new(Offset::new(OffsetOptions::Value(offset_value.get() as f64))),
                    Box::new(Shift::new(shift_options)),
                ];
                Some(SendWrapper::new(middleware))
            })),
    );

    let UseScrollReturn { scroll_ref, .. } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: None::<bool>.into(),
        disable_ref_updates: None,
    });

    view! {
        <h1>Shift</h1>
        <p></p>
        <div class="container">
            <div node_ref=scroll_ref class="scroll" data-x="" style:position="relative">
                <div node_ref=reference_ref class="reference">
                    Reference
                </div>
                <div
                    node_ref=floating_ref
                    class="floating"
                    style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                    style:top=move || format!("{}px", y.get())
                    style:left=move || format!("{}px", x.get())
                >
                    Floating
                </div>
            </div>
        </div>

        <h2>placement</h2>
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

        <h2>offset</h2>
        <div class="controls">
            <For
                each=|| [0, 10]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("offset-{}", value)
                        style:background-color=move || if offset_value.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_offset_value.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>

        <h2>mainAxis</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("mainAxis-{}", value)
                        style:background-color=move || if main_axis.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_main_axis.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>

        <h2>crossAxis</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("crossAxis-{}", value)
                        style:background-color=move || if cross_axis.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_cross_axis.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>

        <h2>limitShift</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("limitShift-{}", value)
                        style:background-color=move || if limit_shift.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_limit_shift.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>

        <Show when=move || limit_shift.get()>
            <h2>limitShift.mainAxis</h2>
            <div class="controls">
                <For
                    each=|| [true, false]
                    key=|value| format!("{value}")
                    children=move |value| view! {
                        <button
                            data-testid=format!("limitShift.mainAxis-{}", value)
                            style:background-color=move || if limit_shift_main_axis.get() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_limit_shift_main_axis.set(value)
                        >
                            {format!("{value}")}
                        </button>
                    }
                />
            </div>

            <h2>limitShift.crossAxis</h2>
            <div class="controls">
                <For
                    each=|| [true, false]
                    key=|value| format!("{value}")
                    children=move |value| view! {
                        <button
                            data-testid=format!("limitShift.crossAxis-{}", value)
                            style:background-color=move || if limit_shift_cross_axis.get() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_limit_shift_cross_axis.set(value)
                        >
                            {format!("{value}")}
                        </button>
                    }
                />
            </div>

            <h2>limitShift.offset</h2>
            <div class="controls">
                <For
                    each=values
                    key=|(name, _)| name.to_string()
                    children=move |(name, _)| view! {
                        <button
                            data-testid=move || format!("limitShift.offset-{name}")
                            style:background-color=move || if limit_shift_offset.get() == name {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_limit_shift_offset.set(name)
                        >
                            {name}
                        </button>
                    }
                />
            </div>
        </Show>
    }
}
