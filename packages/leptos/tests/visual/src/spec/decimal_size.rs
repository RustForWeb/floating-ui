use floating_ui_leptos::{
    ApplyState, MiddlewareState, MiddlewareVec, Size, SizeOptions, UseFloatingOptions,
    UseFloatingReturn, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

const SIZES: [f64; 4] = [0.0, 0.25, 0.5, 0.75];
const INTEGER: f64 = 80.0;

#[component]
pub fn DecimalSize() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (size, set_size) = signal(INTEGER);
    let (truncate, set_truncate) = signal(false);

    let middleware: MiddlewareVec = vec![Box::new(Size::new(SizeOptions::default().apply(
        &|ApplyState { state, .. }| {
            let MiddlewareState {
                elements, rects, ..
            } = state;

            let floating = (*elements.floating)
                .clone()
                .unchecked_into::<web_sys::HtmlElement>();

            floating
                .style()
                .set_property("width", &format!("{}px", rects.floating.width))
                .expect("Style should be updated.");
        },
    )))];

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default().middleware(SendWrapper::new(middleware)),
    );

    Effect::new(move || {
        _ = size.get();
        _ = truncate.get();
        update();
    });

    view! {
        <h1>Decimal Size</h1>
        <p>
            The floating element should be positioned correctly on the bottom when
            the reference and floating elements have a non-integer size (width/height).
        </p>
        <div class="container">
            <div node_ref=reference_ref class="reference" style=move || format!("width: {}px; height: {}px;", size.get(), size.get())>
                Reference
            </div>
            <div
                node_ref=floating_ref
                class="floating"
                style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                style:top=move || format!("{}px", y.get())
                style:left=move || format!("{}px", x.get())
                style:width=move || if truncate.get() {
                    "auto".to_owned()
                } else {
                    format!("{}px", size.get())
                }
                style:height=move || if truncate.get() {
                    "auto".to_owned()
                } else {
                    format!("{}px", size.get())
                }
                style:display=move || if truncate.get() {
                    "block"
                } else {
                    ""
                }
                style:overflow=move || if truncate.get() {
                    "hidden"
                } else {
                    ""
                }
                style:text-overflow=move || if truncate.get() {
                    "ellipsis"
                } else {
                    ""
                }
                style:white-space=move || if truncate.get() {
                    "nowrap"
                } else {
                    ""
                }
            >
                {move || if truncate.get() {
                    "Long text that will be truncated"
                } else {
                    "Floating"
                }}
            </div>
        </div>

        <div class="controls">
            <For
                each=|| SIZES
                key=|local_size| format!("{local_size:?}")
                children=move |local_size| view! {
                    <button
                        data-testid=format!("decimal-size-{}", match local_size {
                            0.0 => ".0".to_owned(),
                            _ => local_size.to_string()[1..].to_string()
                        })
                        style:background-color=move || if size.get().fract() == local_size {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_size.set(INTEGER + local_size)
                    >
                        {match local_size {
                            0.0 => ".0".to_owned(),
                            _ => local_size.to_string()[1..].to_string()
                        }}
                    </button>
                }
            />
        </div>

        <h2>Truncate</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| view! {
                    <button
                        data-testid=format!("truncate-{}", value)
                        style:background-color=move || if truncate.get() == value {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_truncate.set(value)
                    >
                        {format!("{value}")}
                    </button>
                }
            />
        </div>
    }
}
