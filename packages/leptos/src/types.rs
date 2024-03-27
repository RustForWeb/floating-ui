use floating_ui_dom::{Middleware, MiddlewareData, Placement, Strategy};
use leptos::{Attribute, IntoAttribute, MaybeProp, Signal};
use web_sys::{Element, Window};

/// Options for [`use_floating`][`crate::use_floating::use_floating`].
#[derive(Clone, Default)]
pub struct UseFloatingOptions {
    /// Represents the open/close state of the floating element.
    ///
    /// Defaults to `true`.
    pub open: MaybeProp<bool>,

    /// Where to place the floating element relative to the reference element.
    ///
    /// Defaults to [`Placement::Bottom`].
    pub placement: MaybeProp<Placement>,

    /// The strategy to use when positioning the floating element.
    ///
    /// Defaults to [`Strategy::Absolute`].
    pub strategy: MaybeProp<Strategy>,

    /// Array of middleware objects to modify the positioning or provide data for rendering.
    ///
    /// Defaults to an empty vector.
    pub middleware: MaybeProp<Vec<Box<dyn Middleware<Element, Window>>>>,

    ///  Whether to use `transform` for positioning instead of `top` and `left` in the `floatingStyles` object.
    ///
    /// Defaults to `true`.
    pub transform: MaybeProp<bool>,

    /// Callback to handle mounting/unmounting of the elements.
    ///
    ///Detauls to [`Option::None`].
    pub while_elements_mounted: MaybeProp<bool>, // TODO: type
}

impl UseFloatingOptions {
    /// Set [`Self::open`] option.
    pub fn open(mut self, value: MaybeProp<bool>) -> Self {
        self.open = value;
        self
    }

    /// Set [`Self::placement`] option.
    pub fn placement(mut self, value: MaybeProp<Placement>) -> Self {
        self.placement = value;
        self
    }

    /// Set [`Self::strategy`] option.
    pub fn strategy(mut self, value: MaybeProp<Strategy>) -> Self {
        self.strategy = value;
        self
    }

    /// Set [`Self::middleware`] option.
    pub fn middleware(
        mut self,
        value: MaybeProp<Vec<Box<dyn Middleware<Element, Window>>>>,
    ) -> Self {
        self.middleware = value;
        self
    }

    /// Set [`Self::transform`] option.
    pub fn transform(mut self, value: MaybeProp<bool>) -> Self {
        self.transform = value;
        self
    }

    /// Set [`Self::while_elements_mounted`] option.
    pub fn while_elements_mounted(mut self, value: MaybeProp<bool>) -> Self {
        self.while_elements_mounted = value;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FloatingStyles {
    pub position: Strategy,
    pub top: String,
    pub left: String,
    pub transform: Option<String>,
    pub will_change: Option<String>,
}

impl From<FloatingStyles> for String {
    fn from(value: FloatingStyles) -> Self {
        format!(
            "position: {}; top: {}; left: {};{}{}",
            match value.position {
                Strategy::Absolute => "absolute",
                Strategy::Fixed => "fixed",
            },
            value.top,
            value.left,
            value
                .transform
                .map_or("".into(), |transform| format!(" transform: {};", transform),),
            value.will_change.map_or("".into(), |will_change| format!(
                " will-change: {};",
                will_change
            ))
        )
    }
}

impl IntoAttribute for FloatingStyles {
    fn into_attribute(self) -> Attribute {
        let s: String = self.into();
        Attribute::String(s.into())
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
