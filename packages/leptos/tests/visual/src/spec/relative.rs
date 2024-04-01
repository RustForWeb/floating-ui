use convert_case::{Case, Casing};
use floating_ui_leptos::{use_floating, UseFloatingOptions, UseFloatingReturn};
use leptos::{html::Div, *};
use wasm_bindgen::JsCast;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Node {
    None,
    Html,
    Body,
    OffsetParent,
}

const ALL_NODES: [Node; 4] = [Node::None, Node::Html, Node::Body, Node::OffsetParent];

#[component]
pub fn Relative() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();

    let (node, set_node) = create_signal(Node::None);

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(reference_ref, floating_ref, UseFloatingOptions::default());

    create_effect(move |_| {
        let element = match node() {
            Node::Html => document()
                .document_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>()),
            Node::Body => document().body(),
            _ => None,
        };

        if let Some(element) = element {
            element
                .style()
                .set_property("position", "relative")
                .expect("Style should be updated.");
        }

        update();
    });

    on_cleanup(move || {
        let element = match node() {
            Node::Html => document()
                .document_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>()),
            Node::Body => document().body(),
            _ => None,
        };

        if let Some(element) = element {
            element
                .style()
                .remove_property("position")
                .expect("Style should be updated.");
        }
    });

    view! {
        <h1>Relative</h1>
        <p>
            The floating element should be positioned correctly on the bottom when a
            certain parent node has <code>position: relative</code> applied.
        </p>
        <div
            class="container"
            style:position=move || match node() {
                Node::OffsetParent => "relative",
                _ => ""
            }
        >
            <div _ref=reference_ref class="reference">
                Reference
            </div>
            <div
                _ref=floating_ref
                class="floating"
                style:position=move || format!("{:?}", strategy()).to_lowercase()
                style:top=move || format!("{}px", y())
                style:left=move || format!("{}px", x())
            >
                Floating
            </div>
        </div>

        <div class="controls">
            <For
                each=|| ALL_NODES
                key=|local_node| format!("{:?}", local_node)
                children=move |local_node| view! {
                    <button
                        data-testid=move || format!("relative-{}", match local_node {
                            Node::None => "null".into(),
                            _ => format!("{:?}", local_node).to_case(Case::Camel)
                        })
                        style:background-color=move || match node() == local_node {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_node(local_node)
                    >
                        {format!("{:?}", local_node).to_case(Case::Camel)}
                    </button>
                }
            />
        </div>
    }
}
