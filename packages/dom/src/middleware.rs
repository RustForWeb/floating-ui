use floating_ui_core::{
    AutoPlacement as CoreAutoPlacement, Flip as CoreFlip, Offset as CoreOffset, Shift as CoreShift,
};
use web_sys::{Element, Window};

pub use floating_ui_core::{
    AutoPlacementOptions, FlipOptions, OffsetOptions, OffsetOptionsValues, ShiftOptions,
};

pub type AutoPlacement<'a> = CoreAutoPlacement<'a, Element, Window>;
pub type Flip<'a> = CoreFlip<'a, Element, Window>;
pub type Offset = CoreOffset<Element, Window>;
pub type Shift<'a> = CoreShift<'a, Element, Window>;
