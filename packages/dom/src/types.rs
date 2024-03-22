use floating_ui_core::{
    AutoPlacementOptions as CoreAutoPlacementOptions, Boundary as CoreBoundary,
    ComputePositionConfig as CoreComputePositionConfig,
    DetectOverflowOptions as CoreDetectOverflowOptions, Elements as CoreElements,
    Middleware as CoreMiddleware, MiddlewareState as CoreMiddlewareState,
};
use floating_ui_utils::ElementOrVirtual as CoreElementOrVirtual;
use web_sys::Element;

pub type Boundary<'a> = CoreBoundary<'a, Element>;

pub type DetectOverflowOptions<'a> = CoreDetectOverflowOptions<'a, Element>;

pub type ComputePositionConfig<'a> = CoreComputePositionConfig<'a, Element>;

pub type ElementOrVirtual<'a> = CoreElementOrVirtual<'a, Element>;

pub type Elements<'a> = CoreElements<'a, Element>;

pub type MiddlewareState<'a> = CoreMiddlewareState<'a, Element>;

pub trait Middleware: CoreMiddleware<Element> {}

pub type AutoPlacementOptions<'a> = CoreAutoPlacementOptions<'a, Element>;
