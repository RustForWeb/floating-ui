use floating_ui_core::middleware::{
    Arrow as CoreArrow, AutoPlacement as CoreAutoPlacement, Flip as CoreFlip, Hide as CoreHide,
    Offset as CoreOffset, Shift as CoreShift,
};
use web_sys::{Element, Window};

pub use floating_ui_core::middleware::{
    ArrowData, ArrowOptions, AutoPlacementData, AutoPlacementDataOverflow, AutoPlacementOptions,
    FlipData, FlipDataOverflow, FlipOptions, OffsetData, OffsetOptions, OffsetOptionsValues,
    ShiftData, ShiftOptions, ARROW_NAME, AUTO_PLACEMENT_NAME, FLIP_NAME, HIDE_NAME, OFFSET_NAME,
    SHIFT_NAME,
};

/// Provides data to position an inner element of the floating element so that it appears centered to the reference element.
///
/// See <https://floating-ui.com/docs/arrow> for the original documentation.
pub type Arrow<'a> = CoreArrow<'a, Element, Window>;

/// Optimizes the visibility of the floating element by choosing the placement that has the most space available automatically, without needing to specify a preferred placement.
/// Alternative to [`Flip`].
///
/// See <https://floating-ui.com/docs/autoPlacement> for the original documentation.
pub type AutoPlacement<'a> = CoreAutoPlacement<'a, Element, Window>;

/// Optimizes the visibility of the floating element by flipping the `placement` in order to keep it in view when the preferred placement(s) will overflow the clipping boundary.
/// Alternative to [`AutoPlacement`].
///
/// See <https://floating-ui.com/docs/flip> for the original documentation.
pub type Flip<'a> = CoreFlip<'a, Element, Window>;

/// Provides data to hide the floating element in applicable situations,
/// such as when it is not in the same clipping context as the reference element.
///
/// See <https://floating-ui.com/docs/hide> for the original documentation.
pub type Hide<'a> = CoreHide<'a, Element, Window>;

/// Modifies the placement by translating the floating element along the specified axes.
///
/// See <https://floating-ui.com/docs/offset> for the original documentation.
pub type Offset<'a> = CoreOffset<'a, Element, Window>;

/// Optimizes the visibility of the floating element by shifting it in order to keep it in view when it will overflow the clipping boundary.
///
/// See <https://floating-ui.com/docs/shift> for the original documentation.
pub type Shift<'a> = CoreShift<'a, Element, Window>;
