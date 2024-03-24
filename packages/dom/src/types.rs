use floating_ui_core::Boundary as CoreBoundary;
// use floating_ui_core::{
//     AutoPlacementOptions as CoreAutoPlacementOptions, Boundary as CoreBoundary,
//     ComputePositionConfig as CoreComputePositionConfig,
//     DetectOverflowOptions as CoreDetectOverflowOptions, Elements as CoreElements,
//     MiddlewareState as CoreMiddlewareState,
// };
use floating_ui_utils::ElementOrVirtual as CoreElementOrVirtual;
use web_sys::Element;

pub type Boundary<'a> = CoreBoundary<'a, Element>;

// pub type ComputePositionConfig<'a> = CoreComputePositionConfig<'a, Element, Window>;

pub type ElementOrVirtual<'a> = CoreElementOrVirtual<'a, Element>;

// pub type Elements<'a> = CoreElements<'a, Element>;

// pub type MiddlewareState<'a> = CoreMiddlewareState<'a, Element, Window>;

// pub type AutoPlacementOptions<'a> = CoreAutoPlacementOptions<'a, Element>;
