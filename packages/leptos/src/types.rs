use std::{fmt::Display, rc::Rc};

use floating_ui_dom::{
    AutoUpdateOptions, ElementOrVirtual, Middleware, MiddlewareData, Placement, Strategy,
    auto_update,
};
use leptos::{prelude::*, tachys::html::style::IntoStyle};
use send_wrapper::SendWrapper;
use web_sys::{Element, Window};

pub type WhileElementsMountedFn =
    dyn Fn(ElementOrVirtual, &Element, Rc<dyn Fn()>) -> WhileElementsMountedCleanupFn;

pub type WhileElementsMountedCleanupFn = Box<dyn Fn()>;

pub type WrappedMiddleware = SendWrapper<Vec<Box<dyn Middleware<Element, Window>>>>;

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
    pub middleware: MaybeProp<WrappedMiddleware>,

    ///  Whether to use `transform` for positioning instead of `top` and `left` in the `floatingStyles` object.
    ///
    /// Defaults to `true`.
    pub transform: MaybeProp<bool>,

    /// Callback to handle mounting/unmounting of the elements.
    ///
    /// Defaults to [`Option::None`].
    pub while_elements_mounted: MaybeProp<SendWrapper<Rc<WhileElementsMountedFn>>>,
}

impl UseFloatingOptions {
    /// Set `open` option.
    pub fn open<I: Into<MaybeProp<bool>>>(mut self, value: I) -> Self {
        self.open = value.into();
        self
    }

    /// Set `placement` option.
    pub fn placement<I: Into<MaybeProp<Placement>>>(mut self, value: I) -> Self {
        self.placement = value.into();
        self
    }

    /// Set `strategy` option.
    pub fn strategy<I: Into<MaybeProp<Strategy>>>(mut self, value: I) -> Self {
        self.strategy = value.into();
        self
    }

    /// Set `middleware` option.
    pub fn middleware<I: Into<MaybeProp<WrappedMiddleware>>>(mut self, value: I) -> Self {
        self.middleware = value.into();
        self
    }

    /// Set `transform` option.
    pub fn transform<I: Into<MaybeProp<bool>>>(mut self, value: I) -> Self {
        self.transform = value.into();
        self
    }

    /// Set `while_elements_mounted` option.
    pub fn while_elements_mounted<I: Into<MaybeProp<SendWrapper<Rc<WhileElementsMountedFn>>>>>(
        mut self,
        value: I,
    ) -> Self {
        self.while_elements_mounted = value.into();
        self
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with [`AutoUpdateOptions::default`].
    pub fn while_elements_mounted_auto_update(self) -> Self {
        let auto_update_rc: SendWrapper<Rc<WhileElementsMountedFn>> =
            SendWrapper::new(Rc::new(|reference, floating, update| {
                auto_update(reference, floating, update, AutoUpdateOptions::default())
            }));
        self.while_elements_mounted(auto_update_rc)
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with [`AutoUpdateOptions::default`] when `enabled` is `true`.
    pub fn while_elements_mounted_auto_update_with_enabled(self, enabled: Signal<bool>) -> Self {
        let auto_update_rc: SendWrapper<Rc<WhileElementsMountedFn>> =
            SendWrapper::new(Rc::new(|reference, floating, update| {
                auto_update(reference, floating, update, AutoUpdateOptions::default())
            }));
        self.while_elements_mounted(MaybeProp::derive(move || {
            if enabled.get() {
                Some(auto_update_rc.clone())
            } else {
                None
            }
        }))
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with `options`.
    pub fn while_elements_mounted_auto_update_with_options(
        self,
        options: Signal<AutoUpdateOptions>,
    ) -> Self {
        let auto_update_rc =
            move |options: AutoUpdateOptions| -> SendWrapper<Rc<WhileElementsMountedFn>> {
                SendWrapper::new(Rc::new(move |reference, floating, update| {
                    auto_update(reference, floating, update, options.clone())
                }))
            };

        self.while_elements_mounted(MaybeProp::derive(move || {
            Some(auto_update_rc(options.get()))
        }))
    }

    /// Set `while_elements_mounted` option to [`auto_update`] with `options` when `enabled` is `true`.
    pub fn while_elements_mounted_auto_update_with_enabled_and_options(
        self,
        enabled: Signal<bool>,
        options: Signal<AutoUpdateOptions>,
    ) -> Self {
        let auto_update_rc =
            move |options: AutoUpdateOptions| -> SendWrapper<Rc<WhileElementsMountedFn>> {
                SendWrapper::new(Rc::new(move |reference, floating, update| {
                    auto_update(reference, floating, update, options.clone())
                }))
            };

        self.while_elements_mounted(MaybeProp::derive(move || {
            if enabled.get() {
                Some(auto_update_rc(options.get()))
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

impl FloatingStyles {
    pub fn style_position(&self) -> String {
        match self.position {
            Strategy::Absolute => "absolute".to_owned(),
            Strategy::Fixed => "fixed".to_owned(),
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

impl Display for FloatingStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "position: {}; top: {}; left: {};{}{}",
            match self.position {
                Strategy::Absolute => "absolute",
                Strategy::Fixed => "fixed",
            },
            self.top,
            self.left,
            self.transform
                .as_ref()
                .map_or("".to_owned(), |transform| format!(
                    " transform: {transform};"
                ),),
            self.will_change
                .as_ref()
                .map_or("".to_owned(), |will_change| format!(
                    " will-change: {will_change};"
                ))
        )
    }
}

impl IntoStyle for FloatingStyles {
    type AsyncOutput = Self;
    type State = (leptos::tachys::renderer::types::Element, Self);
    type Cloneable = Self;
    type CloneableOwned = Self;
    fn to_html(self, style: &mut String) {
        style.push_str(&self.to_string());
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        el: &leptos::tachys::renderer::types::Element,
    ) -> Self::State {
        (el.clone(), self)
    }

    fn build(self, el: &leptos::tachys::renderer::types::Element) -> Self::State {
        leptos::tachys::renderer::Rndr::set_attribute(el, "style", &self.to_string());
        (el.clone(), self)
    }

    fn rebuild(self, state: &mut Self::State) {
        let (el, prev) = state;
        if self != *prev {
            leptos::tachys::renderer::Rndr::set_attribute(el, "style", &self.to_string());
        }
        *prev = self;
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn reset(state: &mut Self::State) {
        let (el, _prev) = state;
        leptos::tachys::renderer::Rndr::remove_attribute(el, "style");
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
    pub update: SendWrapper<Rc<dyn Fn()>>,
}
