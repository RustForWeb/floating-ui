use convert_case::{Case, Casing};
use floating_ui_leptos::{
    use_floating, Arrow, ArrowData, ArrowOptions, DetectOverflowOptions, IntoReference,
    MiddlewareVec, Offset, OffsetOptions, Padding, Placement, Shift, ShiftOptions, Side,
    UseFloatingOptions, UseFloatingReturn, ARROW_NAME,
};
use leptos::{
    html::{AnyElement, Div},
    *,
};

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_scroll::{use_scroll, UseScrollOptions, UseScrollReturn},
};

#[component]
pub fn Arrow() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();
    let arrow_ref = create_node_ref::<AnyElement>();

    let (placement, set_placement) = create_signal(Placement::Bottom);
    let (padding, set_padding) = create_signal(0);
    let (reference_size, set_reference_size) = create_signal(125);
    let (floating_size, set_floating_size) = create_signal(75);
    let (svg, set_svg) = create_signal(false);
    let (center_offset, set_center_offset) = create_signal(false);
    let (add_offset, set_add_offset) = create_signal(false);
    let (nested, set_nested) = create_signal(false);

    let UseFloatingReturn {
        update,
        placement: resultant_placement,
        middleware_data,
        floating_styles,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement.into())
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let mut middleware: MiddlewareVec = match add_offset.get() {
                    true => vec![Box::new(Offset::new(OffsetOptions::Value(20.0)))],
                    false => vec![],
                };

                middleware.append(&mut vec![
                    Box::new(Shift::new(ShiftOptions::default().detect_overflow(
                        DetectOverflowOptions::default().padding(Padding::All(10.0)),
                    ))),
                    Box::new(Arrow::new(
                        ArrowOptions::new(arrow_ref).padding(Padding::All(padding.get() as f64)),
                    )),
                ]);

                Some(middleware)
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

    let svg_update = update.clone();
    let nested_update = update.clone();

    let UseScrollReturn {
        scroll_ref,
        update_scroll,
        ..
    } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: None::<bool>.into(),
        disable_ref_updates: None,
    });

    let placement_update_scroll = update_scroll.clone();
    let padding_update_scroll = update_scroll.clone();
    let add_offset_update_scroll = update_scroll.clone();
    let center_offset_update_scroll = update_scroll.clone();

    let floating_view = move || {
        let base = view! {
            {move || match center_offset.get() {
                true => center_offset_value().map_or("".into(), |center_offset_value| center_offset_value.to_string()),
                false => "Floating".into()
            }}

            {move || match svg.get() {
                true => view!{
                    <svg
                        class="arrow"
                        style:position="absolute"
                        style:top=move || match static_side() {
                            Side::Top => "-15px".into(),
                            _ => match arrow_y() {
                                Some(arrow_y) => format!("{}px", arrow_y),
                                None => "".into()
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
                            Side::Left => "-15px".into(),
                            _ => match arrow_x() {
                                Some(arrow_x) => format!("{}px", arrow_x),
                                None => "".into()
                            }
                        }
                    />
                }.into_any().node_ref(arrow_ref),
                false => view!{
                    <div
                        class="arrow"
                        style:position="absolute"
                        style:top=move || match static_side() {
                            Side::Top => "-15px".into(),
                            _ => match arrow_y() {
                                Some(arrow_y) => format!("{}px", arrow_y),
                                None => "".into()
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
                            Side::Left => "-15px".into(),
                            _ => match arrow_x() {
                                Some(arrow_x) => format!("{}px", arrow_x),
                                None => "".into()
                            }
                        }
                    />
                }.into_any().node_ref(arrow_ref)
            }}
        };

        match nested.get() {
            true => view! {
                <div
                    _ref=floating_ref
                    style:position=move || floating_styles.get().style_position()
                    style:top=move || floating_styles.get().style_top()
                    style:left=move || floating_styles.get().style_left()
                    style:transform=move || floating_styles.get().style_transform()
                    style:will-change=move || floating_styles.get().style_will_change()
                    style:width=move || format!("{}px", floating_size.get())
                    style:height=move || format!("{}px", floating_size.get())
                >
                    <div class="floating" style:position="relative" style:border="5px solid black">
                        {base}
                    </div>
                </div>
            },
            false => view! {
                <div
                    _ref=floating_ref
                    class="floating"
                    style:position=move || floating_styles.get().style_position()
                    style:top=move || floating_styles.get().style_top()
                    style:left=move || floating_styles.get().style_left()
                    style:transform=move || floating_styles.get().style_transform()
                    style:will-change=move || floating_styles.get().style_will_change()
                    style:width=move || format!("{}px", floating_size.get())
                    style:height=move || format!("{}px", floating_size.get())
                >
                    {base}
                </div>
            },
        }
    };

    view! {
        <h1>Arrow</h1>
        <p></p>
        <div class="container" style:will-change=move || match svg.get() {
            true => "transform",
            false => "",
        }>
            <div _ref=scroll_ref class="scroll" data-x="" style:position="relative">
                <div
                    _ref=reference_ref
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
                key=|size| format!("{:?}", size)
                children=move |size| view! {
                    <button
                        data-testid=format!("reference-{size}")
                        style:background-color=move || match reference_size.get() == size {
                            true => "black",
                            false => ""
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
                key=|size| format!("{:?}", size)
                children=move |size| view! {
                    <button
                        data-testid=format!("floating-{size}")
                        style:background-color=move || match floating_size.get() == size {
                            true => "black",
                            false => ""
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
                key=|size| format!("{:?}", size)
                children=move |size| {
                    let padding_update_scroll = padding_update_scroll.clone();
                    view! {
                        <button
                            data-testid=format!("arrow-padding-{size}")
                            style:background-color=move || match padding.get() == size {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_padding.set(size);
                                // Match React test behaviour
                                padding_update_scroll();
                            }
                        >
                            {format!("{size}")}
                        </button>
                    }
                }
            />
        </div>

        <h2>Add offset</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| {
                    let add_offset_update_scroll = add_offset_update_scroll.clone();
                    view! {
                        <button
                            data-testid=format!("add-offset-{}", value)
                            style:background-color=move || match add_offset.get() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_add_offset.set(value);
                                // Match React test behaviour
                                add_offset_update_scroll();
                            }
                        >
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>

        <h2>Placement</h2>
        <div class="controls">
            <For
                each=|| ALL_PLACEMENTS
                key=|local_placement| format!("{:?}", local_placement)
                children=move |local_placement| {
                    let placement_update_scroll = placement_update_scroll.clone();
                    view! {
                        <button
                            data-testid=format!("Placement{:?}", local_placement).to_case(Case::Kebab)
                            style:background-color=move || match placement.get() == local_placement {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_placement.set(local_placement);
                                // Match React test behaviour
                                placement_update_scroll();
                            }
                        >
                            {format!("{:?}", local_placement).to_case(Case::Kebab)}
                        </button>
                    }
                }
            />
        </div>

        <h2>SVG</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| {
                    let svg_update = svg_update.clone();
                    view! {
                        <button
                            data-testid=format!("svg-{}", value)
                            style:background-color=move || match svg.get() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_svg.set(value);
                                svg_update();
                            }
                        >
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>

        <h2>Nested</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| {
                    let nested_update = nested_update.clone();
                    view! {
                        <button
                            data-testid=format!("nested-{}", value)
                            style:background-color=move || match nested.get() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_nested.set(value);
                                nested_update();
                            }
                        >
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>

        <h2>Center offset</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| {
                    let center_offset_update_scroll = center_offset_update_scroll.clone();
                    view! {
                        <button
                            data-testid=format!("centerOffset-{}", value)
                            style:background-color=move || match center_offset.get() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
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
                                center_offset_update_scroll();
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
