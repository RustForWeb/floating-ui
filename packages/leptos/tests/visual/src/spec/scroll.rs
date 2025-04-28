use convert_case::{Case, Casing};
use floating_ui_leptos::{Strategy, UseFloatingOptions, UseFloatingReturn, use_floating};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

use crate::utils::use_scroll::{UseScrollOptions, UseScrollReturn, use_scroll};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Node {
    ReferenceScrollParent,
    FloatingScrollParent,
    SameScrollParent,
    Body,
}

const ALL_NODES: [Node; 4] = [
    Node::ReferenceScrollParent,
    Node::FloatingScrollParent,
    Node::SameScrollParent,
    Node::Body,
];
const ALL_STRATEGIES: [Strategy; 2] = [Strategy::Absolute, Strategy::Fixed];

#[component]
pub fn Scroll() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (strategy, set_strategy) = signal(Strategy::Absolute);
    let (node, set_node) = signal(Node::ReferenceScrollParent);

    let UseFloatingReturn { x, y, update, .. } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default().strategy(strategy.into()),
    );

    let UseScrollReturn {
        scroll_ref,
        indicator,
        ..
    } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update: update.clone(),
        rtl: None::<bool>.into(),
        disable_ref_updates: Some(true),
    });

    Effect::new(move || {
        _ = strategy.get();
        _ = node.get();
        update();
    });

    let reference_view = move || {
        view! {
            <div
                node_ref=reference_ref
                class="reference"
                style=move || match node.get() {
                    Node::FloatingScrollParent => "position: relative; top: -350px;",
                    _ => ""
                }
            >
                Reference
            </div>
        }
    };

    let floating_view = move || {
        view! {
            <div
                node_ref=floating_ref
                class="floating"
                style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                style:top=move || format!("{}px", y.get())
                style:left=move || format!("{}px", x.get())
            >
                Floating
            </div>
        }
    };

    view! {
        <h1>Scroll</h1>
        <p>
            The floating element should be positioned correctly when a certain node has been scrolled.
        </p>
        <div class="container">
            <Show
                when=move || node.get() != Node::Body
                fallback=move || view! {
                    {reference_view}
                    {floating_view}
                }
            >
                <div
                    node_ref=scroll_ref
                    class="scroll"
                    style:position=move || match node.get() {
                        Node::FloatingScrollParent | Node::SameScrollParent => "relative",
                        _ => "",
                    }
                >
                    {{
                        let indicator = indicator.clone();

                        move || indicator()
                    }}
                    <Show when=move || node.get() != Node::FloatingScrollParent>
                        {reference_view}
                    </Show>
                    {floating_view}
                </div>
                <Show when=move || node.get() == Node::FloatingScrollParent>
                    {reference_view}
                </Show>
            </Show>
        </div>

        <h3>Strategy</h3>
        <div class="controls">
            <For
                each=|| ALL_STRATEGIES
                key=|local_strategy| format!("{local_strategy:?}")
                children=move |local_strategy| view! {
                    <button
                        data-testid=move || format!("Strategy{local_strategy:?}").to_case(Case::Kebab)
                        style:background-color=move || if strategy.get() == local_strategy {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_strategy.set(local_strategy)
                    >
                        {format!("{local_strategy:?}").to_case(Case::Kebab)}
                    </button>
                }
            />
        </div>

        <h3>Node</h3>
        <div class="controls">
            <For
                each=|| ALL_NODES
                key=|local_node| format!("{local_node:?}")
                children=move |local_node| view! {
                    <button
                        data-testid=move || format!("scroll-{}", format!("{local_node:?}").to_case(Case::Camel))
                        style:background-color=move || if node.get() == local_node {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_node.set(local_node)
                    >
                        {format!("{local_node:?}").to_case(Case::Camel)}
                    </button>
                }
            />
        </div>

        <Show when=move || node.get() == Node::Body>
            <div style:width="1px" style:height="1500px" />
        </Show>
    }
}
