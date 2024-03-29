use floating_ui_core::{Boundary as CoreBoundary, Middleware};
use floating_ui_utils::{
    ElementOrVirtual as CoreElementOrVirtual, OwnedElementOrVirtual as CoreOwnedElementOrVirtual,
};
use web_sys::{Element, Window};

pub type Boundary = CoreBoundary<Element>;

pub type ElementOrVirtual<'a> = CoreElementOrVirtual<'a, Element>;
pub type OwnedElementOrVirtual = CoreOwnedElementOrVirtual<Element>;

/// Vector of middleware used in [`ComputePositionConfig`][`crate::ComputePositionConfig`].
pub type MiddlewareVec = Vec<Box<dyn Middleware<Element, Window>>>;
