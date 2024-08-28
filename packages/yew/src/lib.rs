//! Rust port of [Floating UI](https://floating-ui.com/).
//!
//! This is the library to use Floating UI with Yew.
//!
//! See [the Rust Floating UI book](https://floating-ui.rustforweb.org/frameworks/yew.html) for more documenation.

mod arrow;
mod types;
mod use_auto_update;
mod use_floating;
mod utils;

pub use arrow::*;
pub use types::*;
pub use use_auto_update::*;
pub use use_floating::*;

#[doc(no_inline)]
pub use floating_ui_dom::{
    auto_update, compute_position, dom, AlignedPlacement, Alignment, ApplyState, ArrowData,
    AutoPlacement, AutoPlacementData, AutoPlacementDataOverflow, AutoPlacementOptions,
    AutoUpdateOptions, Axis, Boundary, ClientRectObject, ComputePositionConfig,
    ComputePositionReturn, Coords, DefaultLimiter, DefaultVirtualElement, Derivable, DerivableFn,
    DetectOverflowOptions, Dimensions, ElementContext, ElementOrVirtual, ElementRects,
    FallbackStrategy, Flip, FlipData, FlipDataOverflow, FlipOptions, Hide, HideData, HideOptions,
    HideStrategy, Inline, InlineOptions, Length, LimitShift, LimitShiftOffset,
    LimitShiftOffsetValues, LimitShiftOptions, Middleware, MiddlewareData, MiddlewareReturn,
    MiddlewareState, MiddlewareVec, MiddlewareWithOptions, Offset, OffsetData, OffsetOptions,
    OffsetOptionsValues, Padding, Placement, Rect, RootBoundary, Shift, ShiftData, ShiftOptions,
    Side, Size, SizeOptions, Strategy, VirtualElement, ARROW_NAME, AUTO_PLACEMENT_NAME, FLIP_NAME,
    HIDE_NAME, INLINE_NAME, OFFSET_NAME, SHIFT_NAME, SIZE_NAME,
};
