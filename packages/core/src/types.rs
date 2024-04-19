use std::collections::HashMap;
use std::fmt::Debug;

use dyn_clone::DynClone;
use serde::{de::DeserializeOwned, Serialize};

use floating_ui_utils::{
    ClientRectObject, Coords, Dimensions, ElementOrVirtual, ElementOrWindow, ElementRects, Length,
    OwnedElementOrWindow, Placement, Rect, Strategy,
};

pub type DerivableFn<'a, Element, Window, T> = &'a dyn Fn(MiddlewareState<Element, Window>) -> T;

pub enum Derivable<'a, Element: Clone, Window: Clone, T: Clone> {
    Value(T),
    Fn(DerivableFn<'a, Element, Window, T>),
}

impl<'a, Element: Clone, Window: Clone, T: Clone> Clone for Derivable<'a, Element, Window, T> {
    fn clone(&self) -> Self {
        match self {
            Self::Value(value) => Self::Value(value.clone()),
            Self::Fn(value) => Self::Fn(*value),
        }
    }
}

impl<'a, Element: Clone, Window: Clone, T: Clone> Derivable<'a, Element, Window, T> {
    pub fn evaluate(&self, state: MiddlewareState<Element, Window>) -> T {
        match self {
            Derivable::Value(value) => value.clone(),
            Derivable::Fn(func) => func(state),
        }
    }
}

impl<'a, Element: Clone, Window: Clone, T: Clone> From<T> for Derivable<'a, Element, Window, T> {
    fn from(value: T) -> Self {
        Derivable::Value(value)
    }
}

impl<'a, Element: Clone, Window: Clone, T: Clone> From<DerivableFn<'a, Element, Window, T>>
    for Derivable<'a, Element, Window, T>
{
    fn from(value: DerivableFn<'a, Element, Window, T>) -> Self {
        Derivable::Fn(value)
    }
}

/// Arguments for [`Platform::get_element_rects`].
pub struct GetElementRectsArgs<'a, Element: Clone> {
    pub reference: ElementOrVirtual<'a, Element>,
    pub floating: &'a Element,
    pub strategy: Strategy,
}

/// Arguments for [`Platform::get_clipping_rect`].
pub struct GetClippingRectArgs<'a, Element> {
    pub element: &'a Element,
    pub boundary: Boundary<Element>,
    pub root_boundary: RootBoundary,
    pub strategy: Strategy,
}

/// Arguments for [`Platform::convert_offset_parent_relative_rect_to_viewport_relative_rect`].
pub struct ConvertOffsetParentRelativeRectToViewportRelativeRectArgs<
    'a,
    Element: Clone,
    Window: Clone,
> {
    pub elements: Option<Elements<'a, Element>>,
    pub rect: Rect,
    pub offset_parent: Option<ElementOrWindow<'a, Element, Window>>,
    pub strategy: Strategy,
}

/// Platform interface methods to work with the current platform.
///
/// See <https://floating-ui.com/docs/platform> for the original documentation.
pub trait Platform<Element: Clone, Window: Clone>: Debug {
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

    fn get_client_rects(
        &self,
        _element: ElementOrVirtual<Element>,
    ) -> Option<Vec<ClientRectObject>> {
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
        self.values.get(key).map(|value| {
            serde_json::from_value::<D>(value.clone()).expect("JSON should be valid data.")
        })
    }

    pub fn set(&mut self, key: &str, value: serde_json::Value) {
        self.values.insert(key.into(), value);
    }

    pub fn set_as<S: Serialize>(&mut self, key: &str, value: S) {
        self.values.insert(
            key.into(),
            serde_json::to_value(value).expect("Data should be valid JSON."),
        );
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
    pub middleware: Option<Vec<Box<dyn Middleware<Element, Window>>>>,
}

impl<'a, Element, Window> ComputePositionConfig<'a, Element, Window> {
    pub fn new(platform: &'a dyn Platform<Element, Window>) -> Self {
        ComputePositionConfig {
            platform,
            placement: None,
            strategy: None,
            middleware: None,
        }
    }

    /// Set `platform` option.
    pub fn platform(mut self, value: &'a dyn Platform<Element, Window>) -> Self {
        self.platform = value;
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
pub trait Middleware<Element: Clone, Window: Clone>: DynClone {
    /// The name of this middleware.
    fn name(&self) -> &'static str;

    /// Executes this middleware.
    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn;
}

dyn_clone::clone_trait_object!(<Element, Window> Middleware<Element, Window>);

/// Middleware with options.
pub trait MiddlewareWithOptions<Element: Clone, Window: Clone, O: Clone> {
    /// The options passed to this middleware.
    fn options(&self) -> &Derivable<Element, Window, O>;
}

pub struct Elements<'a, Element: Clone> {
    pub reference: ElementOrVirtual<'a, Element>,
    pub floating: &'a Element,
}

impl<'a, Element: Clone> Elements<'a, Element> {
    pub fn get_element_context(
        &self,
        element_context: ElementContext,
    ) -> ElementOrVirtual<'a, Element> {
        match element_context {
            ElementContext::Reference => self.reference.clone(),
            ElementContext::Floating => self.floating.into(),
        }
    }
}

impl<'a, Element: Clone> Clone for Elements<'a, Element> {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference.clone(),
            floating: self.floating,
        }
    }
}

/// State passed to [`Middleware::compute`].
pub struct MiddlewareState<'a, Element: Clone, Window: Clone> {
    pub x: f64,
    pub y: f64,
    pub initial_placement: Placement,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: &'a MiddlewareData,
    pub elements: Elements<'a, Element>,
    pub rects: &'a ElementRects,
    pub platform: &'a dyn Platform<Element, Window>,
}

impl<'a, Element: Clone, Window: Clone> Clone for MiddlewareState<'a, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            initial_placement: self.initial_placement,
            placement: self.placement,
            strategy: self.strategy,
            middleware_data: self.middleware_data,
            elements: self.elements.clone(),
            rects: self.rects,
            platform: self.platform,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Boundary<Element> {
    ClippingAncestors,
    Element(Element),
    Elements(Vec<Element>),
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
