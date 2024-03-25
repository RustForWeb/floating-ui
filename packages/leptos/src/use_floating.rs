use floating_ui_dom::{Middleware, MiddlewareData, Placement, Strategy};
use leptos::{create_signal, MaybeSignal};
use web_sys::{Element, Window};

/// Options for [`use_floating`].
#[derive(Clone, Default)]
pub struct UseFloatingOptions<'a> {
    /// Where to place the floating element relative to the reference element.
    ///
    /// Defaults to [`Placement::Bottom`].
    pub placement: Option<Placement>,

    /// The strategy to use when positioning the floating element.
    ///
    /// Defaults to [`Strategy::Absolute`].
    pub strategy: Option<Strategy>,

    /// Array of middleware objects to modify the positioning or provide data for rendering.
    ///
    /// Defaults to an empty vector.
    pub middleware: Option<Vec<&'a dyn Middleware<Element, Window>>>,
}

/// Data stored by [`use_floating`].
pub struct UseFloatingData {
    pub x: f64,
    pub y: f64,
    pub strategy: Strategy,
    pub placement: Placement,
    pub middleware_data: MiddlewareData,
    pub is_positioned: bool,
}

pub fn use_floating(options: MaybeSignal<UseFloatingOptions>) {
    // let placement = create_memo(move |_| {
    //     options.with(|options| options.placement.unwrap_or(Placement::Bottom))
    // });

    // let strategy = create_memo(move |_| {
    //     options.with(|options| options.strategy.unwrap_or(Strategy::Absolute))
    // });

    let (_data, _set_data) = create_signal(UseFloatingData {
        x: 0.0,
        y: 0.0,
        strategy: options().strategy.unwrap_or(Strategy::Absolute),
        placement: options().placement.unwrap_or(Placement::Bottom),
        middleware_data: MiddlewareData::default(),
        is_positioned: false,
    });

    // let (_latest_middleware, _set_latest_middleware) = create_signal(options.middleware);

    // // TODO: compare latest_middleware and options.middleware and update it

    // let (_reference, _set_reference) = create_signal::<Option<Element>>(None);
    // let (_floating, _set_floating) = create_signal::<Option<Element>>(None);

    // // TODO: setReference and setFloating
}
