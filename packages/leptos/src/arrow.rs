use std::ops::Deref;

use floating_ui_dom::{
    Arrow as CoreArrow, ArrowOptions as CoreArrowOptions, Middleware, MiddlewareReturn,
    MiddlewareState, Padding, ARROW_NAME,
};
use leptos::{html::ElementDescriptor, NodeRef};

/// Options for [`Arrow`].
#[derive(Clone)]
pub struct ArrowOptions<Descriptor, Element>
where
    Descriptor: ElementDescriptor + Deref<Target = Element> + Clone + 'static,
    Element: Deref<Target = web_sys::HtmlElement>,
{
    /// The arrow element to be positioned.
    pub element: NodeRef<Descriptor>,

    /// The padding between the arrow element and the floating element edges.
    /// Useful when the floating element has rounded corners.
    ///
    /// Defaults to `0` on all sides.
    pub padding: Option<Padding>,
}

impl<Descriptor, Element> ArrowOptions<Descriptor, Element>
where
    Descriptor: ElementDescriptor + Deref<Target = Element> + Clone + 'static,
    Element: Deref<Target = web_sys::HtmlElement>,
{
    pub fn new(element: NodeRef<Descriptor>) -> Self {
        ArrowOptions {
            element,
            padding: None,
        }
    }

    /// Set `element` option.
    pub fn element(mut self, value: NodeRef<Descriptor>) -> Self {
        self.element = value;
        self
    }

    /// Set `padding` option.
    pub fn padding(mut self, value: Padding) -> Self {
        self.padding = Some(value);
        self
    }
}

/// Provides data to position an inner element of the floating element so that it appears centered to the reference element.
///
/// See <https://floating-ui.com/docs/arrow> for the original documentation.
#[derive(Clone)]
pub struct Arrow<Descriptor, Element>
where
    Descriptor: ElementDescriptor + Deref<Target = Element> + Clone + 'static,
    Element: Deref<Target = web_sys::HtmlElement> + Clone,
{
    options: ArrowOptions<Descriptor, Element>,
}

impl<Descriptor, Element> Arrow<Descriptor, Element>
where
    Descriptor: ElementDescriptor + Deref<Target = Element> + Clone + 'static,
    Element: Deref<Target = web_sys::HtmlElement> + Clone,
{
    pub fn new(options: ArrowOptions<Descriptor, Element>) -> Self {
        Arrow { options }
    }
}

impl<Descriptor, Element> Middleware<web_sys::Element, web_sys::Window>
    for Arrow<Descriptor, Element>
where
    Descriptor: ElementDescriptor + Deref<Target = Element> + Clone + 'static,
    Element: Deref<Target = web_sys::HtmlElement> + Clone,
{
    fn name(&self) -> &'static str {
        ARROW_NAME
    }

    fn compute(
        &self,
        state: MiddlewareState<web_sys::Element, web_sys::Window>,
    ) -> MiddlewareReturn {
        let element = self.options.element.get_untracked();

        if let Some(element) = element {
            let element: &web_sys::Element = &element;

            CoreArrow::new(CoreArrowOptions {
                element: element.clone(),
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
