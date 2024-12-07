use floating_ui_leptos::{
    DetectOverflowOptions, Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement,
    RootBoundary,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::{
    components::{Chrome, Floating, GridItem, Reference, Scrollable},
    utils::rem_to_px,
};

#[component]
pub fn FlipDemo() -> impl IntoView {
    let boundary_ref = AnyNodeRef::new();

    Effect::new(move |_| {
        if let Some(boundary) = boundary_ref.get() {
            boundary
                .first_element_child()
                .expect("First element child should exist.")
                .set_scroll_top(rem_to_px(275.0 / 16.0) as i32);
        }
    });

    view! {
        <GridItem
            title="Flip"
            description="Changes the placement of your floating element to keep it in view."
            chrome=move || view! {
                <div node_ref={boundary_ref} class="relative overflow-hidden">
                    <Chrome
                        label="Scroll down"
                        scrollable=Scrollable::Y
                        center=true
                        shadow=false
                    >
                        <Floating
                            placement=Placement::Top
                            middleware={
                                let middleware: MiddlewareVec = vec![
                                    Box::new(Offset::new(OffsetOptions::Value(5.0))),
                                    Box::new(Flip::new(FlipOptions::default().detect_overflow(
                                        DetectOverflowOptions::default().root_boundary(RootBoundary::Document)
                                    ))),
                                ];

                                SendWrapper::new(middleware)
                            }
                            content=move || view! {
                                <span class="text-sm font-bold">
                                    Tooltip
                                </span>
                            }
                            reference=move |node_ref| view! {
                                <Reference node_ref=node_ref />
                            }
                        />
                    </Chrome>
                </div>
            }
        />
    }
}
