use convert_case::{Case, Casing};
use floating_ui_leptos::{use_floating, IntoReference, UseFloatingOptions, UseFloatingReturn};
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
    let (offset, set_offset) = create_signal(0);

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default(),
    );

    create_effect(move |_| {
        let element = match node.get() {
            Node::Html => document()
                .document_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>()),
            Node::Body => document().body(),
            _ => document()
                .query_selector(".container")
                .expect("Document should be queried.")
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>()),
        };

        if let Some(element) = element {
            element
                .style()
                .set_property("position", "relative")
                .expect("Style should be updated.");
            element
                .style()
                .set_property("top", &format!("{}px", -offset.get()))
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
                .remove_property("position")
                .expect("Style should be updated.");
            element
                .style()
                .remove_property("top")
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
            style:position=move || match node.get() {
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
                style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                style:top=move || format!("{}px", y.get())
                style:left=move || format!("{}px", x.get())
            >
                Floating
            </div>
        </div>

        <h2>Node</h2>
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
                        style:background-color=move || match node.get() == local_node {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_node.set(local_node)
                    >
                        {format!("{:?}", local_node).to_case(Case::Camel)}
                    </button>
                }
            />
        </div>

        <h2>Offset</h2>
        <div class="controls">
            <For
                each=|| [0, 100]
                key=|local_offset| format!("{:?}", local_offset)
                children=move |local_offset| {
                    view! {
                        <button
                            data-testid=format!("offset-{local_offset}")
                            style:background-color=move || match offset.get() == local_offset {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_offset.set(local_offset);
                            }
                        >
                            {format!("{local_offset}")}
                        </button>
                    }
                }
            />
        </div>
    }
}
