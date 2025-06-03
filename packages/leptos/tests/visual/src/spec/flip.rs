use convert_case::{Case, Casing};
use floating_ui_leptos::{
    Alignment, CrossAxis, FallbackStrategy, Flip, FlipOptions, MiddlewareVec, Placement, Shift,
    ShiftOptions, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_scroll::{UseScrollOptions, UseScrollReturn, use_scroll},
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum FallbackPlacements {
    None,
    Empty,
    All,
}

#[component]
pub fn Flip() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (placement, set_placement) = signal(Placement::Bottom);
    let (main_axis, set_main_axis) = signal(true);
    let (cross_axis, set_cross_axis) = signal(CrossAxis::True);
    let (fallback_placements, set_fallback_placements) = signal(FallbackPlacements::None);
    let (fallback_strategy, set_fallback_strategy) = signal(FallbackStrategy::BestFit);
    let (flip_alignment, set_flip_alignment) = signal(true);
    let (add_shift, set_add_shift) = signal(false);
    let (fallback_axis_side_direction, set_fallback_axis_side_direction) =
        signal(None::<Alignment>);

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
                let mut options = FlipOptions::default()
                    .main_axis(main_axis.get())
                    .cross_axis(cross_axis.get())
                    .fallback_strategy(fallback_strategy.get())
                    .flip_alignment(flip_alignment.get())
                    .fallback_axis_side_direction(Alignment::End);

                options = if add_shift.get() && fallback_axis_side_direction.get().is_none() {
                    options.fallback_placements(vec![Placement::Bottom])
                } else {
                    match fallback_placements.get() {
                        FallbackPlacements::None => options,
                        FallbackPlacements::Empty => options.fallback_placements(vec![]),
                        FallbackPlacements::All => {
                            options.fallback_placements(ALL_PLACEMENTS.into())
                        }
                    }
                };

                let mut middleware: MiddlewareVec = vec![Box::new(Flip::new(options))];

                if add_shift.get() {
                    middleware.push(Box::new(Shift::new(ShiftOptions::default())));
                }

                Some(SendWrapper::new(middleware))
            })),
    );

    let UseScrollReturn {
        scroll_ref,
        indicator,
        ..
    } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: None::<bool>.into(),
        disable_ref_updates: None,
    });

    view! {
        <h1>Flip</h1>
        <p></p>
        <div class="container">
            <div node_ref=scroll_ref class="scroll" data-x="" style:position="relative">
                {move || indicator()}
                <div node_ref=reference_ref class="reference">
                    Reference
                </div>
                <div
                    node_ref=floating_ref
                    class="floating"
                    style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                    style:top=move || format!("{}px", y.get())
                    style:left=move || format!("{}px", x.get())
                    style:width=move || if add_shift.get() { if fallback_axis_side_direction.get().is_none() { "400px" } else { "200px" } } else { Default::default() }
                    style:height=move || if add_shift.get() { if fallback_axis_side_direction.get().is_none() { Default::default() } else { "50px" } } else { Default::default() }
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
                each=|| [CrossAxis::True, CrossAxis::False, CrossAxis::Alignment]
                key=|value| format!("{value:?}")
                children=move |value| view! {
                    <button
                        data-testid=format!("crossAxis-{}", format!("{value:?}").to_case(Case::Camel))
                        style:background-color=move || if cross_axis.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_cross_axis.set(value)
                    >
                        {format!("{value:?}").to_case(Case::Camel)}
                    </button>
                }
            />
        </div>

        <h2>fallbackPlacements</h2>
        <div class="controls">
            <For
                each=|| [FallbackPlacements::None, FallbackPlacements::Empty, FallbackPlacements::All]
                key=|value| format!("{value:?}")
                children=move |value| view! {
                    <button
                        data-testid=format!("fallbackPlacements-{}", match value {
                            FallbackPlacements::None => "undefined",
                            FallbackPlacements::Empty => "[]",
                            FallbackPlacements::All => "all",
                        })
                        style:background-color=move || if fallback_placements.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_fallback_placements.set(value)
                    >
                        {match value {
                            FallbackPlacements::None => "undefined".to_owned(),
                            FallbackPlacements::Empty => "[]".to_owned(),
                            FallbackPlacements::All => format!("[{}]", ALL_PLACEMENTS.map(|p| format!("{p:?}").to_case(Case::Kebab)).join(", ")),
                        }}
                    </button>
                }
            />
        </div>

        <h2>fallbackStrategy</h2>
        <div class="controls">
            <For
                each=|| [FallbackStrategy::BestFit, FallbackStrategy::InitialPlacement]
                key=|local_fallback_strategy| format!("{local_fallback_strategy:?}")
                children=move |local_fallback_strategy| view! {
                    <button
                        data-testid=format!("fallbackStrategy-{}", format!("{local_fallback_strategy:?}").to_case(Case::Camel))
                        style:background-color=move || if fallback_strategy.get() == local_fallback_strategy {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_fallback_strategy.set(local_fallback_strategy)
                    >
                        {format!("{local_fallback_strategy:?}").to_case(Case::Camel)}
                    </button>
                }
            />
        </div>

        <h2>flipAlignment</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("flipAlignment-{}", value)
                        style:background-color=move || if flip_alignment.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_flip_alignment.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>

        <h2>Add shift</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("shift-{}", value)
                        style:background-color=move || if add_shift.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_add_shift.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>

        <h2>fallbackAxisSideDirection</h2>
        <div class="controls">
            <For
                each=|| [Some(Alignment::Start), Some(Alignment::End), None]
                key=|value| format!("{value:?}")
                children=move |value| view! {
                    <button
                        data-testid=format!("fallbackAxisSideDirection-{}", match value {
                            Some(Alignment::Start) => "start",
                            Some(Alignment::End) => "end",
                            None => "none"
                        })
                        style:background-color=move || if fallback_axis_side_direction.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_fallback_axis_side_direction.set(value)
                    >
                        {match value {
                            Some(Alignment::Start) => "start",
                            Some(Alignment::End) => "end",
                            None => "none"
                        }}
                    </button>
                }
            />
        </div>
    }
}
