use convert_case::{Case, Casing};
use floating_ui_leptos::{
    Alignment, AutoPlacement, AutoPlacementOptions, MiddlewareVec, Placement, Shift, ShiftOptions,
    UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::utils::use_scroll::{UseScrollOptions, UseScrollReturn, use_scroll};

const ALL_ALIGNMENTS: [Option<Alignment>; 3] = [None, Some(Alignment::Start), Some(Alignment::End)];

#[derive(Copy, Clone, Debug, PartialEq)]
enum AllowedPlacements {
    None,
    TopCommaBottom,
    LeftCommaRight,
    TopStartCommaTopEndCommaBottomStartCommaBottomEnd,
}

impl From<AllowedPlacements> for Option<Vec<Placement>> {
    fn from(value: AllowedPlacements) -> Self {
        match value {
            AllowedPlacements::None => None,
            AllowedPlacements::TopCommaBottom => Some(vec![Placement::Top, Placement::Bottom]),
            AllowedPlacements::LeftCommaRight => Some(vec![Placement::Left, Placement::Right]),
            AllowedPlacements::TopStartCommaTopEndCommaBottomStartCommaBottomEnd => Some(vec![
                Placement::TopStart,
                Placement::TopEnd,
                Placement::BottomStart,
                Placement::BottomEnd,
            ]),
        }
    }
}

const ALL_ALLOWED_PLACEMENTS: [AllowedPlacements; 4] = [
    AllowedPlacements::None,
    AllowedPlacements::TopCommaBottom,
    AllowedPlacements::LeftCommaRight,
    AllowedPlacements::TopStartCommaTopEndCommaBottomStartCommaBottomEnd,
];

#[component]
pub fn AutoPlacement() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (alignment, set_alignment) = signal::<Option<Alignment>>(Some(Alignment::Start));
    let (auto_alignment, set_auto_alignment) = signal(true);
    let (allowed_placements, set_allowed_placements) =
        signal::<AllowedPlacements>(AllowedPlacements::None);
    let (cross_axis, set_cross_axis) = signal(false);
    let (add_shift, set_add_shift) = signal(false);

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
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let mut middleware: MiddlewareVec =
                    vec![Box::new(AutoPlacement::new(AutoPlacementOptions {
                        detect_overflow: None,
                        cross_axis: Some(cross_axis.get()),
                        alignment: alignment.get(),
                        auto_alignment: Some(auto_alignment.get()),
                        allowed_placements: allowed_placements.get().into(),
                    }))];

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
        <h1>AutoPlacement</h1>
        <p></p>
        <div class="container">
            <div
                node_ref=scroll_ref
                class="scroll"
                data-x=""
                style:position="relative"
            >
                {move || indicator()}
                <div
                    node_ref=reference_ref
                    class="reference"
                    style=move || if add_shift.get() { "width: 50px; height: 25px;" } else { Default::default() }
                >
                    Reference
                </div>
                <div
                    node_ref=floating_ref
                    class="floating"
                    style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                    style:top=move || format!("{}px", y.get())
                    style:left=move || format!("{}px", x.get())
                    style:width=move || if add_shift.get() { "250px" } else { Default::default() }
                    style:height=move || if add_shift.get() { "250px" } else { Default::default() }
                >
                    Floating
                </div>
            </div>
        </div>

        <h2>alignment</h2>
        <div class="controls">
            <For
                each=|| ALL_ALIGNMENTS
                key=|local_alignment| format!("{local_alignment:?}")
                children=move |local_alignment| view! {
                    <button
                        data-testid=move || format!("alignment-{}", match local_alignment {
                            None => "null".to_owned(),
                            Some(local_alignment) => format!("{local_alignment:?}").to_case(Case::Camel)
                        })
                        style:background-color=move || if alignment.get() == local_alignment {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_alignment.set(local_alignment)
                    >
                        {match local_alignment {
                            None => "null".to_owned(),
                            Some(local_alignment) => format!("{local_alignment:?}").to_case(Case::Camel)
                        }}
                    </button>
                }
            />
        </div>

        <h2>autoAlignment</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| {
                    view! {
                        <button
                            data-testid=format!("autoAlignment-{}", value)
                            style:background-color=move || if auto_alignment.get() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_auto_alignment.set(value)
                        >
                            {format!("{value}")}
                        </button>
                    }
                }
            />
        </div>

        <h2>allowedPlacements</h2>
        <div class="controls">
            <For
                each=|| ALL_ALLOWED_PLACEMENTS
                key=|local_allowed_placements| format!("{local_allowed_placements:?}")
                children=move |local_allowed_placements| {
                    view! {
                        <button
                            data-testid=move || format!("allowedPlacements-{}", match local_allowed_placements {
                                AllowedPlacements::None => "undefined".to_owned(),
                                _ => format!("{local_allowed_placements:?}").replace("Comma", ",").to_case(Case::Kebab)
                            })
                            style:background-color=move || if allowed_placements.get() == local_allowed_placements {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_allowed_placements.set(local_allowed_placements)
                        >
                            {match local_allowed_placements {
                                AllowedPlacements::None => "undefined".to_owned(),
                                _ => format!("{local_allowed_placements:?}").replace("Comma", ",").to_case(Case::Kebab)
                            }}
                        </button>
                    }
                }
            />
        </div>

        <h2>crossAxis</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| {
                    view! {
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
                }
            />
        </div>

        <h2>Add shift</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| {
                    view! {
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
                }
            />
        </div>
    }
}
