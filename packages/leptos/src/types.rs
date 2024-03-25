use floating_ui_dom::{Middleware, MiddlewareData, Placement, Strategy};
use leptos::{Attribute, IntoAttribute, Signal};
use web_sys::{Element, Window};

/// Options for [`use_floating`].
#[derive(Clone, Default)]
pub struct UseFloatingOptions<'a> {
    /// Represents the open/close state of the floating element.
    ///
    /// Defaults to `true`.
    pub open: Option<bool>,

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

    ///  Whether to use `transform` for positioning instead of `top` and `left` in the `floatingStyles` object.
    ///
    /// Defaults to `true`.
    pub transform: Option<bool>,

    /// Callback to handle mounting/unmounting of the elements.
    ///
    ///Detauls to [`Option::None`].
    pub while_elements_mounted: Option<bool>, // TODO: type
}

#[derive(Clone, Debug, PartialEq)]
pub struct FloatingStyles {
    pub position: Strategy,
    pub top: String,
    pub left: String,
    pub transform: Option<String>,
    pub will_change: Option<String>,
}

impl IntoAttribute for FloatingStyles {
    fn into_attribute(self) -> Attribute {
        Attribute::String(
            format!(
                "position: {:?}; top: {}; left: {};{}{}",
                self.position,
                self.top,
                self.left,
                self.transform
                    .map_or("".into(), |transform| format!(" transform: {};", transform),),
                self.will_change.map_or("".into(), |will_change| format!(
                    " will-change: {};",
                    will_change
                ))
            )
            .into(),
        )
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

#[derive(Debug)]
pub struct UseFloatingReturn {
    /// The x-coord of the floating element.
    pub x: Signal<f64>,

    /// The y-coord of the floating element.
    pub y: Signal<f64>,

    /// The stateful placement, which can be different from the initial `placement` passed as options.
    pub placement: Signal<Placement>,

    /// The strategy to use when positioning the floating element.
    pub strategy: Signal<Strategy>,

    /// Additional data from middleware.
    pub middleware_data: Signal<MiddlewareData>,

    /// Indicates if the floating element has been positioned.
    pub is_positioned: Signal<bool>,

    /// CSS styles to apply to the floating element to position it.
    pub floating_styles: Signal<FloatingStyles>,

    /// The function to update floating position manually.
    pub update: bool, // TODO: type
}
