use convert_case::{Case, Casing};
use floating_ui_leptos::{use_floating, UseFloatingOptions, UseFloatingReturn};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use wasm_bindgen::JsCast;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Node {
    None,
    Reference,
    Floating,
    Body,
    Html,
    OffsetParent,
    ContentBox,
}

const ALL_NODES: [Node; 7] = [
    Node::None,
    Node::Reference,
    Node::Floating,
    Node::Body,
    Node::Html,
    Node::OffsetParent,
    Node::ContentBox,
];

#[component]
pub fn Border() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (node, set_node) = signal(Node::None);

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(reference_ref, floating_ref, UseFloatingOptions::default());

    Effect::new(move |_| {
        let element = match node.get() {
            Node::Html => document()
                .document_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>()),
            Node::Body => document().body(),
            _ => None,
        };

        if let Some(element) = element {
            element
                .style()
                .set_property("border", "10px solid black")
                .expect("Style should be updated.");
        }

        update();
    });

    on_cleanup(move || {
        let element = match node.get() {
            Node::Html => document()
                .document_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>()),
            Node::Body => document().body(),
            _ => None,
        };

        if let Some(element) = element {
            element
                .style()
                .remove_property("border")
                .expect("Style should be updated.");
        }
    });

    view! {
        <h1>Border</h1>
        <p>
            The floating element should be correctly positioned on the bottom when a certain element has a border.
        </p>
        <div
            class="container"
            style:border=move || match node.get() {
                Node::OffsetParent | Node::ContentBox => "10px solid black",
                _ => ""
            }
            style:overflow="hidden"
            style:padding=move || match node.get() {
                Node::ContentBox => "10px",
                _ => ""
            }
            style:position=move || match node.get() {
                Node::OffsetParent | Node::ContentBox => "relative",
                _ => ""
            }
            style:box-sizing=move || match node.get() {
                Node::ContentBox => "unset",
                _ => ""
            }
        >
            <div
                node_ref=reference_ref
                class="reference"
                style:border=move || match node.get() {
                    Node::Reference => "10px solid black",
                    _ => ""
                }
            >
                Reference
            </div>
            <div
                node_ref=floating_ref
                class="floating"
                style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                style:top=move || format!("{}px", y.get())
                style:left=move || format!("{}px", x.get())
                style:border=move || match node.get() {
                    Node::Floating => "10px solid black",
                    _ => ""
                }
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
                        data-testid=move || format!("border-{}", match local_node {
                            Node::None => "null".to_owned(),
                            Node::ContentBox => "content-box".to_owned(),
                            _ => format!("{:?}", local_node).to_case(Case::Camel)
                        })
                        style:background-color=move || if node.get() == local_node {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_node.set(local_node)
                    >
                        {format!("{:?}", local_node).to_case(Case::Camel)}
                    </button>
                }
            />
        </div>
    }
}
