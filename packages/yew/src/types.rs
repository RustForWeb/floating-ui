use std::rc::Rc;

use floating_ui_dom::{
    auto_update, AutoUpdateOptions, ElementOrVirtual, Middleware, MiddlewareData, Placement,
    Strategy,
};
use web_sys::{Element, Window};
use yew::{Callback, UseStateHandle};

pub type WhileElementsMountedFn =
    dyn Fn(ElementOrVirtual, &Element, Rc<dyn Fn()>) -> WhileElementsMountedCleanupFn;

pub type WhileElementsMountedCleanupFn = Box<dyn Fn()>;

/// Options for [`use_floating`][`crate::use_floating::use_floating`].
#[derive(Clone, Default)]
pub struct UseFloatingOptions {
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
    pub middleware: Option<Vec<Box<dyn Middleware<Element, Window>>>>,

    ///  Whether to use `transform` for positioning instead of `top` and `left` in the `floatingStyles` object.
    ///
    /// Defaults to `true`.
    pub transform: Option<bool>,

    /// Callback to handle mounting/unmounting of the elements.
    ///
    /// Defaults to [`Option::None`].
    pub while_elements_mounted: Option<Rc<WhileElementsMountedFn>>,
}

impl UseFloatingOptions {
    /// Set `open` option.
    pub fn open(mut self, value: bool) -> Self {
        self.open = Some(value);
        self
    }

    /// Set `placement` option.
    pub fn placement(mut self, value: Placement) -> Self {
        self.placement = Some(value);
        self
    }

    /// Set `strategy` option.
    pub fn strategy(mut self, value: Strategy) -> Self {
        self.strategy = Some(value);
        self
    }

    /// Set `middleware` option.
    pub fn middleware(mut self, value: Vec<Box<dyn Middleware<Element, Window>>>) -> Self {
        self.middleware = Some(value);
        self
    }

    /// Set `transform` option.
    pub fn transform(mut self, value: bool) -> Self {
        self.transform = Some(value);
        self
    }

    /// Set `while_elements_mounted` option.
    pub fn while_elements_mounted(mut self, value: Rc<WhileElementsMountedFn>) -> Self {
        self.while_elements_mounted = Some(value);
        self
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with [`AutoUpdateOptions::default`].
    pub fn while_elements_mounted_auto_update(self) -> Self {
        let auto_update_rc: Rc<WhileElementsMountedFn> = Rc::new(|reference, floating, update| {
            auto_update(reference, floating, update, AutoUpdateOptions::default())
        });

        self.while_elements_mounted(auto_update_rc)
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with [`AutoUpdateOptions::default`] when `enabled` is `true`.
    pub fn while_elements_mounted_auto_update_with_enabled(self, enabled: bool) -> Self {
        if enabled {
            self.while_elements_mounted_auto_update()
        } else {
            self
        }
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with `options`.
    pub fn while_elements_mounted_auto_update_with_options(
        self,
        options: AutoUpdateOptions,
    ) -> Self {
        let auto_update_rc: Rc<WhileElementsMountedFn> =
            Rc::new(move |reference, floating, update| {
                auto_update(reference, floating, update, options.clone())
            });

        self.while_elements_mounted(auto_update_rc)
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with `options` when `enabled` is `true`.
    pub fn while_elements_mounted_auto_update_with_enabled_and_options(
        self,
        enabled: bool,
        options: AutoUpdateOptions,
    ) -> Self {
        if enabled {
            self.while_elements_mounted_auto_update_with_options(options)
        } else {
            self
        }
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

impl FloatingStyles {
    pub fn style_position(&self) -> String {
        match self.position {
            Strategy::Absolute => "absolute".into(),
            Strategy::Fixed => "fixed".into(),
        }
    }

    pub fn style_top(&self) -> String {
        self.top.clone()
    }

    pub fn style_left(&self) -> String {
        self.left.clone()
    }

    pub fn style_transform(&self) -> Option<String> {
        self.transform.clone()
    }

    pub fn style_will_change(&self) -> Option<String> {
        self.will_change.clone()
    }
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

/// Return of [`use_floating`][crate::use_floating::use_floating].
pub struct UseFloatingReturn {
    /// The x-coord of the floating element.
    pub x: UseStateHandle<f64>,

    /// The y-coord of the floating element.
    pub y: UseStateHandle<f64>,

    /// The stateful placement, which can be different from the initial `placement` passed as options.
    pub placement: UseStateHandle<Placement>,

    /// The strategy to use when positioning the floating element.
    pub strategy: UseStateHandle<Strategy>,

    /// Additional data from middleware.
    pub middleware_data: UseStateHandle<MiddlewareData>,

    /// Indicates if the floating element has been positioned.
    pub is_positioned: UseStateHandle<bool>,

    /// CSS styles to apply to the floating element to position it.
    pub floating_styles: Rc<FloatingStyles>,

    /// The function to update floating position manually.
    pub update: Callback<()>,
}
