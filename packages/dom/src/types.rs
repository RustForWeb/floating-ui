use floating_ui_core::{Boundary as CoreBoundary, Middleware};
use floating_ui_utils::ElementOrVirtual as CoreElementOrVirtual;
use web_sys::{Element, Window};

pub type Boundary = CoreBoundary<Element>;

pub type ElementOrVirtual<'a> = CoreElementOrVirtual<'a, Element>;

/// Vector of middleware used in [`ComputePositionConfig`][`crate::ComputePositionConfig`].
pub type MiddlewareVec = Vec<Box<dyn Middleware<Element, Window>>>;
