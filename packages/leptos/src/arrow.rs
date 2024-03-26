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
        let element = self.options.element.get();

        if let Some(element) = element {
            CoreArrow::new(CoreArrowOptions {
                element: &element,
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
