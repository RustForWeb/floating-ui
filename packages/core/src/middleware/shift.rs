use std::fmt::Debug;

use dyn_derive::dyn_trait;
use floating_ui_utils::{clamp, get_opposite_axis, get_side_axis, Axis, Coords, Side};
use serde::{Deserialize, Serialize};

use crate::{
    detect_overflow::{detect_overflow, DetectOverflowOptions},
    middleware::{OffsetData, OFFSET_NAME},
    types::{
        Derivable, DerivableFn, Middleware, MiddlewareReturn, MiddlewareState,
        MiddlewareWithOptions,
    },
};

/// Name of the [`Shift`] middleware.
pub const SHIFT_NAME: &str = "shift";

/// Limiter used by [`Shift`] middleware. Limits the shifting done in order to prevent detachment.
#[dyn_trait]
pub trait Limiter<Element: Clone + 'static, Window: Clone + 'static>: Clone + PartialEq {
    fn compute(&self, state: MiddlewareState<Element, Window>) -> Coords;
}

/// Options for [`Shift`] middleware.
#[derive(Clone, PartialEq)]
pub struct ShiftOptions<Element: Clone + 'static, Window: Clone + 'static> {
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

/// Enabled sides stored in [`ShiftData`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ShiftDataEnabled {
    pub x: bool,
    pub y: bool,
}

impl ShiftDataEnabled {
    pub fn set_axis(mut self, axis: Axis, enabled: bool) -> Self {
        match axis {
            Axis::X => {
                self.x = enabled;
            }
            Axis::Y => {
                self.y = enabled;
            }
        }
        self
    }
}

/// Data stored by [`Shift`] middleware.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShiftData {
    pub x: f64,
    pub y: f64,
    pub enabled: ShiftDataEnabled,
}

/// Shift middleware.
///
/// Optimizes the visibility of the floating element by shifting it in order to keep it in view when it will overflow the clipping boundary.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/shift.html) for more documentation.
#[derive(PartialEq)]
pub struct Shift<'a, Element: Clone + 'static, Window: Clone + 'static> {
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

impl<Element: Clone, Window: Clone> Clone for Shift<'_, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}

impl<Element: Clone + PartialEq + 'static, Window: Clone + PartialEq + 'static>
    Middleware<Element, Window> for Shift<'static, Element, Window>
{
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
            let min_side = match cross_axis {
                Axis::X => Side::Left,
                Axis::Y => Side::Top,
            };
            let max_side = match cross_axis {
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
                    enabled: ShiftDataEnabled::default()
                        .set_axis(main_axis, check_main_axis)
                        .set_axis(cross_axis, check_cross_axis),
                })
                .expect("Data should be valid JSON."),
            ),
            reset: None,
        }
    }
}

impl<Element: Clone, Window: Clone>
    MiddlewareWithOptions<Element, Window, ShiftOptions<Element, Window>>
    for Shift<'_, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, ShiftOptions<Element, Window>> {
        &self.options
    }
}

/// Default [`Limiter`], which doesn't limit shifting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DefaultLimiter;

impl<Element: Clone + 'static, Window: Clone + 'static> Limiter<Element, Window>
    for DefaultLimiter
{
    fn compute(&self, state: MiddlewareState<Element, Window>) -> Coords {
        Coords {
            x: state.x,
            y: state.y,
        }
    }
}

/// Axes configuration for [`LimitShiftOffset`].
#[derive(Clone, Default, Debug, PartialEq)]
pub struct LimitShiftOffsetValues {
    pub main_axis: Option<f64>,

    pub cross_axis: Option<f64>,
}

impl LimitShiftOffsetValues {
    /// Set `main_axis` option.
    pub fn main_axis(mut self, value: f64) -> Self {
        self.main_axis = Some(value);
        self
    }

    /// Set `cross_axis` option.
    pub fn cross_axis(mut self, value: f64) -> Self {
        self.cross_axis = Some(value);
        self
    }
}

/// Offset configuration for [`LimitShiftOptions`].
#[derive(Clone, Debug, PartialEq)]
pub enum LimitShiftOffset {
    Value(f64),
    Values(LimitShiftOffsetValues),
}

impl Default for LimitShiftOffset {
    fn default() -> Self {
        LimitShiftOffset::Value(0.0)
    }
}

/// Options for [`LimitShift`] limiter.
#[derive(Clone, PartialEq)]
pub struct LimitShiftOptions<'a, Element: Clone + 'static, Window: Clone> {
    pub offset: Option<Derivable<'a, Element, Window, LimitShiftOffset>>,

