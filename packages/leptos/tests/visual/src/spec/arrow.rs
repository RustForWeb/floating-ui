use convert_case::{Case, Casing};
use floating_ui_leptos::{
    use_floating, Arrow, ArrowData, ArrowOptions, DetectOverflowOptions, MiddlewareVec, Offset,
    OffsetOptions, Padding, Placement, Shift, ShiftOptions, Side, UseFloatingOptions,
    UseFloatingReturn, ARROW_NAME,
};
use leptos::{html::Div, *};

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_scroll::{use_scroll, UseScrollOptions, UseScrollReturn},
};

#[component]
pub fn Arrow() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();
    let arrow_div_ref = create_node_ref::<Div>();
    // let arrow_svg_ref = create_node_ref::<Svg>();

    let (placement, set_placement) = create_signal(Placement::Bottom);
    let (padding, set_padding) = create_signal(0);
    let (reference_size, set_reference_size) = create_signal(125);
    let (floating_size, set_floating_size) = create_signal(75);
    let (svg, set_svg) = create_signal(false);
    let (center_offset, set_center_offset) = create_signal(false);
    let (add_offset, set_add_offset) = create_signal(false);

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        placement: resultant_placement,
        middleware_data,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement.into())
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let mut middleware: MiddlewareVec = match add_offset() {
                    true => vec![Box::new(Offset::new(OffsetOptions::Value(20.0)))],
                    false => vec![],
                };

                middleware.append(&mut vec![
                    Box::new(Shift::new(ShiftOptions::default().detect_overflow(
                        DetectOverflowOptions::default().padding(Padding::All(10.0)),
                    ))),
                    Box::new(Arrow::new(
                        ArrowOptions::new(arrow_div_ref).padding(Padding::All(padding() as f64)),
                    )),
                ]);

                Some(middleware)
            })),
    );

    let static_side = move || resultant_placement().side().opposite();

    let arrow_data = move || {
        let arrow_data: Option<ArrowData> = middleware_data().get_as(ARROW_NAME);
        arrow_data
    };
    let arrow_x = move || arrow_data().and_then(|arrow_data| arrow_data.x);
    let arrow_y = move || arrow_data().and_then(|arrow_data| arrow_data.y);
    let center_offset_value = move || arrow_data().map(|arrow_data| arrow_data.center_offset);

    let UseScrollReturn { scroll_ref, .. } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: None,
    });

    view! {
        <h1>Arrow</h1>
        <p></p>
        <div class="container" style=move || match svg() {
            true => "will-change: transform;",
            false => "",
        }>
            <div _ref=scroll_ref class="scroll" data-x="" style:position="relative">
                <div
                    _ref=reference_ref
                    class="reference"
                    style:width=move || format!("{}px", reference_size())
                    style:height=move || format!("{}px", reference_size())
                >
                    Reference
                </div>
                <div
                    _ref=floating_ref
                    class="floating"
                    style:position=move || format!("{:?}", strategy()).to_lowercase()
                    style:top=move || format!("{}px", y())
                    style:left=move || format!("{}px", x())
                    style:width=move || format!("{}px", floating_size())
                    style:height=move || format!("{}px", floating_size())
                >
                    {move || match center_offset() {
                        true => center_offset_value().map_or("".into(), |center_offset_value| center_offset_value.to_string()),
                        false => "Floating".into()
                    }}

                    <div
                        _ref=arrow_div_ref
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

                    // TODO: replace this with Show
                    // match svg() {
                    //     // TODO: copy attributes to SVG
                    //     true => view!{
                    //         <svg
                    //             _ref=arrow_svg_ref
                    //             class="arrow"
                    //         />
                    //     }.into_any(),
                    //     false => view! {

                    //     }.into_any()
                    // }
                </div>
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
                        style:background-color=move || match reference_size() == size {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_reference_size(size)
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
                        style:background-color=move || match floating_size() == size {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_floating_size(size)
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
                children=move |size| view! {
                    <button
                        data-testid=format!("arrow-padding-{size}")
                        style:background-color=move || match padding() == size {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_padding(size)
                    >
                        {format!("{size}")}
                    </button>
                }
            />
        </div>

        <h2>Add offset</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| {
                    view! {
                        <button
                            data-testid=format!("add-offset-{}", value)
                            style:background-color=move || match add_offset() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| set_add_offset(value)
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

        <h2>SVG</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| {
                    view! {
                        <button
                            data-testid=format!("svg-{}", value)
                            style:background-color=move || match svg() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| set_svg(value)
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
                    view! {
                        <button
                            data-testid=format!("centerOffset-{}", value)
                            style:background-color=move || match center_offset() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_center_offset(value);
                                if value {
                                    set_reference_size(25);
                                    set_floating_size(125);
                                    set_placement(Placement::LeftEnd);
                                    set_padding(25);
                                } else {
                                    set_reference_size(125);
                                    set_floating_size(75);
                                    set_placement(Placement::Bottom);
                                    set_padding(0);
                                }
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
