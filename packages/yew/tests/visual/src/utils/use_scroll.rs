use std::rc::Rc;

use floating_ui_yew::{
    dom::{get_overflow_ancestors, OverflowAncestor},
    use_floating, DetectOverflowOptions, MiddlewareVec, Padding, Placement, Shift, ShiftOptions,
    Strategy, UseFloatingOptions, UseFloatingReturn,
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::EventTarget;
use yew::prelude::*;

pub struct UseScrollOptions {
    pub reference_ref: NodeRef,
    pub floating_ref: NodeRef,
    pub update: Callback<()>,
    pub rtl: Option<bool>,
}

pub struct UseScrollReturn {
    pub scroll_ref: NodeRef,
    // TODO: remove
    #[allow(unused)]
    pub indicator: Rc<dyn Fn() -> Html>,
    pub update_scroll: Callback<(), Vec<OverflowAncestor>>,
}

#[hook]
pub fn use_scroll(options: UseScrollOptions) -> UseScrollReturn {
    let UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl,
    } = options;

    let scroll_ref = use_node_ref();
    let indicator_floating_ref = use_node_ref();

    let middleware = use_memo((), |_| {
        let middleware: MiddlewareVec = vec![Box::new(Shift::new(
            ShiftOptions::default()
                .detect_overflow(
                    DetectOverflowOptions::default()
                        .alt_boundary(true)
                        .padding(Padding::All(10.0)),
                )
                .cross_axis(true),
        ))];

        middleware
    });

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update: indicator_update,
        ..
    } = use_floating(
        reference_ref.clone().into(),
        indicator_floating_ref.clone(),
        UseFloatingOptions::default()
            .strategy(Strategy::Fixed)
            .placement(Placement::Top)
            .middleware((*middleware).clone()),
    );

    let scroll: UseStateHandle<Option<(i32, i32)>> = use_state_eq(|| None);

    let local_update: Rc<Closure<dyn Fn()>> = Rc::new(Closure::new({
        let scroll_ref = scroll_ref.clone();
        let scroll = scroll.clone();
        let update = update.clone();
        let indicator_update = indicator_update.clone();

        move || {
            if let Some(scroll_node) = scroll_ref.get() {
                let scroll_element: &web_sys::Element = scroll_node
                    .dyn_ref()
                    .expect("Scroll element should be an Element.");

                scroll.set(Some((
                    scroll_element.scroll_left(),
                    scroll_element.scroll_top(),
                )));
            }

            update.emit(());
            indicator_update.emit(());
        }
    }));

    let update_scroll = use_callback(
        (
            reference_ref.clone(),
            floating_ref.clone(),
            scroll_ref.clone(),
        ),
        {
            let update = update.clone();
            let local_update = local_update.clone();

            move |_, (reference_ref, floating_ref, scroll_ref)| {
                let mut parents = vec![];

                if let Some(reference) = reference_ref.get() {
                    parents.append(&mut get_overflow_ancestors(&reference, vec![], true));

                    if let Some(floating) = floating_ref.get() {
                        parents.append(&mut get_overflow_ancestors(&floating, vec![], true));
                    }

                    for parent in &parents {
                        let event_target: &EventTarget = match parent {
                            OverflowAncestor::Element(element) => element,
                            OverflowAncestor::Window(window) => window,
                        };

                        event_target
                            .add_event_listener_with_callback(
                                "scroll",
                                (*local_update).as_ref().unchecked_ref(),
                            )
                            .expect("Scroll event listener should be added.");
                    }

                    if let Some(scroll_node) = scroll_ref.get() {
                        let scroll_element: &web_sys::HtmlElement = scroll_node
                            .dyn_ref()
                            .expect("Scroll element should be an HtmlElement.");

                        let x =
                            scroll_element.scroll_width() / 2 - scroll_element.offset_width() / 2;
                        let y =
                            scroll_element.scroll_height() / 2 - scroll_element.offset_height() / 2;
                        scroll_element.set_scroll_top(y);
                        scroll_element.set_scroll_left(match rtl {
                            Some(true) => -x,
                            _ => x,
                        });
                    }

                    update.emit(());
                }

                parents
            }
        },
    );

    use_effect_with(
        (
            reference_ref.clone(),
            floating_ref.clone(),
            scroll_ref.clone(),
            rtl,
        ),
        {
            let local_update = local_update.clone();
            let update_scroll = update_scroll.clone();

            move |_| {
                let parents = update_scroll.emit(());

                move || {
                    for parent in parents {
                        let event_target: EventTarget = match parent {
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
                }
            }
        },
    );

    let indicator = {
        let indicator_floating_ref = indicator_floating_ref.clone();
        let scroll = scroll.clone();

        Rc::new(move || {
            html! {
                <div
                    ref={indicator_floating_ref.clone()}
                    class="scroll-indicator"
                    style={format!(
                        "position: {}; top: {}px; left: {}px;",
                        format!("{:?}", *strategy).to_lowercase(),
                        *y,
                        *x
                    )}
                >
                    {scroll.map_or("x: null, y: null".into(), |scroll| format!("x: {}, y: {}", scroll.0, scroll.1))}
                </div>
            }
        })
    };

    UseScrollReturn {
        scroll_ref,
        indicator,
        update_scroll,
    }
}
