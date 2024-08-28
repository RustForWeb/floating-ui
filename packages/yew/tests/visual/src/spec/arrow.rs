use convert_case::{Case, Casing};
use floating_ui_yew::{
    use_auto_update, use_floating, Arrow as ArrowMiddleware, ArrowData, ArrowOptions,
    DetectOverflowOptions, MiddlewareVec, Offset, OffsetOptions, Padding, Placement, Shift,
    ShiftOptions, Side, UseFloatingOptions, UseFloatingReturn, ARROW_NAME,
};
use yew::prelude::*;

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_scroll::{use_scroll, UseScrollOptions, UseScrollReturn},
};

#[function_component]
pub fn Arrow() -> Html {
    let reference_ref = use_node_ref();
    let floating_ref = use_node_ref();
    let arrow_ref = use_node_ref();

    let placement = use_state_eq(|| Placement::Bottom);
    let padding = use_state_eq(|| 0);
    let reference_size = use_state_eq(|| 125);
    let floating_size = use_state_eq(|| 75);
    let svg = use_state_eq(|| false);
    let center_offset = use_state_eq(|| false);
    let add_offset = use_state_eq(|| false);
    let nested = use_state_eq(|| false);

    let auto_update = use_auto_update();
    let middleware = use_memo(
        (add_offset.clone(), padding.clone()),
        |(add_offset, padding)| {
            let mut middleware: MiddlewareVec = match **add_offset {
                true => vec![Box::new(Offset::new(OffsetOptions::Value(20.0)))],
                false => vec![],
            };

            middleware.append(&mut vec![
                Box::new(Shift::new(ShiftOptions::default().detect_overflow(
                    DetectOverflowOptions::default().padding(Padding::All(10.0)),
                ))),
                Box::new(ArrowMiddleware::new(
                    ArrowOptions::new(arrow_ref.clone()).padding(Padding::All(**padding as f64)),
                )),
            ]);

            middleware
        },
    );

    let UseFloatingReturn {
        update,
        placement: resultant_placement,
        middleware_data,
        floating_styles,
        ..
    } = use_floating(
        reference_ref.clone().into(),
        floating_ref.clone(),
        UseFloatingOptions::default()
            .placement(*placement)
            .while_elements_mounted((*auto_update).clone())
            .middleware((*middleware).clone()),
    );

    let static_side = resultant_placement.side().opposite();

    let arrow_data = use_memo(middleware_data, |middleware_data| {
        let arrow_data: Option<ArrowData> = middleware_data.get_as(ARROW_NAME);
        arrow_data
    });
    let arrow_x = use_memo(arrow_data.clone(), |arrow_data| {
        arrow_data
            .as_ref()
            .clone()
            .and_then(|arrow_data| arrow_data.x)
    });
    let arrow_y = use_memo(arrow_data.clone(), |arrow_data| {
        arrow_data
            .as_ref()
            .clone()
            .and_then(|arrow_data| arrow_data.y)
    });
    let center_offset_value = use_memo(arrow_data.clone(), |arrow_data| {
        arrow_data
            .as_ref()
            .clone()
            .map(|arrow_data| arrow_data.center_offset)
    });

    let UseScrollReturn {
        scroll_ref,
        update_scroll,
        ..
    } = use_scroll(UseScrollOptions {
        reference_ref: reference_ref.clone(),
        floating_ref: floating_ref.clone(),
        update: update.clone(),
        rtl: None,
    });

    let arrow_tag = match *svg {
        true => "svg",
        false => "div",
    };
    let arrow_element = html! {
        <>
            {match *center_offset {
                true => center_offset_value.map_or("".into(), |center_offset_value| center_offset_value.to_string()),
                false => "Floating".into(),
            }}

            <@{arrow_tag}
                ref={arrow_ref}
                class="arrow"
                style={format!(
                    "position: absolute; top: {}; right: {}; bottom: {}; left: {};",
                    match static_side {
                        Side::Top => "-15px".into(),
                        _ => match *arrow_y {
                            Some(arrow_y) => format!("{}px", arrow_y),
                            None => "".into()
                        }
                    },
                    match static_side {
                        Side::Right => "-15px",
                        _ => ""
                    },
                    match static_side {
                        Side::Bottom => "-15px",
                        _ => ""
                    },
                    match static_side {
                        Side::Left => "-15px".into(),
                        _ => match *arrow_x {
                            Some(arrow_x) => format!("{}px", arrow_x),
                            None => "".into()
                        }
                    }
                )}
            />
        </>
    };

    let floating_element = match *nested {
        true => html! {
            <div
                ref={floating_ref.clone()}
                style={format!("{} width: {}px; height: {}px;", floating_styles, *floating_size, *floating_size)}
            >
                <div class="floating" style="position: relative; border: 5px solid black;">
                    {arrow_element}
                </div>
            </div>
        },
        false => html! {
            <div
                ref={floating_ref.clone()}
                class="floating"
                style={format!("{} width: {}px; height: {}px;", floating_styles, *floating_size, *floating_size)}
            >
                {arrow_element}
            </div>
        },
    };

    html! {
        <>
            <h1>{"Arrow"}</h1>
            <p></p>
            <div class="container" style={match *svg {
                true => "will-change: transform;",
                false => "",
            }}>
                <div ref={scroll_ref} class="scroll" data-x="" style="position: relative;">
                    <div
                        ref={reference_ref}
                        class="reference"
                        style={format!("width: {}px; height: {}px;", *reference_size, *reference_size)}
                    >
                        {"Reference"}
                    </div>
                    {floating_element}
                </div>
            </div>

            <h2>{"Reference size"}</h2>
            <div class="controls">
                {
                    [25, 125].into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{:?}", value)}
                                data-testid={format!("reference-{value}")}
                                style={match *reference_size == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let reference_size = reference_size.clone();

                                    move |_| reference_size.set(value)
                                })}
                            >
                                {format!("{}", value)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <h2>{"Floating size"}</h2>
            <div class="controls">
                {
                    [75, 150].into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{:?}", value)}
                                data-testid={format!("floating-{value}")}
                                style={match *floating_size == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let floating_size = floating_size.clone();

                                    move |_| floating_size.set(value)
                                })}
                            >
                                {format!("{}", value)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <h2>{"Arrow padding"}</h2>
            <div class="controls">
                {
                    [0, 20, 200].into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{:?}", value)}
                                data-testid={format!("arrow-padding-{value}")}
                                style={match *padding == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let padding = padding.clone();
                                    let update_scroll = update_scroll.clone();

                                    move |_| {
                                        padding.set(value);

                                        // Match React test behaviour
                                        update_scroll.emit(());
                                    }
                                })}
                            >
                                {format!("{}", value)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <h2>{"Add offset"}</h2>
            <div class="controls">
                {
                    [true, false].into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{}", value)}
                                data-testid={format!("add-offset-{}", value)}
                                style={match *add_offset == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let add_offset = add_offset.clone();
                                    let update_scroll = update_scroll.clone();

                                    move |_| {
                                        add_offset.set(value);

                                        // Match React test behaviour
                                        update_scroll.emit(());
                                    }
                                })}
                            >
                                {format!("{}", value)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <h2>{"Placement"}</h2>
            <div class="controls">
                {
                    ALL_PLACEMENTS.into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{:?}", value)}
                                data-testid={format!("Placement{:?}", value).to_case(Case::Kebab)}
                                style={match *placement == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let placement = placement.clone();
                                    let update_scroll = update_scroll.clone();

                                    move |_| {
                                        placement.set(value);

                                        // Match React test behaviour
                                        update_scroll.emit(());
                                    }
                                })}
                            >
                                {format!("{:?}", value).to_case(Case::Kebab)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <h2>{"SVG"}</h2>
            <div class="controls">
                {
                    [true, false].into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{}", value)}
                                data-testid={format!("svg-{}", value)}
                                style={match *svg == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let svg = svg.clone();
                                    let update = update.clone();

                                    move |_| {
                                        svg.set(value);
                                        update.emit(());
                                    }
                                })}
                            >
                                {format!("{}", value)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <h2>{"Nested"}</h2>
            <div class="controls">
                {
                    [true, false].into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{}", value)}
                                data-testid={format!("nested-{}", value)}
                                style={match *nested == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let nested = nested.clone();
                                    let update = update.clone();

                                    move |_| {
                                        nested.set(value);
                                        update.emit(());
                                    }
                                })}
                            >
                                {format!("{}", value)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <h2>{"Center offset"}</h2>
            <div class="controls">
                {
                    [true, false].into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{}", value)}
                                data-testid={format!("centerOffset-{}", value)}
                                style={match *center_offset == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let center_offset = center_offset.clone();
                                    let reference_size = reference_size.clone();
                                    let floating_size = floating_size.clone();
                                    let placement = placement.clone();
                                    let padding = padding.clone();
                                    let update_scroll = update_scroll.clone();

                                    move |_| {
                                        center_offset.set(value);
                                        if value {
                                            reference_size.set(25);
                                            floating_size.set(125);
                                            placement.set(Placement::LeftEnd);
                                            padding.set(25);
                                        } else {
                                            reference_size.set(125);
                                            floating_size.set(75);
                                            placement.set(Placement::Bottom);
                                            padding.set(0);
                                        }

                                        // Match React test behaviour
                                        update_scroll.emit(());
                                    }
                                })}
                            >
                                {format!("{}", value)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}
