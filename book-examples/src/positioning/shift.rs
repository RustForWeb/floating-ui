use floating_ui_leptos::{
    Boundary, DetectOverflowOptions, MiddlewareVec, Offset, OffsetOptions, Padding,
    PartialSideObject, Placement, RootBoundary, Shift, ShiftOptions,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::{
    components::{Chrome, Floating, GridItem, Reference, Scrollable},
    utils::rem_to_px,
};

#[component]
pub fn ShiftDemo() -> impl IntoView {
    let boundary_ref = AnyNodeRef::new();

    Effect::new(move |_| {
        if let Some(boundary) = boundary_ref.get() {
            boundary
                .first_element_child()
                .expect("First element child should exist.")
                .set_scroll_top(rem_to_px(200.0 / 16.0) as i32);
        }
    });

    view! {
        <GridItem
            title="Shift"
            description="Shifts your floating element to keep it in view."
            chrome=move || view! {
                <div node_ref={boundary_ref} class="relative overflow-hidden">
                    <Chrome
                        label="Scroll the container"
                        scrollable=Scrollable::Y
                        relative=false
                        shadow=false
                    >
                        <Floating
                            placement=Placement::Right
                            middleware=MaybeProp::derive(move || {
                                let mut detect_overflow_options =  DetectOverflowOptions::default()
                                    .root_boundary(RootBoundary::Document)
                                    .padding(Padding::PerSide(PartialSideObject {
                                        top: Some(rem_to_px(54.0 / 16.0)),
                                        right: None,
                                        bottom: Some(rem_to_px(5.0 / 16.0)),
                                        left: None
                                    }));

                                if let Some(boundary) = boundary_ref.get() {
                                    detect_overflow_options = detect_overflow_options.boundary(Boundary::Element(boundary.clone()));
                                }

                                let middleware: MiddlewareVec = vec![
                                    Box::new(Offset::new(OffsetOptions::Value(5.0))),
                                    Box::new(Shift::new(ShiftOptions::default().detect_overflow(detect_overflow_options))),
                                ];

                                Some(SendWrapper::new(middleware))
                            })
                            content=move || view! {
                                <div class="grid h-48 w-20 place-items-center text-sm font-bold">
                                    Popover
                                </div>
                            }
                            reference=move |node_ref| view! {
                                <Reference node_ref=node_ref class="ml-[5%] sm:ml-[33%]" />
                            }
                        />
                    </Chrome>
                </div>
            }
        />
    }
}
