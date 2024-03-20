use floating_ui_utils::{Dimensions, ElementRects, Placement, Rect, Strategy};

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
    reference: ReferenceElement,
    floating: FloatingElement,
    strategy: Strategy,
}

pub struct GetClippingRectArgs {
    element: Element,
    boundary: Boundary,
    root_boundary: RootBoundary,
    strategy: Strategy,
}

pub trait Platform {
    fn get_element_rects(&self, args: GetElementRectsArgs) -> ElementRects;

    fn get_clipping_rect(&self, args: GetClippingRectArgs) -> Rect;

    fn get_dimensions(&self, element: Element) -> Dimensions;

    // TODO: optional funcs
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
    platform: Box<dyn Platform>,
    placement: Option<Placement>,
    strategy: Option<Strategy>,
    middleware: Option<Vec<Box<dyn Middleware>>>,
}

pub struct ComputePositionReturn {
    placement: Placement,
    strategy: Strategy,
    middleware_data: MiddlewareData,
}
