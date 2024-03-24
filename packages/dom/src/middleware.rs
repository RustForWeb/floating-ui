use floating_ui_core::{
    Arrow as CoreArrow, AutoPlacement as CoreAutoPlacement, Flip as CoreFlip, Offset as CoreOffset,
    Shift as CoreShift,
};
use web_sys::{Element, Window};

pub use floating_ui_core::{
    ArrowData, ArrowOptions, AutoPlacementData, AutoPlacementDataOverflow, AutoPlacementOptions,
    FlipData, FlipDataOverflow, FlipOptions, OffsetData, OffsetOptions, OffsetOptionsValues,
    ShiftData, ShiftOptions,
};

pub type Arrow<'a> = CoreArrow<'a, Element, Window>;
pub type AutoPlacement<'a> = CoreAutoPlacement<'a, Element, Window>;
pub type Flip<'a> = CoreFlip<'a, Element, Window>;
pub type Offset = CoreOffset<Element, Window>;
pub type Shift<'a> = CoreShift<'a, Element, Window>;
