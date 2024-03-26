use std::ops::Deref;

use floating_ui_dom::{
    compute_position, ComputePositionConfig, MiddlewareData, Placement, Strategy,
};
use leptos::{
    create_effect, create_memo, create_render_effect, create_rw_signal, create_signal,
    html::ElementDescriptor, watch, MaybeSignal, NodeRef, Signal, SignalGet, SignalUpdate,
};
use log::info;

use crate::{
    types::{FloatingStyles, UseFloatingOptions, UseFloatingReturn},
    utils::{get_dpr::get_dpr, round_by_dpr::round_by_dpr},
};

pub fn use_floating<Reference, Floating, ReferenceEl, FloatingEl>(
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
    let placement_option = move || options.placement.get().unwrap_or(Placement::Bottom);
    let strategy_option = move || options.strategy.get().unwrap_or(Strategy::Absolute);
    let middleware_option = move || options.middleware.get();
    let transform_option = move || options.transform.get().unwrap_or(true);

    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);
    let (strategy, set_strategy) = create_signal(strategy_option());
    let (placement, set_placement) = create_signal(placement_option());
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
        if let Some(reference_element) = reference.get() {
            if let Some(floating_element) = floating.get() {
                let config = ComputePositionConfig {
                    placement: Some(placement_option()),
                    strategy: Some(strategy_option()),
                    middleware: middleware_option(),
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

    let cleanup = move || {
        // TODO
    };

    let attach = move || {
        cleanup();

        // TODO: the rest of the function
        update();
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

    // _ = watch(
    //     move || (middleware_option(), placement_option(), strategy_option()),
    //     move |_, _, _| {
    //         info!("watch update");
    //         update();
    //     },
    //     false,
    // );

    _ = watch(
        open_option,
        move |_, _, _| {
            info!("watch reset");
            reset();
        },
        false,
    );

    UseFloatingReturn {
        x: x.into(),
        y: y.into(),
        placement: placement.into(),
        strategy: strategy.into(),
        middleware_data: middleware_data.into(),
        is_positioned: is_positioned.into(),
        floating_styles: floating_styles.into(),
        update: false, // TODO
    }
}
