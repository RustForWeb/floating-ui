//! This is the library to use Floating UI on the web, wrapping [`floating_ui_core`] with DOM interface logic.

mod middleware;
mod platform;
mod types;
mod utils;

pub use crate::middleware::*;
pub use floating_ui_core::{
    ComputePositionReturn, DetectOverflowOptions, Middleware, MiddlewareData, MiddlewareReturn,
    MiddlewareState, MiddlewareWithOptions,
};
#[doc(no_inline)]
pub use floating_ui_utils::*;

use floating_ui_core::{
    compute_position as compute_position_core, ComputePositionConfig as CoreComputePositionConfig,
};
use web_sys::{Element, Window};

use self::platform::Platform;

const PLATFORM: Platform = Platform {};

/// Options for [`compute_position`].
#[derive(Clone, Default)]
pub struct ComputePositionConfig<'a> {
    pub placement: Option<Placement>,
    pub strategy: Option<Strategy>,
    pub middleware: Option<Vec<&'a dyn Middleware<Element, Window>>>,
}

/// Computes the `x` and `y` coordinates that will place the floating element next to a given reference element.
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
