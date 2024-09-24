use floating_ui_leptos::{
    use_floating, ApplyState, IntoReference, MiddlewareState, MiddlewareVec, Size, SizeOptions,
    UseFloatingOptions, UseFloatingReturn,
};
use leptos::{html::Div, *};
use wasm_bindgen::JsCast;

const SIZES: [f64; 4] = [0.0, 0.25, 0.5, 0.75];
const INTEGER: f64 = 80.0;

#[component]
pub fn DecimalSize() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();

    let (size, set_size) = create_signal(INTEGER);
    let (truncate, set_truncate) = create_signal(false);

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
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default().middleware(middleware.into()),
    );

    let size_update = update.clone();
    let truncate_update = update.clone();

    view! {
        <h1>Decimal Size</h1>
        <p>
            The floating element should be positioned correctly on the bottom when
            the reference and floating elements have a non-integer size (width/height).
        </p>
        <div class="container">
            <div _ref=reference_ref class="reference" style=move || format!("width: {}px; height: {}px;", size.get(), size.get())>
                Reference
            </div>
            <div
                _ref=floating_ref
                class="floating"
                style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                style:top=move || format!("{}px", y.get())
                style:left=move || format!("{}px", x.get())
                style:width=move || match truncate.get() {
                    true => "auto".into(),
                    false => format!("{}px", size.get()),
                }
                style:height=move || match truncate.get() {
                    true => "auto".into(),
                    false => format!("{}px", size.get()),
                }
                style:display=move || match truncate.get() {
                    true => "block",
                    false => "",
                }
                style:overflow=move || match truncate.get() {
                    true => "hidden",
                    false => "",
                }
                style:text-overflow=move || match truncate.get() {
                    true => "ellipsis",
                    false => "",
                }
                style:white-space=move || match truncate.get() {
                    true => "nowrap",
                    false => "",
                }
            >
                {move || match truncate.get() {
                    true => "Long text that will be truncated",
                    false => "Floating",
                }}
            </div>
        </div>

        <div class="controls">
            <For
                each=|| SIZES
                key=|local_size| format!("{:?}", local_size)
                children=move |local_size| {
                    let size_update = size_update.clone();
                    view! {
                        <button
                            data-testid=format!("decimal-size-{}", match local_size {
                                0.0 => ".0".into(),
                                _ => local_size.to_string()[1..].to_string()
                            })
                            style:background-color=move || match size.get().fract() == local_size {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_size.set(INTEGER + local_size);
                                size_update();
                            }
                        >
                            {match local_size {
                                0.0 => ".0".into(),
                                _ => local_size.to_string()[1..].to_string()
                            }}
                        </button>
                    }
                }
            />
        </div>

        <h2>Truncate</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| {
                    let truncate_update = truncate_update.clone();
                    view! {
                        <button
                            data-testid=format!("truncate-{}", value)
                            style:background-color=move || match truncate.get() == value {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_truncate.set(value);
                                truncate_update();
                            }
                        >
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>
    }
}
