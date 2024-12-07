use floating_ui_dom::{
    Arrow as CoreArrow, ArrowOptions as CoreArrowOptions, Middleware, MiddlewareReturn,
    MiddlewareState, Padding, ARROW_NAME,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

/// Options for [`Arrow`].
#[derive(Clone)]
pub struct ArrowOptions {
    /// The arrow element to be positioned.
    pub element: AnyNodeRef,

    /// The padding between the arrow element and the floating element edges.
    /// Useful when the floating element has rounded corners.
    ///
    /// Defaults to `0` on all sides.
    pub padding: Option<Padding>,
}

impl ArrowOptions {
    pub fn new(element: AnyNodeRef) -> Self {
        ArrowOptions {
            element,
            padding: None,
        }
    }

    /// Set `element` option.
    pub fn element(mut self, value: AnyNodeRef) -> Self {
        self.element = value;
        self
    }

    /// Set `padding` option.
    pub fn padding(mut self, value: Padding) -> Self {
        self.padding = Some(value);
        self
    }
}

impl PartialEq for ArrowOptions {
    fn eq(&self, other: &Self) -> bool {
        self.element.get_untracked() == other.element.get_untracked()
            && self.padding == other.padding
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
        let element = self
            .options
            .element
            .get_untracked()
            .and_then(|element| element.dyn_into::<web_sys::Element>().ok());

        if let Some(element) = element {
            CoreArrow::new(CoreArrowOptions {
                element,
                padding: self.options.padding.clone(),
            })
            .compute(state)
        } else {
            MiddlewareReturn {
                x: None,
                y: None,
                data: None,
                reset: None,
            }
        }
    }
}
