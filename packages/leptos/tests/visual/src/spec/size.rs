use convert_case::{Case, Casing};
use floating_ui_leptos::{
    use_floating, ApplyState, DetectOverflowOptions, Flip, FlipOptions, IntoReference,
    MiddlewareState, MiddlewareVec, Placement, Shift, ShiftOptions, Size, SizeOptions,
    UseFloatingOptions, UseFloatingReturn,
};
use leptos::{html::Div, *};
use wasm_bindgen::JsCast;

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_scroll::{use_scroll, UseScrollOptions, UseScrollReturn},
};

#[component]
pub fn Size() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();

    let (rtl, set_rtl) = create_signal(false);
    let (placement, set_placement) = create_signal(Placement::Bottom);
    let (add_flip_shift, set_add_flip_shift) = create_signal(false);

    let has_edge_alignment = move || placement().alignment().is_some();

    let detect_overflow_options =
        DetectOverflowOptions::default().padding(floating_ui_leptos::Padding::All(10.0));

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement.into())
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let mut middleware: MiddlewareVec = vec![];

                if add_flip_shift() {
                    middleware.push(Box::new(Flip::new(
                        FlipOptions::default().detect_overflow(detect_overflow_options.clone()),
                    )));

                    if !has_edge_alignment() {
                        middleware.push(Box::new(Shift::new(
                            ShiftOptions::default()
                                .detect_overflow(detect_overflow_options.clone()),
                        )));
                    }
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
                                .set_property("max-width", &format!("{}px", available_width))
                                .expect("Style should be updated.");
                            floating
                                .style()
                                .set_property("max-height", &format!("{}px", available_height))
                                .expect("Style should be updated.");
                        })
                        .detect_overflow(detect_overflow_options.clone()),
                )));

                if add_flip_shift() && has_edge_alignment() {
                    middleware.push(Box::new(Shift::new(
                        ShiftOptions::default().detect_overflow(detect_overflow_options.clone()),
                    )));
                }

                Some(middleware)
            })),
    );

    let UseScrollReturn { scroll_ref, .. } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: rtl.into(),
        disable_ref_updates: None,
    });

    view! {
        <h1>Size</h1>
        <p></p>
        <div class="container" style:direction=move || match rtl() {
            true => "rtl",
            false => "ltr",
        }>
            <div _ref=scroll_ref class="scroll" data-x="" style:position="relative">
                <div _ref=reference_ref class="reference">
                    Reference
                </div>
                <div
                    _ref=floating_ref
                    class="floating"
                    style:position=move || format!("{:?}", strategy()).to_lowercase()
                    style:top=move || format!("{}px", y())
                    style:left=move || format!("{}px", x())
                    style:width=move || match add_flip_shift() {
                        true => "600px",
                        false => "400px",
                    }
                    style:height=move || match add_flip_shift() {
                        true => "600px",
                        false => "300px",
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
                key=|local_placement| format!("{:?}", local_placement)
                children=move |local_placement| view! {
                    <button
                        data-testid=format!("Placement{:?}", local_placement).to_case(Case::Kebab)
                        style:background-color=move || match placement() == local_placement {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_placement(local_placement)
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
                    view! {
                        <button
                            data-testid=format!("rtl-{}", value)
                            style:background-color=move || match rtl() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_rtl(value);
                            }
                        >
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>

        <h2>Add flip and shift</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("flipshift-{}", value)
                        style:background-color=move || match add_flip_shift() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_add_flip_shift(value)
                    >
                        {format!("{}", value)}
                    </button>
                }
            />
        </div>
    }
}
