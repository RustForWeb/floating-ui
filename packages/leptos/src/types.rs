use std::rc::Rc;

use floating_ui_dom::{
    auto_update, AutoUpdateOptions, Middleware, MiddlewareData, Placement, Strategy,
};
use leptos::{Attribute, IntoAttribute, MaybeProp, MaybeSignal, Signal, SignalGet};
use web_sys::{Element, Window};

pub type WhileElementsMountedFn =
    dyn Fn(&Element, &Element, Rc<dyn Fn()>) -> WhileElementsMountedCleanupFn;

pub type WhileElementsMountedCleanupFn = Box<dyn Fn()>;

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
    /// Defaults to [`Option::None`].
    pub while_elements_mounted: MaybeProp<Rc<WhileElementsMountedFn>>,
}

impl UseFloatingOptions {
    /// Set `open` option.
    pub fn open(mut self, value: MaybeProp<bool>) -> Self {
        self.open = value;
        self
    }

    /// Set `placement` option.
    pub fn placement(mut self, value: MaybeProp<Placement>) -> Self {
        self.placement = value;
        self
    }

    /// Set `strategy` option.
    pub fn strategy(mut self, value: MaybeProp<Strategy>) -> Self {
        self.strategy = value;
        self
    }

    /// Set `middleware` option.
    pub fn middleware(
        mut self,
        value: MaybeProp<Vec<Box<dyn Middleware<Element, Window>>>>,
    ) -> Self {
        self.middleware = value;
        self
    }

    /// Set `transform` option.
    pub fn transform(mut self, value: MaybeProp<bool>) -> Self {
        self.transform = value;
        self
    }

    /// Set `while_elements_mounted` option.
    pub fn while_elements_mounted(mut self, value: MaybeProp<Rc<WhileElementsMountedFn>>) -> Self {
        self.while_elements_mounted = value;
        self
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with [`AutoUpdateOptions::default`].
    pub fn while_elements_mounted_auto_update(self) -> Self {
        let auto_update_rc: Rc<WhileElementsMountedFn> = Rc::new(|reference, floating, update| {
            auto_update(
                reference.into(),
                floating,
                update,
                AutoUpdateOptions::default(),
            )
        });
        self.while_elements_mounted(auto_update_rc.into())
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with [`AutoUpdateOptions::default`].
    pub fn while_elements_mounted_auto_update_enabled(self, enabled: MaybeSignal<bool>) -> Self {
        let auto_update_rc: Rc<WhileElementsMountedFn> = Rc::new(|reference, floating, update| {
            auto_update(
                reference.into(),
                floating,
                update,
                AutoUpdateOptions::default(),
            )
        });
        self.while_elements_mounted(MaybeProp::derive(move || {
            if enabled.get() {
                Some(auto_update_rc.clone())
            } else {
                None
            }
        }))
    }
}

/// CSS styles to apply to the floating element to position it.
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

/// Return of [`use_floating`][crate::use_floating::use_floating].
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
    pub update: Rc<dyn Fn()>,
}
