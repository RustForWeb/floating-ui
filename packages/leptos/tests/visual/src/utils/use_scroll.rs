use std::rc::Rc;

use floating_ui_leptos::{
    DetectOverflowOptions, MiddlewareVec, Padding, Placement, Shift, ShiftOptions, Strategy,
    UseFloatingOptions, UseFloatingReturn,
    dom::{OverflowAncestor, get_overflow_ancestors},
    use_floating,
};
use leptos::{html::Div, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::EventTarget;

pub struct UseScrollOptions {
    pub reference_ref: AnyNodeRef,
    pub floating_ref: AnyNodeRef,
    pub update: SendWrapper<Rc<dyn Fn()>>,
    pub rtl: MaybeProp<bool>,

    pub disable_ref_updates: Option<bool>,
}

pub struct UseScrollReturn {
    pub scroll_ref: NodeRef<Div>,
    pub indicator: SendWrapper<Rc<dyn Fn() -> AnyView>>,
    pub update_scroll: SendWrapper<Rc<dyn Fn()>>,
}

pub fn use_scroll(
    UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl,
        disable_ref_updates,
    }: UseScrollOptions,
) -> UseScrollReturn {
    let scroll_ref = NodeRef::<Div>::new();
    let indicator_floating_ref = AnyNodeRef::new();

    let middleware: MiddlewareVec = vec![Box::new(Shift::new(
        ShiftOptions::default()
            .detect_overflow(
                DetectOverflowOptions::default()
                    .alt_boundary(true)
                    .padding(Padding::All(10.0)),
            )
            .cross_axis(true),
    ))];

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update: indicator_update,
        ..
    } = use_floating(
        reference_ref,
        indicator_floating_ref,
        UseFloatingOptions::default()
            .strategy(Strategy::Fixed.into())
            .placement(Placement::Top.into())
            .middleware(SendWrapper::new(middleware).into()),
    );
    let indicator_update_rc = Rc::new(indicator_update);

    let (ancestors, set_ancestors) =
        signal::<SendWrapper<Vec<OverflowAncestor>>>(SendWrapper::new(vec![]));
    let (scroll, set_scroll) = signal::<Option<(i32, i32)>>(None);

    let local_update: Rc<Closure<dyn Fn()>> = Rc::new(Closure::new({
        let update = update.clone();
        let indicator_update = indicator_update_rc.clone();

        move || {
            if let Some(scroll) = scroll_ref.get_untracked() {
                set_scroll.set(Some((scroll.scroll_left(), scroll.scroll_top())));
            }

            update();
            indicator_update();
        }
    }));

    let effect_local_update = local_update.clone();
    let effect = move || {
        let reference = if disable_ref_updates.unwrap_or(false) {
            reference_ref.get_untracked()
        } else {
            reference_ref.get()
        };

        if let Some(reference) = reference {
            let mut ancestors = get_overflow_ancestors(&reference, vec![], true);

            let floating = if disable_ref_updates.unwrap_or(false) {
                floating_ref.get_untracked()
            } else {
                floating_ref.get()
            };

            if let Some(floating) = floating {
                ancestors.append(&mut get_overflow_ancestors(&floating, vec![], true));
            }

            for parent in &ancestors {
                let event_target: &EventTarget = match parent {
                    OverflowAncestor::Element(element) => element,
                    OverflowAncestor::Window(window) => window,
                };

                event_target
                    .add_event_listener_with_callback(
                        "scroll",
                        (*effect_local_update).as_ref().unchecked_ref(),
                    )
                    .expect("Scroll event listener should be added.");
            }

            set_ancestors.set(SendWrapper::new(ancestors));

            if let Some(scroll) = scroll_ref.get() {
                let x = scroll.scroll_width() / 2 - scroll.offset_width() / 2;
                let y = scroll.scroll_height() / 2 - scroll.offset_height() / 2;
                scroll.set_scroll_top(y);
                scroll.set_scroll_left(match rtl.get() {
                    Some(true) => -x,
                    _ => x,
                });
            }

            update();
        }
    };
    let effect_rc = Rc::new(effect);

    let effect_effect = effect_rc.clone();
    Effect::new(move |_| {
        effect_effect();
    });

    let cleanup = SendWrapper::new(move || {
        for ancestor in ancestors.get().iter() {
            let event_target = match ancestor {
                OverflowAncestor::Element(element) => {
                    element.unchecked_ref::<web_sys::EventTarget>()
                }
                OverflowAncestor::Window(window) => window.unchecked_ref::<web_sys::EventTarget>(),
            };

            event_target
                .remove_event_listener_with_callback(
                    "scroll",
                    (*local_update).as_ref().unchecked_ref(),
                )
                .expect("Scroll event listener should be removed.");
        }
    });

    on_cleanup(move || {
        cleanup();
    });

    let indicator = move || {
        view! {
            <div
                node_ref=indicator_floating_ref
                class="scroll-indicator"
                style:position=move || format!("{:?}", strategy.get())
                style:top=move || format!("{}px", y.get())
                style:left=move || format!("{}px", x.get())
            >
                {move || scroll.get().map_or("x: null, y: null".to_owned(), |scroll| format!("x: {}, y: {}", scroll.0, scroll.1))}
            </div>
        }.into_any()
    };

    let update_scroll = move || {
        effect_rc();
    };

    UseScrollReturn {
        scroll_ref,
        indicator: SendWrapper::new(Rc::new(indicator)),
        update_scroll: SendWrapper::new(Rc::new(update_scroll)),
    }
}
