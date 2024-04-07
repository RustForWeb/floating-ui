use convert_case::{Case, Casing};
use floating_ui_leptos::{
    use_floating, Coords, Flip, FlipOptions, Inline, InlineOptions, MiddlewareVec, Placement, Size,
    SizeOptions, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{
    ev::MouseEvent,
    html::{Div, Span},
    *,
};

use crate::utils::all_placements::ALL_PLACEMENTS;

#[derive(Copy, Clone, Debug, PartialEq)]
enum ConnectedStatus {
    One,
    TwoDisjoined,
    TwoJoined,
    Three,
}

#[component]
pub fn Inline() -> impl IntoView {
    let reference_ref = create_node_ref::<Span>();
    let floating_ref = create_node_ref::<Div>();

    let (placement, set_placement) = create_signal(Placement::Bottom);
    let (status, set_status) = create_signal(ConnectedStatus::TwoDisjoined);
    let (open, set_open) = create_signal(false);
    let (mouse_coords, set_mouse_coords) = create_signal::<Option<Coords>>(None);

    let UseFloatingReturn { x, y, strategy, .. } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement.into())
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let mut options = InlineOptions::default();
                if let Some(mouse_coords) = mouse_coords() {
                    options = options.coords(mouse_coords);
                }

                let middleware: MiddlewareVec = vec![
                    Box::new(Inline::new(options)),
                    Box::new(Flip::new(FlipOptions::default())),
                    Box::new(Size::new(SizeOptions::default())),
                ];

                Some(middleware)
            })),
    );

    let text = move || {
        match status() {
        ConnectedStatus::One => "test",
        ConnectedStatus::TwoDisjoined => "Nulla rutrum dapibus turpis eu volutpat",
        ConnectedStatus::TwoJoined => "Nulla rutrum dapibus turpis eu volutpat. Duis cursus nisi massa, non dictum",
        ConnectedStatus::Three => "Nulla rutrum dapibus turpis eu volutpat. Duis cursus nisi massa, non dictum turpis interdum at. Nulla rutrum dapibus turpis eu volutpat",
    }
    };

    let handle_mouse_enter = move |event: MouseEvent| {
        set_mouse_coords(Some(Coords {
            x: event.client_x() as f64,
            y: event.client_y() as f64,
        }));
        set_open(true);
    };

    let handle_mouse_leave = move |_: MouseEvent| {
        set_mouse_coords(None);
        set_open(false);
    };

    // TODO: effect

    view! {
        <h1>Inline</h1>
        <p>The floating element should choose the most appropriate rect.</p>
        <div class="container">
            <p class="prose" style:padding="10px">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit.{' '}
                <span
                    _ref=reference_ref
                    style:color="royalblue"
                    style:font-weight="bold"
                    on:mouseenter=handle_mouse_enter
                    on:mouseleave=handle_mouse_leave
                >
                    {text}
                </span>. Ut eu magna eu augue efficitur bibendum id commodo tellus. Nullam
                gravida, mi nec sodales tincidunt, lorem orci aliquam ex, id commodo
                erat libero ut risus. Nam molestie non lectus sit amet tempus. Vivamus
                accumsan{' '}
                <strong style:color="red">nunc quis faucibus egestas</strong>.
                Duis cursus nisi massa, non dictum turpis interdum at.
            </p>

            // TODO: use this when refs can be re-mounted
            // <Show when=open>
            //    <div
            //         _ref=floating_ref
            //         class="floating"
            //         style:position=move || format!("{:?}", strategy()).to_lowercase()
            //         style:top=move || format!("{}px", y())
            //         style:left=move || format!("{}px", x())
            //         style:pointer-events="none"
            //     >
            //         Floating
            //     </div>
            // </Show>

            <div
                _ref=floating_ref
                class="floating"
                style:display=move || match open() {
                    true => "block",
                    false => "none"
                }
                style:position=move || format!("{:?}", strategy()).to_lowercase()
                style:top=move || format!("{}px", y())
                style:left=move || format!("{}px", x())
                style:pointer-events="none"
            >
                Floating
            </div>
        </div>

        <h2>Placement</h2>
        <div class="controls">
            <For
                each=|| ALL_PLACEMENTS
                key=|local_placement| format!("{:?}", local_placement)
                children=move |local_placement| view! {
                    <button
                        data-testid=format!("Placement{:?}", local_placement).to_case(Case::Kebab)
                        style:background-color=move || match placement() == local_placement {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_placement(local_placement)
                    >
                        {format!("{:?}", local_placement).to_case(Case::Kebab)}
                    </button>
                }
            />
        </div>

        <h2>Open</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("open-{}", value)
                        style:background-color=move || match open() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_open(value)
                    >
                        {format!("{}", value)}
                    </button>
                }
            />
        </div>

        <h2>Connected</h2>
        <div class="controls">
            <For
                each=|| [ConnectedStatus::One, ConnectedStatus::TwoDisjoined, ConnectedStatus::TwoJoined, ConnectedStatus::Three]
                key=|value| format!("{:?}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("fallbackPlacements-{}", match value {
                            ConnectedStatus::One => "1",
                            ConnectedStatus::TwoDisjoined => "2-disjoined",
                            ConnectedStatus::TwoJoined => "2-joined",
                            ConnectedStatus::Three => "3",
                        })
                        style:background-color=move || match status() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_status(value)
                    >
                        {match value {
                            ConnectedStatus::One => "1",
                            ConnectedStatus::TwoDisjoined => "2-disjoined",
                            ConnectedStatus::TwoJoined => "2-joined",
                            ConnectedStatus::Three => "3",
                        }}
                    </button>
                }
            />
        </div>
    }
}
