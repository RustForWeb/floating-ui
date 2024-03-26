use floating_ui_utils::{clamp, get_opposite_axis, get_side_axis, Axis, Coords, Side};
use serde::{Deserialize, Serialize};

use crate::{
    detect_overflow::{detect_overflow, DetectOverflowOptions},
    types::{
        Derivable, DerivableFn, Middleware, MiddlewareReturn, MiddlewareState,
        MiddlewareWithOptions,
    },
};

/// Name of the [`Shift`] middleware.
pub const SHIFT_NAME: &str = "shift";

/// Limiter used by [`Shift`] middleware. Limits the shifting done in order to prevent detachment.
pub trait Limiter<Element, Window> {
    fn compute(&self, state: MiddlewareState<Element, Window>) -> Coords;
}

/// Default [`Limiter`], which doesn't limit shifting.
pub struct DefaultLimiter;

impl<Element, Window> Limiter<Element, Window> for DefaultLimiter {
    fn compute(&self, state: MiddlewareState<Element, Window>) -> Coords {
        Coords {
            x: state.x,
            y: state.y,
        }
    }
}

/// Options for [`Shift`] middleware.
pub struct ShiftOptions<'a, Element, Window> {
    /// Options for [`detect_overflow`].
    ///
    /// Defaults to [`DetectOverflowOptions::default`].
    pub detect_overflow: Option<DetectOverflowOptions<'a, Element>>,

    /// The axis that runs along the alignment of the floating element. Determines whether overflow along this axis is checked to perform shifting.
    ///
    /// Defaults to `true`.
    pub main_axis: Option<bool>,

    /// The axis that runs along the side of the floating element. Determines whether overflow along this axis is checked to perform shifting.
    ///
    /// Defaults to `false`.
    pub cross_axis: Option<bool>,

    /// Accepts a limiter that limits the shifting done in order to prevent detachment.
    ///
    /// Defaults to [`DefaultLimiter`].
    pub limiter: Option<&'a dyn Limiter<Element, Window>>,
}

impl<'a, Element, Window> Clone for ShiftOptions<'a, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            detect_overflow: self.detect_overflow.clone(),
            main_axis: self.main_axis,
            cross_axis: self.cross_axis,
            limiter: self.limiter,
        }
    }
}

impl<'a, Element, Window> Default for ShiftOptions<'a, Element, Window> {
    fn default() -> Self {
        Self {
            detect_overflow: Default::default(),
            main_axis: Default::default(),
            cross_axis: Default::default(),
            limiter: Default::default(),
        }
    }
}

/// Data stored by [`Shift`] middleware.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShiftData {
    pub x: f64,
    pub y: f64,
}

/// Optimizes the visibility of the floating element by shifting it in order to keep it in view when it will overflow the clipping boundary.
///
/// See <https://floating-ui.com/docs/shift> for the original documentation.
pub struct Shift<'a, Element, Window> {
    options: Derivable<Element, Window, ShiftOptions<'a, Element, Window>>,
}

impl<'a, Element, Window> Shift<'a, Element, Window> {
    /// Constructs a new instance of this middleware.
    pub fn new(options: ShiftOptions<'a, Element, Window>) -> Self {
        Shift {
            options: options.into(),
        }
    }

    /// Constructs a new instance of this middleware with derivable options.
    pub fn new_derivable(
        options: DerivableFn<Element, Window, ShiftOptions<'a, Element, Window>>,
    ) -> Self {
        Shift {
            options: options.into(),
        }
    }
}

impl<'a, Element, Window> Middleware<Element, Window> for Shift<'a, Element, Window> {
    fn name(&self) -> &'static str {
        SHIFT_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let options = self.options.evaluate(state.clone());

        let MiddlewareState {
            x, y, placement, ..
        } = state;

        let check_main_axis = options.main_axis.unwrap_or(true);
        let check_cross_axis = options.cross_axis.unwrap_or(false);
        let limiter = options.limiter.unwrap_or(&DefaultLimiter {});

        let coords = Coords { x, y };
        let overflow = detect_overflow(
            MiddlewareState {
                elements: state.elements.clone(),
                ..state
            },
            options.detect_overflow.unwrap_or_default(),
        );
        let cross_axis = get_side_axis(placement);
        let main_axis = get_opposite_axis(cross_axis);

        let mut main_axis_coord = coords.get_axis(main_axis);
        let mut cross_axis_coord = coords.get_axis(cross_axis);

        if check_main_axis {
            let min_side = match main_axis {
                Axis::X => Side::Left,
                Axis::Y => Side::Top,
            };
            let max_side = match main_axis {
                Axis::X => Side::Right,
                Axis::Y => Side::Bottom,
            };
            let min = main_axis_coord + overflow.get_side(min_side);
            let max = main_axis_coord + overflow.get_side(max_side);

            main_axis_coord = clamp(min, main_axis_coord, max);
        }

        if check_cross_axis {
            let min_side = match main_axis {
                Axis::X => Side::Left,
                Axis::Y => Side::Top,
            };
            let max_side = match main_axis {
                Axis::X => Side::Right,
                Axis::Y => Side::Bottom,
            };
            let min = cross_axis_coord + overflow.get_side(min_side);
            let max = cross_axis_coord + overflow.get_side(max_side);

            cross_axis_coord = clamp(min, cross_axis_coord, max);
        }

        let limited_coords = limiter.compute(MiddlewareState {
            x: match main_axis {
                Axis::X => main_axis_coord,
                Axis::Y => cross_axis_coord,
            },
            y: match main_axis {
                Axis::X => cross_axis_coord,
                Axis::Y => main_axis_coord,
            },
            ..state
        });

        MiddlewareReturn {
            x: Some(limited_coords.x),
            y: Some(limited_coords.y),
            data: Some(
                serde_json::to_value(ShiftData {
                    x: limited_coords.x - x,
                    y: limited_coords.y - y,
                })
                .expect("Data should be valid JSON."),
            ),
            reset: None,
        }
    }
}

impl<'a, Element, Window> MiddlewareWithOptions<Element, Window, ShiftOptions<'a, Element, Window>>
    for Shift<'a, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, ShiftOptions<'a, Element, Window>> {
        &self.options
    }
}