    pub main_axis: Option<bool>,

    pub cross_axis: Option<bool>,
}

impl<'a, Element: Clone, Window: Clone> LimitShiftOptions<'a, Element, Window> {
    /// Set `offset` option.
    pub fn offset(mut self, value: LimitShiftOffset) -> Self {
        self.offset = Some(value.into());
        self
    }

    /// Set `offset` option with derivable offset.
    pub fn offset_derivable(
        mut self,
        value: Derivable<'a, Element, Window, LimitShiftOffset>,
    ) -> Self {
        self.offset = Some(value);
        self
    }

    /// Set `offset` option with derivable offset function.
    pub fn offset_derivable_fn(
        mut self,
        value: DerivableFn<'a, Element, Window, LimitShiftOffset>,
    ) -> Self {
        self.offset = Some(value.into());
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
}

impl<Element: Clone + 'static, Window: Clone> Default for LimitShiftOptions<'_, Element, Window> {
    fn default() -> Self {
        Self {
            offset: Default::default(),
            main_axis: Default::default(),
            cross_axis: Default::default(),
        }
    }
}

/// Built-in [`Limiter`], that will stop [`Shift`] at a certain point.
#[derive(Clone, Default, PartialEq)]
pub struct LimitShift<'a, Element: Clone + 'static, Window: Clone> {
    options: LimitShiftOptions<'a, Element, Window>,
}

impl<'a, Element: Clone, Window: Clone> LimitShift<'a, Element, Window> {
    pub fn new(options: LimitShiftOptions<'a, Element, Window>) -> Self {
        LimitShift { options }
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Limiter<Element, Window>
    for LimitShift<'static, Element, Window>
{
    fn compute(&self, state: MiddlewareState<Element, Window>) -> Coords {
        let MiddlewareState {
            x,
            y,
            placement,
            rects,
            middleware_data,
            ..
        } = state;

        let offset = self
            .options
            .offset
            .clone()
            .unwrap_or(Derivable::Value(LimitShiftOffset::default()));
        let check_main_axis = self.options.main_axis.unwrap_or(true);
        let check_cross_axis = self.options.cross_axis.unwrap_or(true);

        let coords = Coords { x, y };
        let cross_axis = get_side_axis(placement);
        let main_axis = get_opposite_axis(cross_axis);

        let mut main_axis_coord = coords.axis(main_axis);
        let mut cross_axis_coord = coords.axis(cross_axis);

        let raw_offset = offset.evaluate(state.clone());
        let (computed_main_axis, computed_cross_axis) = match raw_offset {
            LimitShiftOffset::Value(value) => (value, 0.0),
            LimitShiftOffset::Values(values) => (
                values.main_axis.unwrap_or(0.0),
                values.cross_axis.unwrap_or(0.0),
            ),
        };

        if check_main_axis {
            let len = main_axis.length();
            let limit_min =
                rects.reference.axis(main_axis) - rects.floating.length(len) + computed_main_axis;
            let limit_max =
                rects.reference.axis(main_axis) + rects.reference.length(len) - computed_main_axis;

            main_axis_coord = clamp(limit_min, main_axis_coord, limit_max);
        }

        if check_cross_axis {
            let len = main_axis.length();
            let is_origin_side = match placement.side() {
                Side::Top | Side::Left => true,
                Side::Bottom | Side::Right => false,
            };

            let data: Option<OffsetData> = middleware_data.get_as(OFFSET_NAME);
            let data_cross_axis = data.map_or(0.0, |data| data.diff_coords.axis(cross_axis));

            let limit_min = rects.reference.axis(cross_axis) - rects.floating.length(len)
                + match is_origin_side {
                    true => data_cross_axis,
                    false => 0.0,
                }
                + match is_origin_side {
                    true => 0.0,
                    false => computed_cross_axis,
                };
            let limit_max = rects.reference.axis(cross_axis)
                + rects.reference.length(len)
                + match is_origin_side {
                    true => 0.0,
                    false => data_cross_axis,
                }
                - match is_origin_side {
                    true => computed_cross_axis,
                    false => 0.0,
                };

            cross_axis_coord = clamp(limit_min, cross_axis_coord, limit_max);
        }

        Coords {
            x: match main_axis {
                Axis::X => main_axis_coord,
                Axis::Y => cross_axis_coord,
            },
            y: match main_axis {
                Axis::X => cross_axis_coord,
                Axis::Y => main_axis_coord,
            },
        }
    }
}
