use std::{rc::Rc, time::Duration};

use convert_case::{Case, Casing};
use floating_ui_leptos::{
    use_floating, ClientRectObject, Coords, DefaultVirtualElement, Flip, FlipOptions, Inline,
    InlineOptions, MiddlewareVec, Placement, Size, SizeOptions, UseFloatingOptions,
    UseFloatingReturn, VirtualElement, VirtualElementOrNodeRef,
};
use leptos::{
    ev::{self, MouseEvent},
    prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

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
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (placement, set_placement) = signal(Placement::Bottom);
    let (status, set_status) = signal(ConnectedStatus::TwoDisjoined);
    let (open, set_open) = signal(false);
    let (mouse_coords, set_mouse_coords) = signal::<Option<Coords>>(None);

    let reference_signal = RwSignal::<VirtualElementOrNodeRef>::new(reference_ref.into());

    let UseFloatingReturn { x, y, strategy, .. } = use_floating(
        reference_signal,
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement.into())
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let mut options = InlineOptions::default();
                if let Some(mouse_coords) = mouse_coords.get() {
                    options = options.coords(mouse_coords);
                }

                let middleware: MiddlewareVec = vec![
                    Box::new(Inline::new(options)),
                    Box::new(Flip::new(FlipOptions::default())),
                    Box::new(Size::new(SizeOptions::default())),
                ];

                Some(SendWrapper::new(middleware))
            })),
    );

    let text = move || {
        match status.get() {
        ConnectedStatus::One => "test",
        ConnectedStatus::TwoDisjoined => "Nulla rutrum dapibus turpis eu volutpat",
        ConnectedStatus::TwoJoined => "Nulla rutrum dapibus turpis eu volutpat. Duis cursus nisi massa, non dictum",
        ConnectedStatus::Three => "Nulla rutrum dapibus turpis eu volutpat. Duis cursus nisi massa, non dictum turpis interdum at. Nulla rutrum dapibus turpis eu volutpat",
    }
    };

    let handle_mouse_enter = move |event: MouseEvent| {
        set_mouse_coords.set(Some(Coords {
            x: event.client_x() as f64,
            y: event.client_y() as f64,
        }));
        set_open.set(true);
    };

    let handle_mouse_leave = move |_: MouseEvent| {
        set_mouse_coords.set(None);
        set_open.set(false);
    };

    let handle_mouse_up = move |event: MouseEvent| {
        let target: web_sys::Node = event_target(&event);

        if let Some(floating) = floating_ref.get() {
            if floating.contains(Some(&target)) {
                return;
            }
        }

        set_timeout(
            move || {
                let selection = window()
                    .get_selection()
                    .expect("Window should have selection.");
                let range =
                    selection
                        .as_ref()
                        .and_then(|selection| match selection.range_count() {
                            0 => None,
                            _ => selection.get_range_at(0).ok(),
                        });

                if selection.is_some_and(|selection| selection.is_collapsed()) {
                    set_open.set(false);
                    return;
                }

                if let Some(range) = range {
                    reference_signal.set(
                        (Box::new(
                            DefaultVirtualElement::new(Rc::new({
                                let range = range.clone();

                                move || range.get_bounding_client_rect().into()
                            }))
                            .get_client_rects(Rc::new({
                                move || {
                                    ClientRectObject::from_dom_rect_list(
                                        range
                                            .get_client_rects()
                                            .expect("Range should have client rects."),
                                    )
                                }
                            })),
                        ) as Box<dyn VirtualElement<web_sys::Element>>)
                            .into(),
                    );
                    set_open.set(true);
                }
            },
            Duration::from_millis(0),
        );
    };
    let handle_mouse_down = move |event: MouseEvent| {
        let target: web_sys::Node = event_target(&event);

        if let Some(floating) = floating_ref.get() {
            if floating.contains(Some(&target)) {
                return;
            }
        }

        if window()
            .get_selection()
            .expect("Window should have selection.")
            .is_some_and(|selection| selection.is_collapsed())
        {
            set_open.set(false);
        }
    };

    let mouse_up_handle = window_event_listener(ev::mouseup, handle_mouse_up);
    let mouse_down_handle = window_event_listener(ev::mousedown, handle_mouse_down);

    on_cleanup(move || {
        mouse_up_handle.remove();
        mouse_down_handle.remove();
    });

    view! {
        <h1>Inline</h1>
        <p>The floating element should choose the most appropriate rect.</p>
        <div class="container">
            <p class="prose" style:padding="10px">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit.{' '}
                <strong
                    node_ref=reference_ref
                    style:color="royalblue"
                    on:mouseenter=handle_mouse_enter
                    on:mouseleave=handle_mouse_leave
                >
                    {text}
                </strong>. Ut eu magna eu augue efficitur bibendum id commodo tellus. Nullam
                gravida, mi nec sodales tincidunt, lorem orci aliquam ex, id commodo
                erat libero ut risus. Nam molestie non lectus sit amet tempus. Vivamus
                accumsan{' '}
                <strong style:color={"red"}>nunc quis faucibus egestas</strong>.
                Duis cursus nisi massa, non dictum turpis interdum at.
            </p>

            <Show when=move || open.get()>
               <div
                    node_ref=floating_ref
                    class="floating"
                    style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                    style:top=move || format!("{}px", y.get())
                    style:left=move || format!("{}px", x.get())
                    style:pointer-events="none"
                >
                    Floating
                </div>
            </Show>
        </div>

        <h2>Placement</h2>
        <div class="controls">
            <For
                each=|| ALL_PLACEMENTS
                key=|local_placement| format!("{:?}", local_placement)
                children=move |local_placement| view! {
                    <button
                        data-testid=format!("Placement{:?}", local_placement).to_case(Case::Kebab)
                        style:background-color=move || if placement.get() == local_placement {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_placement.set(local_placement)
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
                        style:background-color=move || if open.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_open.set(value)
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
                        data-testid=format!("connected-{}", match value {
                            ConnectedStatus::One => "1",
                            ConnectedStatus::TwoDisjoined => "2-disjoined",
                            ConnectedStatus::TwoJoined => "2-joined",
                            ConnectedStatus::Three => "3",
                        })
                        style:background-color=move || if status.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_status.set(value)
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
