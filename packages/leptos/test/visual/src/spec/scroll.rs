use convert_case::{Case, Casing};
use floating_ui_leptos::{use_floating, Strategy, UseFloatingOptions, UseFloatingReturn};
use leptos::{html::Div, *};

use crate::utils::use_scroll::{use_scroll, UseScrollOptions, UseScrollReturn};

#[derive(Copy, Clone, Debug, PartialEq)]
enum NodeType {
    ReferenceScrollParent,
    FloatingScrollParent,
    SameScrollParent,
    Body,
}

const ALL_NODE_TYPES: [NodeType; 4] = [
    NodeType::ReferenceScrollParent,
    NodeType::FloatingScrollParent,
    NodeType::SameScrollParent,
    NodeType::Body,
];
const ALL_STRATEGIES: [Strategy; 2] = [Strategy::Absolute, Strategy::Fixed];

#[component]
pub fn Scroll() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();

    let (strategy, set_strategy) = create_signal(Strategy::Absolute);
    let (node_type, set_node_type) = create_signal(NodeType::ReferenceScrollParent);

    let UseFloatingReturn { x, y, update, .. } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default().strategy(strategy.into()),
    );

    let UseScrollReturn {
        scroll_ref,
        indicator,
    } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: None,
    });

    let reference_view = move || {
        view! {
            <div
                _ref=reference_ref
                class="reference"
                style=move || match node_type() {
                    NodeType::FloatingScrollParent => "position: relative; top: -350px;",
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
                _ref=floating_ref
                class="floating"
                style:position=move || format!("{:?}", strategy()).to_lowercase()
                style:top=move || format!("{}px", y())
                style:left=move || format!("{}px", x())
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
                when=move || node_type() != NodeType::Body
                fallback=move || view! {
                    {reference_view}
                    {floating_view}
                }
            >
                <div
                    _ref=scroll_ref
                    class="scroll"
                    style:position=move || match node_type() {
                        NodeType::FloatingScrollParent | NodeType::SameScrollParent => "relative",
                        _ => "",
                    }
                >
                    {indicator.clone()}
                    <Show when=move || node_type() != NodeType::FloatingScrollParent>
                        {reference_view}
                    </Show>
                    {floating_view}
                </div>
                <Show when=move || node_type() == NodeType::FloatingScrollParent>
                    {reference_view}
                </Show>
            </Show>
        </div>

        <h3>Strategy</h3>
        <div class="controls">
            <For
                each=|| ALL_STRATEGIES
                key=|local_strategy| format!("{:?}", local_strategy)
                children=move |local_strategy| view! {
                    <button
                        data-testid=move || format!("Stategy{:?}", local_strategy).to_case(Case::Kebab)
                        style:background-color=move || match strategy() == local_strategy {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_strategy(local_strategy)
                    >
                        {format!("{:?}", local_strategy).to_case(Case::Kebab)}
                    </button>
                }
            />
        </div>

        <h3>Node</h3>
        <div class="controls">
            <For
                each=|| ALL_NODE_TYPES
                key=|local_node_type| format!("{:?}", local_node_type)
                children=move |local_node_type| view! {
                    <button
                        data-testid=move || format!("Scroll{:?}", local_node_type).to_case(Case::Camel)
                        style:background-color=move || match node_type() == local_node_type {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_node_type(local_node_type)
                    >
                        {format!("{:?}", local_node_type).to_case(Case::Camel)}
                    </button>
                }
            />
        </div>

        <Show when=move || node_type() == NodeType::Body>
            <div style:width="1px" style:height="1500px" />
        </Show>
    }
}
