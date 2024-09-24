use convert_case::{Case, Casing};
use floating_ui_leptos::{use_floating, UseFloatingOptions, UseFloatingReturn};
use leptos::{
    html::{AnyElement, Div},
    *,
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Node {
    Table,
    Td,
    Th,
}

const ALL_NODES: [Node; 3] = [Node::Table, Node::Td, Node::Th];

#[component]
pub fn Table() -> impl IntoView {
    let reference_table_ref = create_node_ref::<AnyElement>();
    let reference_tr_ref = create_node_ref::<AnyElement>();
    let reference_td_ref = create_node_ref::<AnyElement>();
    let floating_ref = create_node_ref::<Div>();

    let (same_parent, set_same_parent) = create_signal(false);
    let (node, set_node) = create_signal(Node::Td);

    let reference_signal = MaybeProp::derive(move || match node.get() {
        Node::Table => Some(reference_table_ref.into()),
        Node::Td => Some(reference_td_ref.into()),
        Node::Th => Some(reference_tr_ref.into()),
    });

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_signal,
        floating_ref,
        UseFloatingOptions::default(),
    );

    let same_parent_update = update.clone();
    let node_update = update.clone();
    _ = watch(
        move || same_parent.get(),
        move |_, _, _| same_parent_update(),
        false,
    );
    _ = watch(move || node.get(), move |_, _, _| node_update(), false);

    let floating_view = move || {
        view! {
            <div
                _ref=floating_ref
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
        <h1>Table</h1>
        <p>
            The floating element should be correctly positioned when the reference or ancestor is a table element.
        </p>
        <div class="container">
            {move || view! {
                <table>
                    <thead>
                        {move || view! {
                            <tr>
                                <th>Reference th</th>
                            </tr>
                        }.into_any().node_ref(reference_tr_ref)}
                    </thead>
                    <tbody>
                        <tr>
                            {move || view! {
                                <td>
                                    Reference td
                                    <Show when=move || same_parent.get()>
                                        {floating_view}
                                    </Show>
                                </td>
                            }.into_any().node_ref(reference_td_ref)}
                        </tr>
                    </tbody>
                </table>
            }.into_any().node_ref(reference_table_ref)}

            <Show when=move || !same_parent.get()>
                {floating_view}
            </Show>
        </div>

        <h2>Inside table</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("inside-{}", value)
                        style:background-color=move || match same_parent.get() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_same_parent.set(value)
                    >
                        {format!("{}", value)}
                    </button>
                }
            />
        </div>

        <h2>Reference node</h2>
        <div class="controls">
            <For
                each=|| ALL_NODES
                key=|local_node| format!("{:?}", local_node)
                children=move |local_node| view! {
                    <button
                        data-testid=move || format!("reference-{}", format!("{:?}", local_node).to_case(Case::Camel))
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
    }
}
