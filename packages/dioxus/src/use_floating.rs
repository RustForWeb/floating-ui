use std::{cell::RefCell, rc::Rc};

use dioxus::{core::use_drop, prelude::*, web::WebEventExt};
use floating_ui_dom::{
    ComputePositionConfig, MiddlewareData, Placement, Strategy, compute_position,
};

use crate::{
    FloatingStyles, UseFloatingOptions, UseFloatingReturn, WhileElementsMountedCleanupFn,
    utils::{get_dpr::get_dpr, round_by_dpr::round_by_dpr},
};

/// Computes the `x` and `y` coordinates that will place the floating element next to a reference element.
pub fn use_floating(
    reference: Signal<Option<Rc<MountedData>>>,
    floating: Signal<Option<Rc<MountedData>>>,
    options: UseFloatingOptions,
) -> UseFloatingReturn {
    let open_option = use_memo(move || options.open.unwrap_or(true));
    let placement_option = use_memo(move || options.placement.unwrap_or(Placement::Bottom));
    let strategy_option = use_memo(move || options.strategy.unwrap_or(Strategy::Absolute));
    let middleware_option = use_memo(move || options.middleware.clone().unwrap_or_default());
    let transform_option = use_memo(move || options.transform.unwrap_or(true));
    let while_elements_mounted_option = options.while_elements_mounted;

    let mut x = use_signal(|| 0.0);
    let mut y = use_signal(|| 0.0);
    #[expect(clippy::redundant_closure)]
    let mut strategy = use_signal(|| strategy_option());
    #[expect(clippy::redundant_closure)]
    let mut placement = use_signal(|| placement_option());
    let mut middleware_data = use_signal(MiddlewareData::default);
    let mut is_positioned = use_signal(|| false);
    let floating_styles = use_memo(move || {
        let initial_styles = FloatingStyles {
            position: strategy(),
            top: "0".to_owned(),
            left: "0".to_owned(),
            transform: None,
            will_change: None,
        };

        match floating().map(|floating| floating.as_web_event()) {
            Some(floating_element) => {
                let x_val = round_by_dpr(&floating_element, x());
                let y_val = round_by_dpr(&floating_element, y());

                if transform_option() {
                    FloatingStyles {
                        transform: Some(format!("translate({x_val}px, {y_val}px)")),
                        will_change: (get_dpr(&floating_element) >= 1.5)
                            .then_some("transform".to_owned()),
                        ..initial_styles
                    }
                } else {
                    FloatingStyles {
                        left: format!("{x_val}px"),
                        top: format!("{y_val}px"),
                        ..initial_styles
                    }
                }
            }
            _ => initial_styles,
        }
    });

    let update = use_callback(move |_| {
        if let Some(reference_element) = reference().map(|reference| reference.as_web_event())
            && let Some(floating_element) = floating().map(|floating| floating.as_web_event())
        {
            let config = ComputePositionConfig {
                placement: Some(placement_option()),
                strategy: Some(strategy_option()),
                middleware: Some(middleware_option()),
            };

            let open = open_option();

            let position = compute_position((&reference_element).into(), &floating_element, config);
            x.set(position.x);
            y.set(position.y);
            strategy.set(position.strategy);
            placement.set(position.placement);
            middleware_data.set(position.middleware_data);
            // The floating element's position may be recomputed while it's closed
            // but still mounted (such as when transitioning out). To ensure
            // `is_positioned` will be `false` initially on the next open,
            // avoid setting it to `true` when `open === false` (must be specified).
            is_positioned.set(open);
        }
    });

    let while_elements_mounted_cleanup = use_hook::<
        Rc<RefCell<Option<Rc<WhileElementsMountedCleanupFn>>>>,
    >(|| Rc::new(RefCell::new(None)));

    let cleanup = use_callback({
        let while_elements_mounted_cleanup = while_elements_mounted_cleanup.clone();

        move |_| {
            if let Some(while_elements_mounted_cleanup) = while_elements_mounted_cleanup.take() {
                while_elements_mounted_cleanup();
            }
        }
    });

    let attach = use_callback(move |_| {
        cleanup.call(());

        if let Some(while_elements_mounted) = &while_elements_mounted_option {
            if let Some(reference_element) = reference().map(|reference| reference.as_web_event())
                && let Some(floating_element) = floating().map(|floating| floating.as_web_event())
            {
                while_elements_mounted_cleanup.replace(Some(Rc::new((*while_elements_mounted)(
                    (&reference_element).into(),
                    &floating_element,
                    Rc::new(move || {
                        update.call(());
                    }),
                ))));
            }
        } else {
            update.call(());
        }
    });

    let reset = use_callback(move |_| {
        if open_option() {
            is_positioned.set(false);
        }
    });

    use_effect(move || {
        _ = open_option();
        _ = placement_option();
        _ = strategy_option();
        _ = middleware_option();

        update.call(());
    });

    use_effect(move || {
        _ = reference();
        _ = floating();

        attach(());
    });

    use_effect(move || {
        _ = open_option();

        reset.call(());
    });

    use_drop(move || {
        cleanup.call(());
    });

    UseFloatingReturn {
        x,
        y,
        placement,
        strategy,
        middleware_data,
        is_positioned,
        floating_styles,
        update,
    }
}
