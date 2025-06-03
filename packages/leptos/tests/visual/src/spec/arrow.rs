use convert_case::{Case, Casing};
use floating_ui_leptos::{
    ARROW_NAME, Arrow, ArrowData, ArrowOptions, DetectOverflowOptions, MiddlewareVec, Offset,
    OffsetOptions, Padding, Placement, Shift, ShiftOptions, Side, UseFloatingOptions,
    UseFloatingReturn, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_scroll::{UseScrollOptions, UseScrollReturn, use_scroll},
};

#[component]
pub fn Arrow() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();
    let arrow_ref = AnyNodeRef::new();

    let (placement, set_placement) = signal(Placement::Bottom);
    let (padding, set_padding) = signal(0);
    let (reference_size, set_reference_size) = signal(125);
    let (floating_size, set_floating_size) = signal(75);
    let (svg, set_svg) = signal(false);
    let (center_offset, set_center_offset) = signal(false);
    let (add_offset, set_add_offset) = signal(false);
    let (nested, set_nested) = signal(false);

    let UseFloatingReturn {
        update,
        placement: resultant_placement,
        middleware_data,
        floating_styles,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement)
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let mut middleware: MiddlewareVec = if add_offset.get() {
                    vec![Box::new(Offset::new(OffsetOptions::Value(20.0)))]
                } else {
                    vec![]
                };

                middleware.append(&mut vec![
                    Box::new(Shift::new(ShiftOptions::default().detect_overflow(
                        DetectOverflowOptions::default().padding(Padding::All(10.0)),
                    ))),
                    Box::new(Arrow::new(
                        ArrowOptions::new(arrow_ref).padding(Padding::All(padding.get() as f64)),
                    )),
                ]);

                Some(SendWrapper::new(middleware))
            })),
    );

    let static_side = move || resultant_placement.get().side().opposite();

    let arrow_data = move || {
        let arrow_data: Option<ArrowData> = middleware_data.get().get_as(ARROW_NAME);
        arrow_data
    };
    let arrow_x = move || arrow_data().and_then(|arrow_data| arrow_data.x);
    let arrow_y = move || arrow_data().and_then(|arrow_data| arrow_data.y);
    let center_offset_value = move || arrow_data().map(|arrow_data| arrow_data.center_offset);

    let UseScrollReturn {
        scroll_ref,
        update_scroll,
        ..
    } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update: update.clone(),
        rtl: None::<bool>.into(),
        disable_ref_updates: None,
    });

    let floating_view = move || {
        let base = view! {
            {move || if center_offset.get() {
                center_offset_value().map_or("".to_owned(), |center_offset_value| center_offset_value.to_string())
            } else {
                "Floating".to_owned()
            }}

            {move || if svg.get() {
                view!{
                    <svg
                        node_ref=arrow_ref
                        class="arrow"
                        style:position="absolute"
                        style:top=move || match static_side() {
                            Side::Top => "-15px".to_owned(),
                            _ => match arrow_y() {
                                Some(arrow_y) => format!("{arrow_y}px"),
                                None => "".to_owned()
                            }
                        }
                        style:right=move || match static_side() {
                            Side::Right => "-15px",
                            _ => ""
                        }
                        style:bottom=move || match static_side() {
                            Side::Bottom => "-15px",
                            _ => ""
                        }
                        style:left=move || match static_side() {
                            Side::Left => "-15px".to_owned(),
                            _ => match arrow_x() {
                                Some(arrow_x) => format!("{arrow_x}px"),
                                None => "".to_owned()
                            }
                        }
                    />
                }.into_any()
            } else {
                view!{
                    <div
                        node_ref=arrow_ref
                        class="arrow"
                        style:position="absolute"
                        style:top=move || match static_side() {
                            Side::Top => "-15px".to_owned(),
                            _ => match arrow_y() {
                                Some(arrow_y) => format!("{arrow_y}px"),
                                None => "".to_owned()
                            }
                        }
                        style:right=move || match static_side() {
                            Side::Right => "-15px",
                            _ => ""
                        }
                        style:bottom=move || match static_side() {
                            Side::Bottom => "-15px",
                            _ => ""
                        }
                        style:left=move || match static_side() {
                            Side::Left => "-15px".to_owned(),
                            _ => match arrow_x() {
                                Some(arrow_x) => format!("{arrow_x}px"),
                                None => "".to_owned()
                            }
                        }
                    />
                }.into_any()
            }}
        };

        if nested.get() {
            view! {
                <div
                    node_ref=floating_ref
                    style:position=move || floating_styles.get().style_position()
                    style:top=move || floating_styles.get().style_top()
                    style:left=move || floating_styles.get().style_left()
                    style:transform=move || floating_styles.get().style_transform().unwrap_or_default()
                    style:will-change=move || floating_styles.get().style_will_change().unwrap_or_default()
                    style:width=move || format!("{}px", floating_size.get())
                    style:height=move || format!("{}px", floating_size.get())
                >
                    <div class="floating" style:position="relative" style:border="5px solid black">
                        {base}
                    </div>
                </div>
            }
            .into_any()
        } else {
            view! {
                <div
                    node_ref=floating_ref
                    class="floating"
                    style:position=move || floating_styles.get().style_position()
                    style:top=move || floating_styles.get().style_top()
                    style:left=move || floating_styles.get().style_left()
                    style:transform=move || floating_styles.get().style_transform().unwrap_or_default()
                    style:will-change=move || floating_styles.get().style_will_change().unwrap_or_default()
                    style:width=move || format!("{}px", floating_size.get())
                    style:height=move || format!("{}px", floating_size.get())
                >
                    {base}
                </div>
            }
            .into_any()
        }
    };

    view! {
        <h1>Arrow</h1>
        <p></p>
        <div class="container" style:will-change=move || if svg.get() {
            "transform"
        } else {
            ""
        }>
            <div node_ref=scroll_ref class="scroll" data-x="" style:position="relative">
                <div
                    node_ref=reference_ref
                    class="reference"
                    style:width=move || format!("{}px", reference_size.get())
                    style:height=move || format!("{}px", reference_size.get())
                >
                    Reference
                </div>
                {floating_view}
            </div>
        </div>

        <h2>Reference size</h2>
        <div class="controls">
            <For
                each=|| [25, 125]
                key=|size| format!("{size:?}")
                children=move |size| view! {
                    <button
                        data-testid=format!("reference-{size}")
                        style:background-color=move || if reference_size.get() == size {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_reference_size.set(size)
                    >
                        {format!("{size}")}
                    </button>
                }
            />
        </div>

        <h2>Floating size</h2>
        <div class="controls">
            <For
                each=|| [75, 150]
                key=|size| format!("{size:?}")
                children=move |size| view! {
                    <button
                        data-testid=format!("floating-{size}")
                        style:background-color=move || if floating_size.get() == size {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_floating_size.set(size)
                    >
                        {format!("{size}")}
                    </button>
                }
            />
        </div>

        <h2>Arrow padding</h2>
        <div class="controls">
            <For
                each=|| [0, 20, 200]
                key=|size| format!("{size:?}")
                children={
                    let update_scroll = update_scroll.clone();

                    move |size| {
                        view! {
                            <button
                                data-testid=format!("arrow-padding-{size}")
                                style:background-color=move || if padding.get() == size {
                                    "black"
                                } else {
                                    ""
                                }
                                on:click={
                                    let update_scroll = update_scroll.clone();

                                    move |_| {
                                        set_padding.set(size);

                                        // Match React test behaviour
                                        update_scroll();
                                    }
                                }
                            >
                                {format!("{size}")}
                            </button>
                        }
                    }
                }
            />
        </div>

        <h2>Add offset</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children={
                    let update_scroll = update_scroll.clone();

                    move |value| {
                        view! {
                            <button
                                data-testid=format!("add-offset-{}", value)
                                style:background-color=move || if add_offset.get() == value {
                                    "black"
                                } else {
                                    ""
                                }
                                on:click={
                                    let update_scroll = update_scroll.clone();

                                    move |_| {
                                        set_add_offset.set(value);

                                        // Match React test behaviour
                                        update_scroll();
                                    }
                                }
                            >
                                {format!("{value}")}
                            </button>
                        }
                    }
                }
            />
        </div>

        <h2>Placement</h2>
        <div class="controls">
            <For
                each=|| ALL_PLACEMENTS
                key=|local_placement| format!("{local_placement:?}")
                children={
                    let update_scroll = update_scroll.clone();

                    move |local_placement| {
                        view! {
                            <button
                                data-testid=format!("Placement{local_placement:?}").to_case(Case::Kebab)
                                style:background-color=move || if placement.get() == local_placement {
                                    "black"
                                } else {
                                    ""
                                }
                                on:click={
                                    let update_scroll = update_scroll.clone();

                                    move |_| {
                                        set_placement.set(local_placement);

                                        // Match React test behaviour
                                        update_scroll();
                                    }
                                }
                            >
                                {format!("{local_placement:?}").to_case(Case::Kebab)}
                            </button>
                        }
                    }
                }
            />
        </div>

        <h2>SVG</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("svg-{}", value)
                        style:background-color=move || if svg.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_svg.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>

        <h2>Nested</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("nested-{}", value)
                        style:background-color=move || if nested.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_nested.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>

        <h2>Center offset</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children={
                    let update_scroll = update_scroll.clone();

                    move |value| {
                        view! {
                            <button
                                data-testid=format!("centerOffset-{}", value)
                                style:background-color=move || if center_offset.get() == value {
                                    "black"
                                } else {
                                    ""
                                }
                                on:click={
                                    let update_scroll = update_scroll.clone();

                                    move |_| {
                                        set_center_offset.set(value);
                                        if value {
                                            set_reference_size.set(25);
                                            set_floating_size.set(125);
                                            set_placement.set(Placement::LeftEnd);
                                            set_padding.set(25);
                                        } else {
                                            set_reference_size.set(125);
                                            set_floating_size.set(75);
                                            set_placement.set(Placement::Bottom);
                                            set_padding.set(0);
                                        }

                                        // Match React test behaviour
                                        update_scroll();
                                    }
                                }
                            >
                                {format!("{value}")}
                            </button>
                        }
                    }
                }
            />
        </div>
    }
}
