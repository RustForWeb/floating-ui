use convert_case::{Case, Casing};
use floating_ui_yew::{
    use_auto_update, use_floating, Placement as PlacementEnum, UseFloatingOptions,
    UseFloatingReturn,
};
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::utils::{all_placements::ALL_PLACEMENTS, use_size::use_size};

#[function_component]
pub fn Placement() -> Html {
    let reference_ref = use_node_ref();
    let floating_ref = use_node_ref();

    let rtl = use_state_eq(|| false);
    let placement = use_state_eq(|| PlacementEnum::Bottom);

    let auto_update = use_auto_update();

    let UseFloatingReturn {
        floating_styles,
        update,
        ..
    } = use_floating(
        reference_ref.clone().into(),
        floating_ref.clone(),
        UseFloatingOptions::default()
            .placement(*placement)
            .while_elements_mounted((*auto_update).clone()),
    );

    let size = use_size(None, None);

    html! {
        <>
            <h1>{"Placement"}</h1>
            <p>
                {"The floating element should be correctly positioned when given each of the 12 placements."}
            </p>
            <div class="container" style={format!("direction: {}", match *rtl {
                true => "rtl",
                false => "ltr"
            })}>
                <div ref={reference_ref} class="reference">
                    {"Reference"}
                </div>
                <div
                    ref={floating_ref}
                    class="floating"
                    style={format!("{} width: {}px; height: {}px;", floating_styles, *size, *size)}
                >
                    {"Floating"}
                </div>
            </div>

            <div class="controls">
                <label for="size">{"Size"}</label>
                <input
                    id="size"
                    type="range"
                    min="1"
                    max="200"
                    value={size.to_string()}
                    oninput={Callback::from(move |event: InputEvent| {
                        size.set(
                            event
                                .target()
                                .unwrap()
                                .unchecked_into::<web_sys::HtmlInputElement>()
                                .value()
                                .parse()
                                .unwrap(),
                        );
                    })}
                />
            </div>

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

                                    move |_| placement.set(value)
                                })}
                            >
                                {format!("{:?}", value).to_case(Case::Kebab)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <h2>{"RTL"}</h2>
            <div class="controls">
                {
                    [true, false].into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{}", value)}
                                data-testid={format!("rtl-{}", value)}
                                style={match *rtl == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let rtl = rtl.clone();
                                    let update = update.clone();

                                    move |_| {
                                        rtl.set(value);
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
        </>
    }
}
