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

pub struct GetClippingRectArgs {
    pub element: Element,
    pub boundary: Boundary,
    pub root_boundary: RootBoundary,
    pub strategy: Strategy,
}

pub trait Platform {
    // TODO: check arg type, currently all anys are replaced by Element

    fn get_element_rects(&self, args: GetElementRectsArgs) -> ElementRects;

    fn get_clipping_rect(&self, args: GetClippingRectArgs) -> Rect;

    fn get_dimensions(&self, element: Element) -> Dimensions;

    // TODO: args
    fn convert_offset_parent_relative_react_to_viewport_relative_rect(&self) -> Option<Rect> {
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

pub struct MiddlewareData {
    // TODO
}

pub struct ComputePositionConfig {
    pub platform: Box<dyn Platform>,
    pub placement: Option<Placement>,
    pub strategy: Option<Strategy>,
    pub middleware: Option<Vec<Box<dyn Middleware>>>,
}

pub struct ComputePositionReturn {
    pub x: isize,
    pub y: isize,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: MiddlewareData,
}

pub enum ResetRects {
    True,
    Value(ElementRects),
}

pub struct ResetValue {
    pub placement: Option<Placement>,
    pub rects: Option<ResetRects>,
}

pub enum Reset {
    True,
    Value(ResetValue),
}

pub struct MiddlewareReturn {
    pub x: Option<isize>,
    pub y: Option<isize>,
    pub data: Option<bool>, // TODO
    pub reset: Option<Reset>,
}

pub trait Middleware {
    fn name(&self) -> String;

    // TODO: return type
    fn options(&self) -> bool;

    fn compute(&self, state: MiddlewareState) -> MiddlewareReturn;
}

pub type ReferenceElement = Element;
pub type FloatingElement = Element;

pub struct Elements {
    pub reference: ReferenceElement,
    pub floating: FloatingElement,
}

pub struct MiddlewareState<'a> {
    pub x: isize,
    pub y: isize,
    pub initial_placement: Placement,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: &'a MiddlewareData,
    pub elements: &'a Elements,
    pub rects: &'a ElementRects,
    pub platform: &'a Box<dyn Platform>,
}

pub type Boundary = Element;

pub enum RootBoundary {
    Viewport,
    Document,
    Rect(Rect),
}

// pub enum ElementContext {
//     Reference,
//     Floating
// }
