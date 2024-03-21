use floating_ui_utils::{
    ClientRectObject, Coords, Dimensions, ElementRects, Placement, Rect, Strategy,
};

// TODO
pub type Element = bool;

pub struct GetElementRectsArgs {
    pub reference: ReferenceElement,
    pub floating: FloatingElement,
    pub strategy: Strategy,
}

pub struct GetClippingRectArgs<'a> {
    pub element: &'a Element,
    pub boundary: Boundary,
    pub root_boundary: RootBoundary,
    pub strategy: Strategy,
}

pub struct ConvertOffsetParentRelativeRectToViewportRelativeRectArgs<'a> {
    pub elements: Option<&'a Elements>,
    pub rect: Rect,
    pub offset_parent: Option<Element>,
    pub strategy: Strategy,
}

pub trait Platform {
    // TODO: check arg type, currently all anys are replaced by Element

    fn get_element_rects(&self, args: GetElementRectsArgs) -> ElementRects;

    fn get_clipping_rect(&self, args: GetClippingRectArgs) -> Rect;

    fn get_dimensions(&self, element: Element) -> Dimensions;

    fn convert_offset_parent_relative_react_to_viewport_relative_rect(
        &self,
        args: ConvertOffsetParentRelativeRectToViewportRelativeRectArgs,
    ) -> Option<Rect> {
        None
    }

    fn get_offset_parent(&self, element: Element) -> Option<Element> {
        None
    }

    fn is_element(&self, value: Element) -> Option<bool> {
        None
    }

    fn get_document_element(&self, element: Element) -> Option<Element> {
        None
    }

    fn get_client_rects(&self, element: Element) -> Option<Vec<ClientRectObject>> {
        None
    }

    fn is_rtl(&self, element: Element) -> Option<bool> {
        None
    }

    fn get_scale(&self, element: Element) -> Option<Coords> {
        None
    }
}

#[derive(Clone, Debug)]
pub struct MiddlewareData {
    // TODO
}

#[derive(Clone)]
pub struct ComputePositionConfig<'a> {
    pub platform: &'a dyn Platform,
    pub placement: Option<Placement>,
    pub strategy: Option<Strategy>,
    pub middleware: Option<Vec<&'a dyn Middleware>>,
}

#[derive(Clone, Debug)]
pub struct ComputePositionReturn {
    pub x: isize,
    pub y: isize,
    pub placement: Placement,
    pub strategy: Strategy,
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

#[derive(Clone, Debug)]
pub struct MiddlewareReturn {
    pub x: Option<isize>,
    pub y: Option<isize>,
    pub data: Option<bool>, // TODO
    pub reset: Option<Reset>,
}

pub trait Middleware {
    fn name(&self) -> String;

    fn compute(&self, state: MiddlewareState) -> MiddlewareReturn;
}

pub trait MiddlewareWithOptions<O> {
    fn options(&self) -> &O;
}

pub type ReferenceElement = Element;
pub type FloatingElement = Element;

#[derive(Clone, Debug)]
pub struct Elements {
    pub reference: ReferenceElement,
    pub floating: FloatingElement,
}

impl Elements {
    pub fn get_element_context(&self, element_context: ElementContext) -> Element {
        match element_context {
            ElementContext::Reference => self.reference,
            ElementContext::Floating => self.floating,
        }
    }
}

#[derive(Clone)]
pub struct MiddlewareState<'a> {
    pub x: isize,
    pub y: isize,
    pub initial_placement: Placement,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: &'a MiddlewareData,
    pub elements: &'a Elements,
    pub rects: &'a ElementRects,
    pub platform: &'a dyn Platform,
}

#[derive(Clone, Debug)]
pub enum Boundary {
    ClippingAncestors,
    Element(Element),
    Elements(Vec<Element>),
    Rect(Rect),
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
