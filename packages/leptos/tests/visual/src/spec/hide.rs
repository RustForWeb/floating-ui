use convert_case::{Case, Casing};
use floating_ui_leptos::{
    use_floating, ApplyState, Hide, HideData, HideOptions, HideStrategy, MiddlewareState,
    MiddlewareVec, Placement, Shift, ShiftOptions, Size, SizeOptions, Strategy, UseFloatingOptions,
    UseFloatingReturn, HIDE_NAME,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_scroll::{use_scroll, UseScrollOptions, UseScrollReturn},
};

#[component]
pub fn Hide() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (placement, set_placement) = signal(Placement::Bottom);
    let (hierarchy, set_hierarchy) = signal('a');
    let is_fixed_strategy = move || ['j', 'k', 'l', 'm', 'o', 'p', 'q'].contains(&hierarchy.get());

    let UseFloatingReturn {
        x,
        y,
        strategy,
        middleware_data,
        update,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement.into())
            .strategy(MaybeProp::derive(move || {
                Some(if is_fixed_strategy() {
                    Strategy::Fixed
                } else {
                    Strategy::Absolute
                })
            }))
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let mut middleware: MiddlewareVec = vec![
                    Box::new(Hide::new(
                        HideOptions::default().strategy(HideStrategy::ReferenceHidden),
                    )),
                    Box::new(Hide::new(
                        HideOptions::default().strategy(HideStrategy::Escaped),
                    )),
                ];

                if hierarchy.get() == 'o' {
                    middleware.push(Box::new(Shift::new(ShiftOptions::default())));
                }

                middleware.push(Box::new(Size::new(SizeOptions::default().apply(
                    if is_fixed_strategy() {
                        &|ApplyState {
                              state,
                              available_height,
                              ..
                          }| {
                            let MiddlewareState { elements, .. } = state;

                            let floating = (*elements.floating)
                                .clone()
                                .unchecked_into::<web_sys::HtmlElement>();

                            floating
                                .style()
                                .set_property("max-height", &format!("{}px", available_height))
                                .expect("Style should be updated.");
                        }
                    } else {
                        &|ApplyState { state, .. }| {
                            let MiddlewareState { elements, .. } = state;

                            let floating = (*elements.floating)
                                .clone()
                                .unchecked_into::<web_sys::HtmlElement>();

                            floating
                                .style()
                                .remove_property("max-height")
                                .expect("Style should be updated.");
                        }
                    },
                ))));

                Some(SendWrapper::new(middleware))
            })),
    );

    let hide_data = Signal::derive(move || middleware_data.get().get_as::<HideData>(HIDE_NAME));
    let reference_hidden = Signal::derive(move || {
        hide_data
            .get()
            .and_then(|data| data.reference_hidden)
            .unwrap_or(false)
    });
    let escaped = Signal::derive(move || {
        hide_data
            .get()
            .and_then(|data| data.escaped)
            .unwrap_or(false)
    });

    let UseScrollReturn {
        scroll_ref,
        indicator,
        update_scroll,
    } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: None::<bool>.into(),
        disable_ref_updates: Some(true),
    });

    Effect::new(move || {
        // Match React test behaviour
        if ['j', 'm', 'k', 'l'].contains(&hierarchy.get()) {
            update_scroll();
        }
    });

    let reference_view = move || {
        let base = view! {
            <div node_ref=reference_ref class="reference">
                Reference
            </div>
        };

        match hierarchy.get() {
            'b' => view! {
                <div style:overflow="hidden" style:height="0px">
                    <div style:position="absolute" style:top="0px" style:left="0px">
                        {base}
                    </div>
                </div>
            }.into_any(),
            'c' => view! {
                <div style:overflow="scroll" style:height="0px">
                    <div style:overflow="hidden">
                        <div style:position="absolute" style:top="0px" style:left="0px">
                            {base}
                        </div>
                    </div>
                </div>
            }.into_any(),
            'd' => view! {
                <div style:overflow="hidden" style:height="0px">
                    <div node_ref=reference_ref class="reference" style:position="absolute" style:top="0px" style:left="0px">
                        Reference
                    </div>
                </div>
            }.into_any(),
            'e' => view! {
                <div style:overflow="scroll" style:height="0px" style:position="relative">
                    <div style:overflow="hidden">
                        <div style:position="absolute">
                            {base}
                        </div>
                    </div>
                </div>
            }.into_any(),
            'f' => view! {
                <div style:overflow="scroll" style:width="20px" style:height="20px" style:position="relative">
                    <div style:overflow="hidden">
                        <div style:position="absolute">
                            {base}
                        </div>
                    </div>
                </div>
            }.into_any(),
            'g' => view! {
                <div style:overflow="scroll" style:height="0px">
                    <div style:overflow="hidden">
                        <div style:position="absolute" style:top="0px" style:left="0px">
                            <div style:position="absolute">
                                {base}
                            </div>
                        </div>
                    </div>
                </div>
            }.into_any(),
            'h' => view! {
                <div style:overflow="scroll" style:height="0px">
                    <div style:overflow="hidden">
                        <div style:position="absolute" style:top="0px" style:left="0px" style:overflow="hidden">
                            <div style:position="absolute">
                                {base}
                            </div>
                        </div>
                    </div>
                </div>
            }.into_any(),
            'i' => view! {
                <div style:position="relative">
                    <div style:overflow="hidden">
                        <div style:position="absolute" style:overflow="hidden" style:height="200px" style:width="200px" style:border="1px solid blue">
                            <div style:position="absolute" style:left="20px" style:top="20px">
                                {base}
                            </div>
                        </div>
                    </div>
                </div>
            }.into_any(),
            'n' => view! {
                <div style:position="fixed" style:top="150px" style:left="225px" style:overflow="hidden">
                    {base}
                </div>
            }.into_any(),
            'p' => view! {
                <div style:overflow="hidden" style:height="0px">
                    <div style:position="relative">
                        <div style:position="fixed" style:top="100px" style:left="300px">
                            {base}
                        </div>
                    </div>
                </div>
            }.into_any(),
            'q' => view! {
                <div style:position="fixed" style:overflow="hidden" style:height="0px">
                    <div style:position="fixed" style:top="100px" style:left="300px">
                        {base}
                    </div>
                </div>
            }.into_any(),
            _ => base.into_any(),
        }
    };

    let floating_view = move || {
        let base = view! {
            <div
                node_ref=floating_ref
                class="floating"
                style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                style:top=move || format!("{}px", y.get())
                style:left=move || format!("{}px", x.get())
                style:background-color=move || if reference_hidden.get() {
                    "black"
                } else if escaped.get() {
                    "yellow"
                } else {
                    ""
                }
            >
                Floating
            </div>
        };

        match hierarchy.get() {
            'j' => view! {
                <div style:overflow="hidden" style:position="relative" style:width="80px" style:height="40px">
                    {base}
                </div>
            }.into_any(),
            'k' => view! {
                <div style:overflow="hidden" style:position="relative" style:width="80px" style:height="40px" style:transform="translateZ(0)">
                    {base}
                </div>
            }.into_any(),
            'l' => view! {
                <div style:overflow="hidden" style:position="relative" style:width="80px" style:height="40px">
                    <div style:transform="translateZ(0)">
                        {base}
                    </div>
                </div>
            }.into_any(),
            'm' => view! {
               <div style:overflow="hidden" style:position="relative" style:width="80px" style:height="40px">
                    <div
                        node_ref=floating_ref
                        class="floating"
                        style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                        style:top=move || format!("{}px", y.get())
                        style:left=move || format!("{}px", x.get())
                        style:transform="translateZ(0)"
                    >
                        Floating
                    </div>
               </div>
            }.into_any(),
            'o' => view! {
                <div
                    style:width="50px"
                    style:height="50px"
                    style:overflow="auto"
                    style:position="absolute"
                    style:top="50px"
                    style:left="50px"
                    style:background="blue"
                    style:display="inline-block"
                >
                    <div style:position="fixed">
                        <div style:transform="translateZ(0)">
                            {base}
                        </div>
                    </div>
                </div>
            }.into_any(),
            _ => base.into_any(),
        }
    };

    view! {
        <h1>Hide</h1>
        <p></p>
        <div class="container" style:position="relative">
            <div node_ref=scroll_ref class="scroll" data-x="">
                {move || indicator()}
                {reference_view}
                {floating_view}
            </div>
        </div>

        <div class="controls">
            <For
                each=|| ALL_PLACEMENTS
                key=|local_placement| format!("{:?}", local_placement)
                children=move |local_placement| view! {
                    <button
                        data-testid=format!("Placement{:?}", local_placement).to_case(Case::Kebab)
                        style:background-color=move || if placement.get() == local_placement {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_placement.set(local_placement)
                    >
                        {format!("{:?}", local_placement).to_case(Case::Kebab)}
                    </button>
                }
            />
        </div>

        <h2>Hierarchy</h2>
        <div class="controls">
            <For
                each=|| ['a', 'b', 'c', 'd', 'e', 'f', 'g','h','i','j','k','l','m','n','o','p','q']
                key=|local_hierarchy| format!("{:?}", local_hierarchy)
                children=move |local_hierarchy| view! {
                    <button
                        data-testid=format!("hierarchy-{}", local_hierarchy)
                        style:background-color=move || if hierarchy.get() == local_hierarchy {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_hierarchy.set(local_hierarchy)
                    >
                        {local_hierarchy}
                    </button>
                }
            />
        </div>
    }
}
