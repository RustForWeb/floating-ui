use std::rc::Rc;

use floating_ui_leptos::{
    dom::{get_overflow_ancestors, OverflowAncestor},
    use_floating, DetectOverflowOptions, IntoReference, MiddlewareVec, Padding, Placement, Shift,
    ShiftOptions, Strategy, UseFloatingOptions, UseFloatingReturn,
};
use leptos::{html::Div, *};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::EventTarget;

pub struct UseScrollOptions {
    pub reference_ref: NodeRef<Div>,
    pub floating_ref: NodeRef<Div>,
    pub update: Rc<dyn Fn()>,
    pub rtl: MaybeProp<bool>,

    pub disable_ref_updates: Option<bool>,
}

pub struct UseScrollReturn {
    pub scroll_ref: NodeRef<Div>,
    pub indicator: Rc<dyn Fn() -> HtmlElement<Div>>,
    pub update_scroll: Rc<dyn Fn()>,
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
    let scroll_ref = create_node_ref::<Div>();
    let indicator_floating_ref = create_node_ref::<Div>();

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
        reference_ref.into_reference(),
        indicator_floating_ref,
        UseFloatingOptions::default()
            .strategy(Strategy::Fixed.into())
            .placement(Placement::Top.into())
            .middleware(middleware.into()),
    );
    let indicator_update_rc = Rc::new(indicator_update);

    let (ancestors, set_ancestors) = create_signal::<Vec<OverflowAncestor>>(vec![]);
    let (scroll, set_scroll) = create_signal::<Option<(i32, i32)>>(None);

    let local_update_update = update.clone();
    let local_update_indicator_update = indicator_update_rc.clone();
    let local_update: Rc<Closure<dyn Fn()>> = Rc::new(Closure::new(move || {
        if let Some(scroll) = scroll_ref.get_untracked() {
            set_scroll(Some((scroll.scroll_left(), scroll.scroll_top())));
        }

        local_update_update();
        local_update_indicator_update();
    }));

    let effect_local_update = local_update.clone();
    let effect = move || {
        let reference = match disable_ref_updates.unwrap_or(false) {
            true => reference_ref.get_untracked(),
            false => reference_ref.get(),
        };

        if let Some(reference) = reference {
            let mut ancestors = get_overflow_ancestors(&reference, vec![], true);

            let floating = match disable_ref_updates.unwrap_or(false) {
                true => floating_ref.get_untracked(),
                false => floating_ref.get(),
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

            set_ancestors(ancestors);

            if let Some(scroll) = scroll_ref() {
                let x = scroll.scroll_width() / 2 - scroll.offset_width() / 2;
                let y = scroll.scroll_height() / 2 - scroll.offset_height() / 2;
                scroll.set_scroll_top(y);
                scroll.set_scroll_left(match rtl() {
                    Some(true) => -x,
                    _ => x,
                });
            }

            update();
        }
    };
    let effect_rc = Rc::new(effect);

    let effect_effect = effect_rc.clone();
    create_effect(move |_| {
        effect_effect();
    });

    on_cleanup(move || {
        for ancestor in ancestors.get() {
            let event_target: EventTarget = match ancestor {
                OverflowAncestor::Element(element) => element.into(),
                OverflowAncestor::Window(window) => window.into(),
            };

            event_target
                .remove_event_listener_with_callback(
                    "scroll",
                    (*local_update).as_ref().unchecked_ref(),
                )
                .expect("Scroll event listener should be removed.");
        }
    });

    let indicator = move || {
        view! {
            <div
                _ref=indicator_floating_ref
                class="scroll-indicator"
                style:position=move || format!("{:?}", strategy())
                style:top=move || format!("{}px", y())
                style:left=move || format!("{}px", x())
            >
                {move || scroll().map_or("x: null, y: null".into(), |scroll| format!("x: {}, y: {}", scroll.0, scroll.1))}
            </div>
        }
    };

    let update_scroll = move || {
        effect_rc();
    };

    UseScrollReturn {
        scroll_ref,
        indicator: Rc::new(indicator),
        update_scroll: Rc::new(update_scroll),
    }
}
