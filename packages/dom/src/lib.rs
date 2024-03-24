mod middleware;
mod platform;
mod types;
mod utils;

pub use crate::middleware::*;
pub use floating_ui_core::{ComputePositionReturn, DetectOverflowOptions};
pub use floating_ui_utils::{
    AlignedPlacement, Alignment, Axis, ClientRectObject, Coords, Dimensions, ElementRects, Length,
    Padding, Placement, Rect, Side, SideObject, Strategy,
};

use floating_ui_core::{
    compute_position as compute_position_core, ComputePositionConfig as CoreComputePositionConfig,
    Middleware,
};
use web_sys::{Element, Window};

use self::platform::Platform;

const PLATFORM: Platform = Platform {};

#[derive(Clone, Default)]
pub struct ComputePositionConfig<'a> {
    pub placement: Option<Placement>,
    pub strategy: Option<Strategy>,
    pub middleware: Option<Vec<&'a dyn Middleware<Element, Window>>>,
}

pub fn compute_position(
    reference: &Element,
    floating: &Element,
    config: Option<ComputePositionConfig>,
) -> ComputePositionReturn {
    let config = config.unwrap_or_default();

    // TODO: cache

    compute_position_core(
        reference,
        floating,
        CoreComputePositionConfig {
            platform: &PLATFORM,
            placement: config.placement,
            strategy: config.strategy,
            middleware: config.middleware,
        },
    )
}
