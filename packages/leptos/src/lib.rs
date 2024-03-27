mod arrow;
mod types;
mod use_floating;
mod utils;

pub use arrow::*;
pub use types::*;
pub use use_floating::*;

#[doc(no_inline)]
pub use floating_ui_dom::{
    compute_position, dom, AlignedPlacement, Alignment, AutoPlacement, AutoPlacementData,
    AutoPlacementDataOverflow, AutoPlacementOptions, Axis, ClientRectObject, ComputePositionConfig,
    ComputePositionReturn, Coords, DetectOverflowOptions, Dimensions, ElementOrVirtual,
    ElementRects, Flip, FlipData, FlipDataOverflow, FlipOptions, Length, Middleware,
    MiddlewareData, MiddlewareReturn, MiddlewareState, MiddlewareVec, MiddlewareWithOptions,
    Offset, OffsetData, OffsetOptions, OffsetOptionsValues, Padding, Placement, Rect, Shift,
    ShiftData, ShiftOptions, Side, Strategy, VirtualElement,
};
