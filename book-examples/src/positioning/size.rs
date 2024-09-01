use floating_ui_leptos::{
    DetectOverflowOptions, MiddlewareVec, Offset, OffsetOptions, Padding, RootBoundary, Size,
    SizeOptions,
};
use leptos::*;

use crate::components::{Chrome, Floating, GridItem, Reference, Scrollable};

#[component]
pub fn SizeDemo() -> impl IntoView {
    view! {
        <GridItem
            title="Size"
            description="Changes the size of your floating element to keep it in view."
            chrome=move || view! {
                <Chrome
                    label="Scroll the container"
                    scrollable=Scrollable::Y
                    center=true
                    shadow=false
                >
                    <Floating
                        class="h-[300px] overflow-hidden max-h-0"
                        middleware={
                            let middleware: MiddlewareVec = vec![
                                Box::new(Offset::new(OffsetOptions::Value(5.0))),
                                Box::new(Size::new(SizeOptions::default().detect_overflow(
                                    DetectOverflowOptions::default()
                                        .root_boundary(RootBoundary::Document)
                                        .padding(Padding::All(8.0))
                                ))),
                            ];

                            middleware
                        }
                        content=move || view! {
                            <div class="grid items-center text-sm font-bold">
                                Dropdown
                            </div>
                        }
                        reference=move |node_ref| view! {
                            <Reference node_ref=node_ref />
                        }
                    />
                </Chrome>
            }
        />
    }
}
