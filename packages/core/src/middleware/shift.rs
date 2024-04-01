use std::fmt::Debug;

use dyn_clone::DynClone;
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
pub trait Limiter<Element, Window>: Debug + DynClone {
    fn compute(&self, state: MiddlewareState<Element, Window>) -> Coords;
}

dyn_clone::clone_trait_object!(<Element, Window> Limiter<Element, Window>);

/// Default [`Limiter`], which doesn't limit shifting.
#[derive(Clone, Debug, Default)]
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
#[derive(Clone, Debug)]
pub struct ShiftOptions<Element: Clone, Window: Clone> {
    /// Options for [`detect_overflow`].
    ///
    /// Defaults to [`DetectOverflowOptions::default`].
    pub detect_overflow: Option<DetectOverflowOptions<Element>>,

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
    pub limiter: Option<Box<dyn Limiter<Element, Window>>>,
}

impl<Element: Clone, Window: Clone> ShiftOptions<Element, Window> {
    /// Set `detect_overflow` option.
    pub fn detect_overflow(mut self, value: DetectOverflowOptions<Element>) -> Self {
        self.detect_overflow = Some(value);
        self
    }

    /// Set `main_axis` option.
    pub fn main_axis(mut self, value: bool) -> Self {
        self.main_axis = Some(value);
        self
    }

    /// Set `cross_axis` option.
    pub fn cross_axis(mut self, value: bool) -> Self {
        self.cross_axis = Some(value);
        self
    }

    /// Set `limiter` option.
    pub fn limiter(mut self, value: Box<dyn Limiter<Element, Window>>) -> Self {
        self.limiter = Some(value);
        self
    }
}

impl<Element: Clone, Window: Clone> Default for ShiftOptions<Element, Window> {
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
pub struct Shift<'a, Element: Clone, Window: Clone> {
    options: Derivable<'a, Element, Window, ShiftOptions<Element, Window>>,
}

impl<'a, Element: Clone, Window: Clone> Shift<'a, Element, Window> {
    /// Constructs a new instance of this middleware.
    pub fn new(options: ShiftOptions<Element, Window>) -> Self {
        Shift {
            options: options.into(),
        }
    }

    /// Constructs a new instance of this middleware with derivable options.
    pub fn new_derivable(
        options: Derivable<'a, Element, Window, ShiftOptions<Element, Window>>,
    ) -> Self {
        Shift { options }
    }

    /// Constructs a new instance of this middleware with derivable options function.
    pub fn new_derivable_fn(
        options: DerivableFn<'a, Element, Window, ShiftOptions<Element, Window>>,
    ) -> Self {
        Shift {
            options: options.into(),
        }
    }
}

impl<'a, Element: Clone, Window: Clone> Clone for Shift<'a, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}

impl<'a, Element: Clone, Window: Clone> Middleware<Element, Window> for Shift<'a, Element, Window> {
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
        #[allow(clippy::unwrap_or_default)]
        let limiter = options.limiter.unwrap_or(Box::<DefaultLimiter>::default());

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

        let mut main_axis_coord = coords.axis(main_axis);
        let mut cross_axis_coord = coords.axis(cross_axis);

        if check_main_axis {
            let min_side = match main_axis {
                Axis::X => Side::Left,
                Axis::Y => Side::Top,
            };
            let max_side = match main_axis {
                Axis::X => Side::Right,
                Axis::Y => Side::Bottom,
            };
            let min = main_axis_coord + overflow.side(min_side);
            let max = main_axis_coord - overflow.side(max_side);

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
            let min = cross_axis_coord + overflow.side(min_side);
            let max = cross_axis_coord - overflow.side(max_side);

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

impl<'a, Element: Clone, Window: Clone>
    MiddlewareWithOptions<Element, Window, ShiftOptions<Element, Window>>
    for Shift<'a, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, ShiftOptions<Element, Window>> {
        &self.options
    }
}
