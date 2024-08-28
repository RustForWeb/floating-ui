use convert_case::{Case, Casing};
use floating_ui_yew::{use_floating, UseFloatingOptions, UseFloatingReturn};
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Node {
    None,
    Html,
    Body,
    OffsetParent,
}

const ALL_NODES: [Node; 4] = [Node::None, Node::Html, Node::Body, Node::OffsetParent];

#[function_component]
pub fn Relative() -> Html {
    let reference_ref = use_node_ref();
    let floating_ref = use_node_ref();

    let node = use_state_eq(|| Node::None);

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_ref.clone().into(),
        floating_ref.clone(),
        UseFloatingOptions::default(),
    );

    use_effect_with(node.clone(), move |node| {
        let element = match **node {
            Node::Html => window()
                .expect("Window should exist.")
                .document()
                .expect("Document should exist.")
                .document_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>()),
            Node::Body => window()
                .expect("Window should exist.")
                .document()
                .expect("Document should exist.")
                .body(),
            _ => None,
        };

        if let Some(element) = element.as_ref() {
            element
                .style()
                .set_property("position", "relative")
                .expect("Style should be updated.");
        }

        update.emit(());

        move || {
            if let Some(element) = element {
                element
                    .style()
                    .remove_property("position")
                    .expect("Style should be updated.");
            }
        }
    });

    html! {
        <>
            <h1>{"Relative"}</h1>
            <p>
                {"The floating element should be positioned correctly on the bottom when a
                certain parent node has "}
                <code>{"position: relative"}</code>
                {" applied."}
            </p>
            <div
                class="container"
                style={match *node {
                    Node::OffsetParent => Some("position: relative;"),
                    _ => None
                }}
            >
                <div ref={reference_ref} class="reference">
                    {"Reference"}
                </div>
                <div
                    ref={floating_ref}
                    class="floating"
                    style={format!(
                        "position: {}; top: {}px; left: {}px;",
                        format!("{:?}", *strategy).to_lowercase(),
                        *y,
                        *x
                    )}
                >
                    {"Floating"}
                </div>
            </div>

            <div class="controls">
                {
                    ALL_NODES.into_iter().map(|value| {
                        html! {
                            <button
                                key={format!("{:?}", value)}
                                data-testid={format!("relative-{}", match value {
                                    Node::None => "null".into(),
                                    _ => format!("{:?}", value).to_case(Case::Camel)
                                })}
                                style={match *node == value {
                                    true => "background-color: black;",
                                    false => ""
                                }}
                                onclick={Callback::from({
                                    let node = node.clone();

                                    move |_| node.set(value)
                                })}
                            >
                                {format!("{:?}", value).to_case(Case::Camel)}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}
