use floating_ui_core::middleware::{
    Arrow as CoreArrow, AutoPlacement as CoreAutoPlacement, Flip as CoreFlip, Hide as CoreHide,
    Inline as CoreInline, Offset as CoreOffset, Shift as CoreShift, Size as CoreSize,
};
use web_sys::{Element, Window};

pub use floating_ui_core::middleware::{
    ApplyState, ArrowData, ArrowOptions, AutoPlacementData, AutoPlacementDataOverflow,
    AutoPlacementOptions, DefaultLimiter, FallbackStrategy, FlipData, FlipDataOverflow,
    FlipOptions, HideData, HideOptions, HideStrategy, InlineOptions, LimitShift, LimitShiftOffset,
    LimitShiftOffsetValues, LimitShiftOptions, OffsetData, OffsetOptions, OffsetOptionsValues,
    ShiftData, ShiftOptions, SizeOptions, ARROW_NAME, AUTO_PLACEMENT_NAME, FLIP_NAME, HIDE_NAME,
    INLINE_NAME, OFFSET_NAME, SHIFT_NAME, SIZE_NAME,
};

/// Arrow middleware.
///
/// Provides data to position an inner element of the floating element so that it appears centered to the reference element.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/arrow.html) for more documentation.
pub type Arrow<'a> = CoreArrow<'a, Element, Window>;

/// Auto placement middleware.
///
/// Optimizes the visibility of the floating element by choosing the placement that has the most space available automatically,
/// without needing to specify a preferred placement.
///
/// Alternative to [`Flip`].
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/auto-placement.html) for more documentation.
pub type AutoPlacement<'a> = CoreAutoPlacement<'a, Element, Window>;

/// Flip middleware.
///
/// Optimizes the visibility of the floating element by flipping the `placement` in order to keep it in view when the preferred placement(s) will overflow the clipping boundary.
/// Alternative to [`AutoPlacement`].
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/flip.html) for more documentation.
pub type Flip<'a> = CoreFlip<'a, Element, Window>;

/// Hide middleware.
///
/// Provides data to hide the floating element in applicable situations,
/// such as when it is not in the same clipping context as the reference element.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/hide.html) for more documentation.
pub type Hide<'a> = CoreHide<'a, Element, Window>;

/// Inline middleware.
///
/// Provides improved positioning for inline reference elements that can span over multiple lines, such as hyperlinks or range selections.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/inline.html) for more documentation.
pub type Inline<'a> = CoreInline<'a, Element, Window>;

/// Offset middleware.
///
/// Modifies the placement by translating the floating element along the specified axes.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/offset.html) for more documentation.
pub type Offset<'a> = CoreOffset<'a, Element, Window>;

/// Shift middleware.
///
/// Optimizes the visibility of the floating element by shifting it in order to keep it in view when it will overflow the clipping boundary.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/shift.html) for more documentation.
pub type Shift<'a> = CoreShift<'a, Element, Window>;

/// Size middleware.
///
/// Provides data that allows you to change the size of the floating element -
/// for instance, prevent it from overflowing the clipping boundary or match the width of the reference element.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/size.html) for more documentation.
pub type Size<'a> = CoreSize<'a, Element, Window>;
