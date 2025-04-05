use convert_case::{Case, Casing};
use floating_ui_leptos::{
    AutoUpdateOptions, Strategy, UseFloatingOptions, UseFloatingReturn, auto_update, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

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
            .strategy(Strategy::Fixed.into())
            .while_elements_mounted_auto_update_with_enabled(while_elements_mounted.into()),
    );

    type CleanupFn = Box<dyn Fn()>;
    let cleanup: StoredValue<Option<SendWrapper<CleanupFn>>> = StoredValue::new(None);

    Effect::new({
        let update = update.clone();

        move |_| {
            if let Some(reference) = reference_ref.get() {
                if let Some(floating) = floating_ref.get() {
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
                        &floating,
                        (*update).clone(),
                        options
                            .get()
                            .layout_shift(layout_shift.get() != LayoutShift::None),
                    ))));
                }
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
                key=|value| format!("{}", value)
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
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>

        <h2>ancestorResize</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
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
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>

        <h2>elementResize</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
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
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>

        <h2>layoutShift</h2>
        <div class="controls">
            <For
                each=|| ALL_LAYOUT_SHIFTS
                key=|local_layout_shift| format!("{:?}", local_layout_shift)
                children=move |local_layout_shift| view! {
                    <button
                        data-testid=move || format!("layoutShift-{}", format!("{:?}", local_layout_shift).to_case(Case::Camel))
                        style:background-color=move || if layout_shift.get() == local_layout_shift {
                            "black"
                        } else {
                            ""
                        }
                        on:click=move |_| set_layout_shift.set(local_layout_shift)
                    >
                        {format!("{:?}", local_layout_shift).to_case(Case::Camel)}
                    </button>
                }
            />
        </div>

        <h2>animationFrame</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
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
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>

        <h2>Reactive whileElementsMounted</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
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
                            {format!("{}", value)}
                        </button>
                    }
                }
            />
        </div>
    }
}
