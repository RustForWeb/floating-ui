use floating_ui_core::{
    AutoPlacementOptions as CoreAutoPlacementOptions, Boundary as CoreBoundary,
    ComputePositionConfig as CoreComputePositionConfig,
    DetectOverflowOptions as CoreDetectOverflowOptions, Elements as CoreElements,
    Middleware as CoreMiddleware, MiddlewareState as CoreMiddlewareState, Platform as CorePlatform,
};
use web_sys::Element;

pub trait Platform: CorePlatform<Element> {}

#[derive(Clone, Debug)]
pub struct NodeScroll {
    pub scroll_left: isize,
    pub scroll_top: isize,
}

pub type Boundary<'a> = CoreBoundary<'a, Element>;

pub type DetectOverflowOptions<'a> = CoreDetectOverflowOptions<'a, Element>;

pub type ComputePositionConfig<'a> = CoreComputePositionConfig<'a, Element>;

// pub struct VirtualElement;

pub type Elements<'a> = CoreElements<'a, Element>;

pub type MiddlewareState<'a> = CoreMiddlewareState<'a, Element>;

pub trait Middleware: CoreMiddleware<Element> {}

pub type AutoPlacementOptions<'a> = CoreAutoPlacementOptions<'a, Element>;
