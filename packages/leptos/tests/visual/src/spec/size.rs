use convert_case::{Case, Casing};
use floating_ui_leptos::{
    ApplyState, DetectOverflowOptions, Flip, FlipOptions, LimitShift, LimitShiftOffset,
    LimitShiftOptions, MiddlewareState, MiddlewareVec, Padding, Placement, Shift, ShiftOptions,
    Size, SizeOptions, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_resize::use_resize,
    use_scroll::{UseScrollOptions, UseScrollReturn, use_scroll},
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum ShiftOrder {
    None,
    Before,
    After,
}

#[component]
pub fn Size() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (rtl, set_rtl) = signal(false);
    let (placement, set_placement) = signal(Placement::Bottom);
    let (add_flip, set_add_flip) = signal(false);
    let (add_shift, set_add_shift) = signal(ShiftOrder::None);
    let (shift_cross_axis, set_shift_cross_axis) = signal(false);
    let (shift_limiter, set_shift_limiter) = signal(false);

    let has_edge_alignment = move || placement.get().alignment().is_some();

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
                let mut middleware: MiddlewareVec = vec![];

                let detect_overflow_options =
                    DetectOverflowOptions::default().padding(Padding::All(10.0));

                let mut shift_options = ShiftOptions::default()
                    .detect_overflow(detect_overflow_options.clone())
                    .cross_axis(shift_cross_axis.get());
                if shift_limiter.get() {
                    shift_options = shift_options.limiter(Box::new(LimitShift::new(
                        LimitShiftOptions::default().offset(LimitShiftOffset::Value(50.0)),
                    )));
                }

                if add_flip.get() {
                    middleware.push(Box::new(Flip::new(
                        FlipOptions::default().detect_overflow(detect_overflow_options.clone()),
                    )));
                }

                if add_shift.get() == ShiftOrder::Before {
                    middleware.push(Box::new(Shift::new(shift_options.clone())));
                }

                middleware.push(Box::new(Size::new(
                    SizeOptions::default()
                        .apply(&|ApplyState {
                                     state,
                                     available_width,
                                     available_height,
                                 }: ApplyState<
                            web_sys::Element,
                            web_sys::Window,
                        >| {
                            let MiddlewareState { elements, .. } = state;

                            let floating = (*elements.floating)
                                .clone()
                                .unchecked_into::<web_sys::HtmlElement>();

                            floating
                                .style()
                                .set_property("max-width", &format!("{available_width}px"))
                                .expect("Style should be updated.");
                            floating
                                .style()
                                .set_property("max-height", &format!("{available_height}px"))
                                .expect("Style should be updated.");
                        })
                        .detect_overflow(detect_overflow_options.clone()),
                )));

                if add_shift.get() == ShiftOrder::After {
                    middleware.push(Box::new(Shift::new(shift_options.clone())));
                }

                Some(SendWrapper::new(middleware))
            })),
    );

    let UseScrollReturn { scroll_ref, .. } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update: update.clone(),
        rtl: rtl.into(),
        disable_ref_updates: None,
    });

    use_resize(scroll_ref, update);

    view! {
        <h1>Size</h1>
        <p></p>
        <div class="container" style:direction=move || if rtl.get() {
            "rtl"
        } else {
            "ltr"
        }>
            <div node_ref=scroll_ref class="scroll resize" data-x="" style:position="relative">
                <div node_ref=reference_ref class="reference">
                    Reference
                </div>
                <div
                    node_ref=floating_ref
                    class="floating"
                    style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                    style:top=move || format!("{}px", y.get())
                    style:left=move || format!("{}px", x.get())
                    style:width=move || if add_shift.get() != ShiftOrder::None {
                        if add_shift.get() == ShiftOrder::Before && shift_cross_axis.get() {
                            "100px"
                        } else if add_shift.get() == ShiftOrder::Before && has_edge_alignment() {
                            "360px"
                        } else {
                            "600px"
                        }
                    } else {
                        "400px"
                    }
                    style:height=move || if add_shift.get() != ShiftOrder::None {
                        "600px"
                    } else {
                        "300px"
                    }
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

        <h2>RTL</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| {
                    view! {
                        <button
                            data-testid=format!("rtl-{}", value)
                            style:background-color=move || if rtl.get() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| {
                                set_rtl.set(value);
                            }
                        >
                            {format!("{value}")}
                        </button>
                    }
                }
            />
        </div>

        <h2>Add flip</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("flip-{}", value)
                        style:background-color=move || if add_flip.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_add_flip.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>

        <h2>Add shift</h2>
        <div class="controls">
            <For
                each=|| [ShiftOrder::None, ShiftOrder::Before, ShiftOrder::After]
                key=|value| format!("{value:?}")
                children=move |value| view! {
                    <button
                        data-testid=format!("shift-{}", format!("{value:?}").to_case(Case::Camel))
                        style:background-color=move || if add_shift.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_add_shift.set(value)
                    >
                        {format!("{value:?}").to_case(Case::Camel)}
                    </button>
                }
            />
        </div>

        <Show when=move || add_shift.get() != ShiftOrder::None>
            <h3>shift.crossAxis</h3>
            <div class="controls">
                <For
                    each=|| [true, false]
                    key=|value| format!("{value}")
                    children=move |value| view! {
                        <button
                            data-testid=format!("shift.crossAxis-{}", value)
                            style:background-color=move || if shift_cross_axis.get() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_shift_cross_axis.set(value)
                        >
                            {format!("{value}")}
                        </button>
                    }
                />
            </div>

            <h3>shift.limiter</h3>
            <div class="controls">
                <For
                    each=|| [true, false]
                    key=|value| format!("{value}")
                    children=move |value| view! {
                        <button
                            data-testid=format!("shift.limiter-{}", value)
                            style:background-color=move || if shift_limiter.get() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_shift_limiter.set(value)
                        >
                            {format!("{value}")}
                        </button>
                    }
                />
            </div>
        </Show>
    }
}
