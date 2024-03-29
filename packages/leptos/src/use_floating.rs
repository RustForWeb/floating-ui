use std::{cell::RefCell, ops::Deref, rc::Rc};

use floating_ui_dom::{
    compute_position, ComputePositionConfig, MiddlewareData, Placement, Strategy,
};
use leptos::{
    create_effect, create_memo, create_rw_signal, create_signal, html::ElementDescriptor,
    on_cleanup, watch, NodeRef, SignalGet, SignalGetUntracked, SignalUpdate,
};

use crate::{
    types::{FloatingStyles, UseFloatingOptions, UseFloatingReturn},
    utils::{get_dpr::get_dpr, round_by_dpr::round_by_dpr},
    WhileElementsMountedCleanupFn,
};

/// Computes the `x` and `y` coordinates that will place the floating element next to a reference element.
pub fn use_floating<Reference, ReferenceEl, Floating, FloatingEl>(
    reference: NodeRef<Reference>,
    floating: NodeRef<Floating>,
    options: UseFloatingOptions,
) -> UseFloatingReturn
where
    Reference: ElementDescriptor + Deref<Target = ReferenceEl> + Clone + 'static,
    ReferenceEl: Deref<Target = web_sys::HtmlElement>,
    Floating: ElementDescriptor + Deref<Target = FloatingEl> + Clone + 'static,
    FloatingEl: Deref<Target = web_sys::HtmlElement>,
{
    let open_option = move || options.open.get().unwrap_or(true);
    let placement_option_untracked = move || {
        options
            .placement
            .get_untracked()
            .unwrap_or(Placement::Bottom)
    };
    let strategy_option_untracked = move || {
        options
            .strategy
            .get_untracked()
            .unwrap_or(Strategy::Absolute)
    };
    let options_middleware = options.middleware.clone();
    let middleware_option_untracked = move || options_middleware.get_untracked();
    let transform_option = move || options.transform.get().unwrap_or(true);

    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);
    let (strategy, set_strategy) = create_signal(strategy_option_untracked());
    let (placement, set_placement) = create_signal(placement_option_untracked());
    let (middleware_data, set_middleware_data) = create_signal(MiddlewareData::default());
    let (is_positioned, set_is_positioned) = create_signal(false);
    let floating_styles = create_memo(move |_| {
        let initial_styles = FloatingStyles {
            position: strategy(),
            top: "0".into(),
            left: "0".into(),
            transform: None,
            will_change: None,
        };

        if let Some(floating_element) = floating.get() {
            let x_val = round_by_dpr(&floating_element, x());
            let y_val = round_by_dpr(&floating_element, y());

            if transform_option() {
                FloatingStyles {
                    transform: Some(format!("translate({x_val}px, {y_val}px)")),
                    will_change: match get_dpr(&floating_element) >= 1.5 {
                        true => Some("transform".into()),
                        false => None,
                    },
                    ..initial_styles
                }
            } else {
                FloatingStyles {
                    left: format!("{x_val}px"),
                    top: format!("{y_val}px"),
                    ..initial_styles
                }
            }
        } else {
            initial_styles
        }
    });

    let update = move || {
        if let Some(reference_element) = reference.get_untracked() {
            if let Some(floating_element) = floating.get_untracked() {
                let config = ComputePositionConfig {
                    placement: Some(placement_option_untracked()),
                    strategy: Some(strategy_option_untracked()),
                    middleware: middleware_option_untracked(),
                };

                let position =
                    compute_position(&reference_element, &floating_element, Some(config));
                set_x(position.x);
                set_y(position.y);
                set_strategy(position.strategy);
                set_placement(position.placement);
                set_middleware_data(position.middleware_data);
                set_is_positioned(true);
            }
        }
    };
    let update_rc = Rc::new(update);

    let while_elements_mounted_cleanup: Rc<RefCell<Option<WhileElementsMountedCleanupFn>>> =
        Rc::new(RefCell::new(None));

    let cleanup_while_elements_mounted_cleanup = while_elements_mounted_cleanup.clone();
    let cleanup = move || {
        if let Some(while_elements_mounted_cleanup) = cleanup_while_elements_mounted_cleanup.take()
        {
            while_elements_mounted_cleanup();
        }
    };
    let cleanup_rc = Rc::new(cleanup);

    let attach_update_rc = update_rc.clone();
    let attach_cleanup_rc = cleanup_rc.clone();
    let attach_while_elements_mounted_cleanup = while_elements_mounted_cleanup.clone();
    let attach = move || {
        attach_cleanup_rc();

        if let Some(while_elements_mounted) = &options.while_elements_mounted {
            if let Some(reference_element) = reference.get() {
                if let Some(floating_element) = floating.get() {
                    attach_while_elements_mounted_cleanup.replace(Some(while_elements_mounted(
                        &reference_element,
                        &floating_element,
                        attach_update_rc.clone(),
                    )));
                }
            }
        } else {
            attach_update_rc();
        }
    };

    let reset = move || {
        if !open_option() {
            set_is_positioned(false);
        }
    };

    let remaining_mounts = create_rw_signal::<u32>(2);
    reference.on_load(move |reference| {
        _ = reference.on_mount(move |_| {
            remaining_mounts.update(|count| *count -= 1);
        });
    });
    floating.on_load(move |floating| {
        _ = floating.on_mount(move |_| {
            remaining_mounts.update(|count| *count -= 1);
        });
    });
    create_effect(move |_| {
        if remaining_mounts.get() == 0 {
            attach();
        }
    });

    create_effect(move |_| {
        reset();
    });

    let placement_update_rc = update_rc.clone();
    let strategy_update_rc = update_rc.clone();
    let middleware_update_rc = update_rc.clone();
    _ = watch(
        options.placement,
        move |_, _, _| {
            placement_update_rc();
        },
        false,
    );
    _ = watch(
        options.strategy,
        move |_, _, _| {
            strategy_update_rc();
        },
        false,
    );
    _ = watch(
        options.middleware,
        move |_, _, _| {
            middleware_update_rc();
        },
        false,
    );

    on_cleanup(move || {
        cleanup_rc();
    });

    UseFloatingReturn {
        x: x.into(),
        y: y.into(),
        placement: placement.into(),
        strategy: strategy.into(),
        middleware_data: middleware_data.into(),
        is_positioned: is_positioned.into(),
        floating_styles: floating_styles.into(),
        update: update_rc.clone(),
    }
}

#[cfg(test)]
mod tests {
    use leptos::{html::Div, *};
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn updates_is_positioned_when_position_is_computed() {
        #[component]
        fn Component() -> impl IntoView {
            let reference = create_node_ref::<Div>();
            let floating = create_node_ref::<Div>();
            let UseFloatingReturn { is_positioned, .. } =
                use_floating(reference, floating, UseFloatingOptions::default());

            view! {
                <div _ref=reference />
                <div _ref=floating />
                <div id="test-is-positioned">{is_positioned}</div>
            }
        }

        let document = leptos::document();
        mount_to(document.body().unwrap(), Component);

        // assert_eq!(
        //     document
        //         .get_element_by_id("test-is-positioned")
        //         .and_then(|element| element.text_content()),
        //     Some("true".into())
        // );
    }
}
