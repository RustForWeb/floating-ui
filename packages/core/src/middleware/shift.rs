use std::fmt::Debug;

use floating_ui_utils::{clamp, get_opposite_axis, get_side_axis, Axis, Coords, Side};
use serde::{Deserialize, Serialize};

use crate::{
    detect_overflow::{detect_overflow, DetectOverflowOptions},
    types::{Middleware, MiddlewareReturn, MiddlewareState, MiddlewareWithOptions},
};

pub trait Limiter<Element, Window> {
    fn compute(&self, state: MiddlewareState<Element, Window>) -> Coords;
}

pub struct DefaultLimiter;

impl<Element, Window> Limiter<Element, Window> for DefaultLimiter {
    fn compute(&self, state: MiddlewareState<Element, Window>) -> Coords {
        Coords {
            x: state.x,
            y: state.y,
        }
    }
}

#[derive(Clone)]
pub struct ShiftOptions<'a, Element, Window> {
    pub detect_overflow: Option<DetectOverflowOptions<'a, Element>>,
    pub main_axis: Option<bool>,
    pub cross_axis: Option<bool>,
    pub limiter: Option<&'a dyn Limiter<Element, Window>>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShiftData {
    pub x: f64,
    pub y: f64,
}

pub struct Shift<'a, Element, Window> {
    options: ShiftOptions<'a, Element, Window>,
}

impl<'a, Element, Window> Shift<'a, Element, Window> {
    pub fn new(options: ShiftOptions<'a, Element, Window>) -> Self {
        Shift { options }
    }
}

impl<'a, Element, Window> Middleware<Element, Window> for Shift<'a, Element, Window> {
    fn name(&self) -> &'static str {
        "shift"
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let MiddlewareState {
            x, y, placement, ..
        } = state;

        // TODO: support options fn

        let check_main_axis = self.options.main_axis.unwrap_or(true);
        let check_cross_axis = self.options.cross_axis.unwrap_or(false);
        let limiter = self.options.limiter.unwrap_or(&DefaultLimiter {});

        let coords = Coords { x, y };
        let overflow = detect_overflow(
            MiddlewareState {
                elements: state.elements.clone(),
                ..state
            },
            self.options.detect_overflow.clone().unwrap_or_default(),
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
                .unwrap(),
            ),
            reset: None,
        }
    }
}

impl<'a, Element, Window> MiddlewareWithOptions<ShiftOptions<'a, Element, Window>>
    for Shift<'a, Element, Window>
{
    fn options(&self) -> &ShiftOptions<'a, Element, Window> {
        &self.options
    }
}
