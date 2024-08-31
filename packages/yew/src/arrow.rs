use floating_ui_dom::{
    Arrow as CoreArrow, ArrowOptions as CoreArrowOptions, Middleware, MiddlewareReturn,
    MiddlewareState, Padding, ARROW_NAME,
};
use web_sys::wasm_bindgen::JsCast;
use yew::NodeRef;

/// Options for [`Arrow`].
#[derive(Clone, PartialEq)]
pub struct ArrowOptions {
    /// The arrow element to be positioned.
    pub element: NodeRef,

    /// The padding between the arrow element and the floating element edges.
    /// Useful when the floating element has rounded corners.
    ///
    /// Defaults to `0` on all sides.
    pub padding: Option<Padding>,
}

impl ArrowOptions {
    pub fn new(element: NodeRef) -> Self {
        ArrowOptions {
            element,
            padding: None,
        }
    }

    /// Set `element` option.
    pub fn element(mut self, value: NodeRef) -> Self {
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
        if let Some(element) = self.options.element.get() {
            CoreArrow::new(CoreArrowOptions {
                element: element
                    .dyn_into()
                    .expect("Arrow element should be an Element."),
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
