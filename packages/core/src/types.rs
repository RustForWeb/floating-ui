use floating_ui_utils::{
    ClientRectObject, Coords, Dimensions, ElementRects, Placement, Rect, Strategy,
};

// TODO
type Element = bool;
pub type ReferenceElement = Element;
pub type FloatingElement = Element;
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

pub trait Middleware {
    fn name(&self) -> String;

    // TODO: return type
    fn options(&self) -> bool;

    // fn run(&self, state: MiddlewareState) -> MiddlewareReturn;
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
    placement: Placement,
    strategy: Strategy,
    middleware_data: MiddlewareData,
}
