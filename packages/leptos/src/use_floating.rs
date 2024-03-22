use floating_ui_core::Middleware;
use floating_ui_utils::{Placement, Strategy};
use leptos::{create_signal, leptos_dom::Element};

pub struct UseFloatingOptions {
    // TODO: should these be signals?
    // TODO: check these types and implement the rest
    strategy: Strategy,
    placement: Placement,
    middleware: Vec<Box<dyn Middleware<Element>>>,
}

pub struct UseFloatingData {
    pub x: f64,
    pub y: f64,
    pub strategy: Strategy,
    pub placement: Placement,
    // TODO
    pub middleware_data: bool,
    pub is_positioned: bool,
}

pub fn use_floating(options: UseFloatingOptions) {
    let (_data, _set_data) = create_signal(UseFloatingData {
        x: 0.0,
        y: 0.0,
        strategy: options.strategy,
        placement: options.placement,
        middleware_data: false,
        is_positioned: false,
    });

    let (_latest_middleware, _set_latest_middleware) = create_signal(options.middleware);

    // TODO: compare latest_middleware and options.middleware and update it

    let (_reference, _set_reference) = create_signal::<Option<Element>>(None);
    let (_floating, _set_floating) = create_signal::<Option<Element>>(None);

    // TODO: setReference and setFloating
}
