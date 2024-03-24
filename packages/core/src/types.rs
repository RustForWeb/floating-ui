use std::collections::HashMap;
use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use floating_ui_utils::{
    ClientRectObject, Coords, Dimensions, ElementOrWindow, ElementRects, Length,
    OwnedElementOrWindow, Placement, Rect, Strategy,
};

/// Arguments for [`Platform::get_element_rects`].
pub struct GetElementRectsArgs<'a, Element> {
    pub reference: &'a Element,
    pub floating: &'a Element,
    pub strategy: Strategy,
}

/// Arguments for [`Platform::get_clipping_rect`].
pub struct GetClippingRectArgs<'a, Element> {
    pub element: &'a Element,
    pub boundary: Boundary<'a, Element>,
    pub root_boundary: RootBoundary,
    pub strategy: Strategy,
}

/// Arguments for [`Platform::convert_offset_parent_relative_rect_to_viewport_relative_rect`].
pub struct ConvertOffsetParentRelativeRectToViewportRelativeRectArgs<'a, Element, Window> {
    pub elements: Option<Elements<'a, Element>>,
    pub rect: Rect,
    pub offset_parent: Option<ElementOrWindow<'a, Element, Window>>,
    pub strategy: Strategy,
}

/// Platform interface methods to work with the current platform.
///
/// See <https://floating-ui.com/docs/platform> for the original documentation.
pub trait Platform<Element, Window>: Debug {
    fn get_element_rects(&self, args: GetElementRectsArgs<Element>) -> ElementRects;

    fn get_clipping_rect(&self, args: GetClippingRectArgs<Element>) -> Rect;

    fn get_dimensions(&self, element: &Element) -> Dimensions;

    fn convert_offset_parent_relative_rect_to_viewport_relative_rect(
        &self,
        _args: ConvertOffsetParentRelativeRectToViewportRelativeRectArgs<Element, Window>,
    ) -> Option<Rect> {
        None
    }

    fn get_offset_parent(
        &self,
        _element: &Element,
    ) -> Option<OwnedElementOrWindow<Element, Window>> {
        None
    }

    fn is_element(&self, _value: &ElementOrWindow<Element, Window>) -> Option<bool> {
        None
    }

    fn get_document_element(&self, _element: &Element) -> Option<Element> {
        None
    }

    fn get_client_rects(&self, _element: &Element) -> Option<Vec<ClientRectObject>> {
        None
    }

    fn is_rtl(&self, _element: &Element) -> Option<bool> {
        None
    }

    fn get_scale(&self, _element: &Element) -> Option<Coords> {
        None
    }

    fn get_client_length(&self, _element: &Element, _length: Length) -> Option<f64> {
        None
    }
}

/// Data stored by middleware.
#[derive(Clone, Debug, Default)]
pub struct MiddlewareData {
    values: HashMap<String, serde_json::Value>,
}

impl MiddlewareData {
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.values.get(key)
    }

    pub fn get_as<D: DeserializeOwned>(&self, key: &str) -> Option<D> {
        self.values
            .get(key)
            .map(|value| serde_json::from_value::<D>(value.clone()).unwrap())
    }

    pub fn set(&mut self, key: &str, value: serde_json::Value) {
        self.values.insert(key.into(), value);
    }

    pub fn set_as<S: Serialize>(&mut self, key: &str, value: S) {
        self.values
            .insert(key.into(), serde_json::to_value(value).unwrap());
    }
}

/// Options for [`compute_position`][crate::compute_position::compute_position].
#[derive(Clone)]
pub struct ComputePositionConfig<'a, Element, Window> {
    /// Object to interface with the current platform.
    pub platform: &'a dyn Platform<Element, Window>,

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
}

/// Return of [`compute_position`][crate::compute_position::compute_position].
#[derive(Clone, Debug)]
pub struct ComputePositionReturn {
    pub x: f64,
    pub y: f64,

    /// The final chosen placement of the floating element.
    pub placement: Placement,

    /// The strategy used to position the floating element.
    pub strategy: Strategy,

    /// Object containing data returned from all middleware, keyed by their name.
    pub middleware_data: MiddlewareData,
}

#[derive(Clone, Debug)]
pub enum ResetRects {
    True,
    Value(ElementRects),
}

#[derive(Clone, Debug)]
pub struct ResetValue {
    pub placement: Option<Placement>,
    pub rects: Option<ResetRects>,
}

#[derive(Clone, Debug)]
pub enum Reset {
    True,
    Value(ResetValue),
}

/// Return of [`Middleware::compute`].
#[derive(Clone, Debug)]
pub struct MiddlewareReturn {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub data: Option<serde_json::Value>,
    pub reset: Option<Reset>,
}

/// Middleware used by [`compute_position`][`crate::compute_position::compute_position`].
pub trait Middleware<Element, Window> {
    /// The name of this middleware.
    fn name(&self) -> &'static str;

    /// Executes this middleware.
    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn;
}

/// Middleware with options.
pub trait MiddlewareWithOptions<O> {
    /// The options passed to this middleware.
    fn options(&self) -> &O;
}

#[derive(Clone, Debug)]
pub struct Elements<'a, Element> {
    pub reference: &'a Element,
    pub floating: &'a Element,
}

impl<'a, Element> Elements<'a, Element> {
    pub fn get_element_context(&self, element_context: ElementContext) -> &Element {
        match element_context {
            ElementContext::Reference => self.reference,
            ElementContext::Floating => self.floating,
        }
    }
}

/// State passed to [`Middleware::compute`].
#[derive(Debug)]
pub struct MiddlewareState<'a, Element, Window> {
    pub x: f64,
    pub y: f64,
    pub initial_placement: Placement,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: &'a MiddlewareData,
    pub elements: Elements<'a, &'a Element>,
    pub rects: &'a ElementRects,
    pub platform: &'a dyn Platform<Element, Window>,
}

#[derive(Debug)]
pub enum Boundary<'a, Element> {
    ClippingAncestors,
    Element(&'a Element),
    Elements(Vec<&'a Element>),
}

impl<'a, Element> Clone for Boundary<'a, Element> {
    fn clone(&self) -> Self {
        match self {
            Self::ClippingAncestors => Self::ClippingAncestors,
            Self::Element(e) => Self::Element(e),
            Self::Elements(e) => Self::Elements(e.clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum RootBoundary {
    Viewport,
    Document,
    Rect(Rect),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ElementContext {
    Reference,
    Floating,
}
