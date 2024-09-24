use std::{ops::Deref, rc::Rc};

use convert_case::{Case, Casing};
use floating_ui_leptos::{
    use_floating, DefaultVirtualElement, MiddlewareVec, Shift, ShiftOptions, UseFloatingOptions,
    UseFloatingReturn, VirtualElement, VirtualElementOrNodeRef,
};
use leptos::{
    html::{AnyElement, Div},
    *,
};
use wasm_bindgen::JsCast;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Node {
    None,
    Reference,
    Floating,
    Body,
    Html,
    OffsetParent,
    OffsetParent3d,
    OffsetParentInverse,
    OffsetParentReference,
    Virtual,
    Inline,
}

const ALL_NODES: [Node; 11] = [
    Node::None,
    Node::Reference,
    Node::Floating,
    Node::Body,
    Node::Html,
    Node::OffsetParent,
    Node::OffsetParent3d,
    Node::OffsetParentInverse,
    Node::OffsetParentReference,
    Node::Virtual,
    Node::Inline,
];

#[component]
pub fn Transform() -> impl IntoView {
    let reference_ref = create_node_ref::<AnyElement>();
    let floating_ref = create_node_ref::<Div>();
    let offset_parent_ref = create_node_ref::<Div>();

    let (node, set_node) = create_signal(Node::None);

    let reference_signal: MaybeProp<VirtualElementOrNodeRef<NodeRef<AnyElement>, AnyElement>> =
        MaybeProp::derive(move || match node.get() {
            Node::Virtual => {
                let context_element = document()
                    .get_element_by_id("virtual-context")
                    .expect("Element should exist.");

                Some(
                    (Box::new(
                        DefaultVirtualElement::new(Rc::new({
                            let context_element = context_element.clone();

                            move || context_element.get_bounding_client_rect().into()
                        }))
                        .context_element(context_element),
                    ) as Box<dyn VirtualElement<web_sys::Element>>)
                        .into(),
                )
            }
            _ => Some(reference_ref.into()),
        });

    let middleware: MiddlewareVec = vec![Box::new(Shift::new(
        ShiftOptions::default().cross_axis(true),
    ))];

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_signal,
        floating_ref,
        UseFloatingOptions::default()
            .middleware(middleware.into())
            .while_elements_mounted_auto_update(),
    );

    create_effect(move |_| {
        let element = match node.get() {
            Node::Html => document()
                .document_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>()),
            Node::Body => document().body(),
            Node::OffsetParent
            | Node::OffsetParent3d
            | Node::OffsetParentInverse
            | Node::OffsetParentReference
            | Node::Virtual
            | Node::Inline => offset_parent_ref
                .get()
                .map(|offset_parent| offset_parent.deref().clone().into()),
            _ => None,
        };

        if let Some(element) = element {
            let transform = match node.get() {
                Node::OffsetParent3d => "scale3d(0.5, 0.2, 0.7) translate3d(2rem, -2rem, 0)",
                Node::OffsetParentInverse | Node::Virtual => "scale(0.5)",
                _ => "scale(0.5) translate(2rem, -2rem)",
            };

            element
                .style()
                .set_property("transform", transform)
                .expect("Style should be updated.");

            if node.get() == Node::Virtual {}
        }

        update();
    });

    on_cleanup(move || {
        let element = match node.get() {
            Node::Html => document()
                .document_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>()),
            Node::Body => document().body(),
            Node::OffsetParent
            | Node::OffsetParent3d
            | Node::OffsetParentInverse
            | Node::OffsetParentReference
            | Node::Virtual
            | Node::Inline => offset_parent_ref
                .get()
                .map(|offset_parent| offset_parent.deref().clone().into()),
            _ => None,
        };

        if let Some(element) = element {
            element
                .style()
                .remove_property("transform")
                .expect("Style should be updated.");
        }
    });

    view! {
        <h1>Transform</h1>
        <p>
            The floating element should be positioned correctly on the bottom when a certain node has been transformed.
        </p>
        <div
            _ref=offset_parent_ref
            class="container"
            style:overflow="hidden"
            style:position=move || match node.get() {
                Node::OffsetParent => "relative",
                _ => ""
            }
        >
            <span style:position=move || match node.get() {
                Node::Inline => "relative",
                _ => ""
            }>
                <Show when=move || node.get() == Node::Virtual>
                    <div
                        id="virtual-context"
                        style:width="50px"
                        style:height="50px"
                        style:background="black"
                    />
                </Show>
                {move || view!{
                    <div
                        class="reference"
                        style:transform=move || match node.get() {
                            Node::Reference | Node::OffsetParentReference => "scale(1.25) translate(2rem, -2rem)",
                            _ => ""
                        }
                    >
                        Reference
                    </div>
                }.into_any().node_ref(reference_ref)}
                <div
                    _ref=floating_ref
                    class="floating"
                    style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                    style:top=move || format!("{}px", y.get())
                    style:left=move || format!("{}px", x.get())
                    style:transform=move || match node.get() {
                        Node::Floating => "scale(1.25)",
                        _ => ""
                    }
                    style:transform-origin="top"
                >
                    Floating
                </div>
            </span>
        </div>

        <div class="controls">
            <For
                each=|| ALL_NODES
                key=|local_node| format!("{:?}", local_node)
                children=move |local_node| view! {
                    <button
                        data-testid=move || format!("transform-{}", match local_node {
                            Node::None => "null".into(),
                            Node::OffsetParent3d => "offsetParent-3d".into(),
                            Node::OffsetParentInverse => "offsetParent-inverse".into(),
                            Node::OffsetParentReference => "offsetParent-reference".into(),
                            _ => format!("{:?}", local_node).to_case(Case::Camel)
                        })
                        style:background-color=move || match node.get() == local_node {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_node.set(local_node)
                    >
                        {match local_node {
                            Node::OffsetParent3d => "offsetParent-3d".into(),
                            Node::OffsetParentInverse => "offsetParent-inverse".into(),
                            Node::OffsetParentReference => "offsetParent-reference".into(),
                            _ => format!("{:?}", local_node).to_case(Case::Camel)
                        }}
                    </button>
                }
            />
        </div>
    }
}
