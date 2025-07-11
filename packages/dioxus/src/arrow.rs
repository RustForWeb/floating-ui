use std::rc::Rc;

use dioxus::{html::MountedData, signals::Signal, web::WebEventExt};
use floating_ui_dom::{
    ARROW_NAME, Arrow as CoreArrow, ArrowOptions as CoreArrowOptions, Middleware, MiddlewareReturn,
    MiddlewareState, Padding,
};

/// Options for [`Arrow`].
#[derive(Clone, PartialEq)]
pub struct ArrowOptions {
    /// The arrow element to be positioned.
    pub element: Signal<Option<Rc<MountedData>>>,

    /// The padding between the arrow element and the floating element edges.
    /// Useful when the floating element has rounded corners.
    ///
    /// Defaults to `0` on all sides.
    pub padding: Option<Padding>,
}

impl ArrowOptions {
    pub fn new(element: Signal<Option<Rc<MountedData>>>) -> Self {
        ArrowOptions {
            element,
            padding: None,
        }
    }

    /// Set `element` option.
    pub fn element(mut self, value: Signal<Option<Rc<MountedData>>>) -> Self {
        self.element = value;
        self
    }

    /// Set `padding` option.
    pub fn padding(mut self, value: Padding) -> Self {
        self.padding = Some(value);
        self
    }
}

/// Arrow middleware.
///
/// Provides data to position an inner element of the floating element so that it appears centered to the reference element.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/arrow.html) for more documentation.
#[derive(Clone, PartialEq)]
pub struct Arrow {
    options: ArrowOptions,
}

impl Arrow {
    pub fn new(options: ArrowOptions) -> Self {
        Arrow { options }
    }
}

impl Middleware<web_sys::Element, web_sys::Window> for Arrow {
    fn name(&self) -> &'static str {
        ARROW_NAME
    }

    fn compute(
        &self,
        state: MiddlewareState<web_sys::Element, web_sys::Window>,
    ) -> MiddlewareReturn {
        match (self.options.element)().map(|element| element.as_web_event()) {
            Some(element) => CoreArrow::new(CoreArrowOptions {
                element,
                padding: self.options.padding.clone(),
            })
            .compute(state),
            _ => MiddlewareReturn {
                x: None,
                y: None,
                data: None,
                reset: None,
            },
        }
    }
}
