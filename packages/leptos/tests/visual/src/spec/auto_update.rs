use convert_case::{Case, Casing};
use floating_ui_leptos::{
    AutoUpdateOptions, Strategy, UseFloatingOptions, UseFloatingReturn, auto_update, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{HtmlElement, window};

#[derive(Copy, Clone, Debug, PartialEq)]
enum LayoutShift {
    Move,
    Insert,
    Delete,
    None,
    Init,
}

const ALL_LAYOUT_SHIFTS: [LayoutShift; 5] = [
    LayoutShift::Move,
    LayoutShift::Insert,
    LayoutShift::Delete,
    LayoutShift::None,
    LayoutShift::Init,
];

#[component]
pub fn AutoUpdate() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (layout_shift, set_layout_shift) = signal(LayoutShift::None);
    let (options, set_options) = signal(AutoUpdateOptions {
        ancestor_scroll: Some(false),
        ancestor_resize: Some(false),
        element_resize: Some(false),
        layout_shift: None,
        animation_frame: Some(false),
    });
    let (reference_size, set_reference_size) = signal(200);
    let (floating_size, set_floating_size) = signal(100);
    let (while_elements_mounted, set_while_elements_mounted) = signal(false);

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .strategy(Strategy::Fixed)
            .while_elements_mounted_auto_update_with_enabled(while_elements_mounted.into()),
    );

    type CleanupFn = Box<dyn Fn()>;
    let cleanup: StoredValue<Option<SendWrapper<CleanupFn>>> = StoredValue::new(None);

    Effect::new({
        let update = update.clone();

        move |_| {
            if let Some(reference) = reference_ref.get()
                && let Some(floating) = floating_ref.get()
            {
                if let Some(cleanup) = &*cleanup.read_value() {
                    cleanup();
                }

                let size_factor = match layout_shift.get() {
                    LayoutShift::Move => 0.9,
                    _ => 1.0,
                };

                // Match React test behaviour by moving the size change from style attributes to here.
                // The style attributes update after this effect, so `auto_update` would not use the correct size.
                let style = reference.unchecked_ref::<web_sys::HtmlElement>().style();

                style
                    .set_property(
                        "width",
                        &format!("{}px", reference_size.get() as f64 * size_factor),
                    )
                    .expect("Style should be updated.");
                style
                    .set_property(
                        "height",
                        &format!("{}px", reference_size.get() as f64 * size_factor),
                    )
                    .expect("Style should be updated.");

                cleanup.set_value(Some(SendWrapper::new(auto_update(
                    (&reference).into(),
                    Some(&floating),
                    (*update).clone(),
                    options
                        .get()
                        .layout_shift(layout_shift.get() != LayoutShift::None),
                ))));
            }
        }
    });

    on_cleanup(move || {
        if let Some(cleanup) = &*cleanup.read_value() {
            cleanup();
        }
    });

    Effect::new(move |_| {
        if options.get().element_resize.unwrap() {
            set_reference_size.set(100);
            set_floating_size.set(50);
        } else {
            set_reference_size.set(200);
            set_floating_size.set(100);
        }
    });

    view! {
        <h1>AutoUpdate</h1>
        <Show when=move || layout_shift.get() != LayoutShift::Delete>
            <p>The floating element should update when required.</p>
        </Show>
        <Show when=move || layout_shift.get() == LayoutShift::Insert>
            <p>inserted content</p>
        </Show>
        <div
            class="container"
            data-flexible
        >
            <div
                node_ref=reference_ref
                class="reference"
                style:position="relative"
                style:top=move || match layout_shift.get() {
                    LayoutShift::Move => "-50px",
                    _ => ""
                }
                style:left=move || match layout_shift.get() {
                    LayoutShift::Move => "50px",
                    _ => ""
                }
                style:width=move || format!("{}px", match layout_shift.get() {
                    LayoutShift::Move => reference_size.get() as f64 * 0.9,
                    _ => reference_size.get() as f64
                })
                style:height=move || format!("{}px", match layout_shift.get() {
                    LayoutShift::Move => reference_size.get() as f64 * 0.9,
                    _ => reference_size.get() as f64
                })
                style:animation=move || match options.get().animation_frame {
                    Some(true) => "scale 0.5s ease infinite alternate",
                    _ => ""
                }
            >
                Reference
            </div>
            <div
                node_ref=floating_ref
                class="floating"
                style:position=move || format!("{:?}", strategy.get()).to_lowercase()
                style:top=move || format!("{}px", y.get())
                style:left=move || format!("{}px", x.get())
                style:width=move || format!("{}px", floating_size.get())
                style:height=move || format!("{}px", floating_size.get())
            >
                Floating
            </div>
        </div>

        <h2>ancestorScroll</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| {
                    view! {
                        <button
                            data-testid=format!("ancestorScroll-{}", value)
                            style:background-color=move || if options.get().ancestor_scroll.unwrap() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_options.set(options.get().ancestor_scroll(value))
                        >
                            {format!("{value}")}
                        </button>
                    }
                }
            />
        </div>

        <h2>ancestorResize</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| {
                    view! {
                        <button
                            data-testid=format!("ancestorResize-{}", value)
                            style:background-color=move || if options.get().ancestor_resize.unwrap() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_options.set(options.get().ancestor_resize(value))
                        >
                            {format!("{value}")}
                        </button>
                    }
                }
            />
        </div>

        <h2>elementResize</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| {
                    view! {
                        <button
                            data-testid=format!("elementResize-{}", value)
                            style:background-color=move || if options.get().element_resize.unwrap() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_options.set(options.get().element_resize(value))
                        >
                            {format!("{value}")}
                        </button>
                    }
                }
            />
        </div>

        <h2>layoutShift</h2>
        <div class="controls">
            <For
                each=|| ALL_LAYOUT_SHIFTS
                key=|local_layout_shift| format!("{local_layout_shift:?}")
                children=move |local_layout_shift| view! {
                    <button
                        data-testid=move || format!("layoutShift-{}", format!("{local_layout_shift:?}").to_case(Case::Camel))
                        style:background-color=move || if layout_shift.get() == local_layout_shift {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_layout_shift.set(local_layout_shift)
                    >
                        {format!("{local_layout_shift:?}").to_case(Case::Camel)}
                    </button>
                }
            />
            <button
                on:click=move |_| {
                    // Move the reference twice on consecutive frames, with the
                    // second move landing after the IntersectionObserver measured
                    // the intermediate position but before its first callback.
                    let el = reference_ref.get().expect("Reference should exist.").unchecked_into::<HtmlElement>();
                    el.style().set_property("left", "40px").expect("Property should be set.");

                    let inner_closure: Closure<dyn FnMut()> = Closure::once(move || {
                        el.style().set_property("left", "280px").expect("Property should be set.");
                    });

                    let closure: Closure<dyn FnMut()> = Closure::once(move || {
                         window()
                        .expect("Window should exist.")
                        .request_animation_frame(inner_closure.as_ref().unchecked_ref())
                        .expect("Request animation frame should be successful.");
                    });

                     window()
                        .expect("Window should exist.")
                        .request_animation_frame(closure.as_ref().unchecked_ref())
                        .expect("Request animation frame should be successful.");
                }
                data-testid="layoutShift-moveTwice"
            >
                moveTwice
            </button>
        </div>

        <h2>animationFrame</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| {
                    view! {
                        <button
                            data-testid=format!("animationFrame-{}", value)
                            style:background-color=move || if options.get().animation_frame.unwrap() == value {
                               "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_options.set(options.get().animation_frame(value))
                        >
                            {format!("{value}")}
                        </button>
                    }
                }
            />
        </div>

        <h2>Reactive whileElementsMounted</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{value}")
                children=move |value| {
                    view! {
                        <button
                            data-testid=format!("whileElementsMounted-{}", value)
                            style:background-color=move || if while_elements_mounted.get() == value {
                                "black"
                            } else {
                                ""
                            }
                            on:click=move |_| set_while_elements_mounted.set(value)
                        >
                            {format!("{value}")}
                        </button>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn AutoUpdateRootResize() -> impl IntoView {
    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let (moved, set_moved) = signal(false);

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default().strategy(Strategy::Fixed),
    );

    type CleanupFn = Box<dyn Fn()>;
    let cleanup: StoredValue<Option<SendWrapper<CleanupFn>>> = StoredValue::new(None);

    Effect::new({
        let update = update.clone();

        move |_| {
            if let Some(reference) = reference_ref.get()
                && let Some(floating) = floating_ref.get()
            {
                if let Some(cleanup) = &*cleanup.read_value() {
                    cleanup();
                }

                // Match React test behaviour by moving the size change from style attributes to here.
                // The style attributes update after this effect, so `auto_update` would not use the correct size.
                let style = reference.unchecked_ref::<web_sys::HtmlElement>().style();

                style
                    .set_property(
                        "width",
                        if moved.get() {
                            "650px"
                        } else {
                            "calc(100vw - 220px"
                        },
                    )
                    .expect("Style should be updated.");

                cleanup.set_value(Some(SendWrapper::new(auto_update(
                    (&reference).into(),
                    Some(&floating),
                    (*update).clone(),
                    AutoUpdateOptions::default()
                        .ancestor_resize(false)
                        .element_resize(false)
                        .layout_shift(false),
                ))));
            }
        }
    });

    on_cleanup(move || {
        if let Some(cleanup) = &*cleanup.read_value() {
            cleanup();
        }
    });

    view! {
        <h1>AutoUpdate Root Resize</h1>
        <button
            node_ref=reference_ref
            data-testid="rootResize-reference"
            on:click=move |_| set_moved.set(true)
            style:position="relative"
            style:top="32px"
            style:left=move || if moved.get() { "650px" } else { "calc(100vw - 220px)" }
            style:width="75px"
            style:height="22px"
        >
            Toggle
        </button>
        <div
            node_ref=floating_ref
            class="floating"
            data-testid="rootResize-floating"
            style:position=move || format!("{:?}", strategy.get()).to_lowercase()
            style:top=move || format!("{}px", y.get())
            style:left=move || format!("{}px", x.get())
            style:width="75px"
            style:height="22px"
        >
            Floating
        </div>
    }
}
