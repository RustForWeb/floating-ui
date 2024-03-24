use std::marker::PhantomData;

use floating_ui_utils::{
    get_alignment, get_alignment_sides, get_expanded_placements, get_opposite_axis_placements,
    get_opposite_placement, get_side, Alignment, Placement,
};
use serde::{Deserialize, Serialize};

use crate::{
    detect_overflow::{detect_overflow, DetectOverflowOptions},
    types::{Middleware, MiddlewareReturn, MiddlewareState, MiddlewareWithOptions},
    Reset, ResetValue,
};

#[derive(Copy, Clone, Debug, Default)]
pub enum FallbackStrategy {
    #[default]
    BestFit,
    InitialPlacement,
}

#[derive(Clone, Debug)]
pub struct FlipOptions<'a, Element> {
    pub detect_overflow: Option<DetectOverflowOptions<'a, Element>>,
    pub main_axis: Option<bool>,
    pub cross_axis: Option<bool>,
    pub fallback_placements: Option<Vec<Placement>>,
    pub fallback_strategy: Option<FallbackStrategy>,
    pub fallback_axis_side_direction: Option<Alignment>,
    pub flip_alignment: Option<bool>,
}

impl<'a, Element> Default for FlipOptions<'a, Element> {
    fn default() -> Self {
        Self {
            detect_overflow: Default::default(),
            main_axis: Default::default(),
            cross_axis: Default::default(),
            fallback_placements: Default::default(),
            fallback_strategy: Default::default(),
            fallback_axis_side_direction: Default::default(),
            flip_alignment: Default::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlipDataOverflow {
    pub placement: Placement,
    pub overflows: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlipData {
    pub index: usize,
    pub overflows: Vec<FlipDataOverflow>,
}

pub struct Flip<'a, Element, Window> {
    window: PhantomData<Window>,

    options: FlipOptions<'a, Element>,
}

impl<'a, Element, Window> Flip<'a, Element, Window> {
    pub fn new(options: FlipOptions<'a, Element>) -> Self {
        Flip {
            window: PhantomData,
            options,
        }
    }
}

impl<'a, Element, Window> Middleware<Element, Window> for Flip<'a, Element, Window> {
    fn name(&self) -> &'static str {
        "flip"
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let MiddlewareState {
            placement,
            initial_placement,
            middleware_data,
            elements,
            rects,
            platform,
            ..
        } = state;

        // TODO: support options fn

        let data: FlipData = middleware_data.get_as(self.name()).unwrap_or(FlipData {
            index: 0,
            overflows: vec![],
        });

        let check_main_axis = self.options.main_axis.unwrap_or(true);
        let check_cross_axis = self.options.cross_axis.unwrap_or(true);
        let specified_fallback_placements = self.options.fallback_placements.clone();
        let fallback_strategy = self.options.fallback_strategy.unwrap_or_default();
        let fallback_axis_side_direction = self.options.fallback_axis_side_direction;
        let flip_alignment = self.options.flip_alignment.unwrap_or(true);

        // TODO: arrow check

        let side = get_side(placement);
        let is_base_placement = get_alignment(initial_placement).is_none();
        let rtl = platform.is_rtl(elements.floating);

        let has_specified_fallback_placements = specified_fallback_placements.is_some();
        let mut placements =
            specified_fallback_placements.unwrap_or(match is_base_placement || !flip_alignment {
                true => vec![get_opposite_placement(initial_placement)],
                false => get_expanded_placements(initial_placement),
            });

        if !has_specified_fallback_placements && fallback_axis_side_direction.is_some() {
            placements.append(&mut get_opposite_axis_placements(
                initial_placement,
                flip_alignment,
                fallback_axis_side_direction,
                rtl,
            ));
        }

        placements.insert(0, initial_placement);

        let overflow = detect_overflow(
            MiddlewareState {
                elements: elements.clone(),
                ..state
            },
            self.options.detect_overflow.clone().unwrap_or_default(),
        );

        let mut overflows: Vec<f64> = Vec::new();
        let mut overflows_data = data.overflows;

        if check_main_axis {
            overflows.push(overflow.get_side(side));
        }
        if check_cross_axis {
            let sides = get_alignment_sides(placement, rects, rtl);
            overflows.push(overflow.get_side(sides.0));
            overflows.push(overflow.get_side(sides.1));
        }

        overflows_data.push(FlipDataOverflow {
            placement,
            overflows: overflows.clone(),
        });

        // One or more sides is overflowing.
        if !overflows.into_iter().all(|side| side <= 0.0) {
            let next_index = data.index + 1;
            let next_placement = placements.get(next_index);

            if let Some(next_placement) = next_placement {
                // Try next placement and re-run the lifecycle.
                return MiddlewareReturn {
                    x: None,
                    y: None,
                    data: Some(
                        serde_json::to_value(FlipData {
                            index: next_index,
                            overflows: overflows_data,
                        })
                        .unwrap(),
                    ),
                    reset: Some(Reset::Value(ResetValue {
                        placement: Some(*next_placement),
                        rects: None,
                    })),
                };
            }

            // First, find the candidates that fit on the main axis side of overflow, then find the placement that fits the best on the main cross axis side.
            let mut reset_placement: Vec<&FlipDataOverflow> = overflows_data
                .iter()
                .filter(|overflow| overflow.overflows[0] <= 0.0)
                .collect();
            reset_placement.sort_by_key(|overflow| overflow.overflows[1] as i64);

            let mut reset_placement = reset_placement.first().map(|overflow| overflow.placement);

            // Otherwise fallback.
            if reset_placement.is_none() {
                match fallback_strategy {
                    FallbackStrategy::BestFit => {
                        let mut placement: Vec<(Placement, f64)> = overflows_data
                            .into_iter()
                            .map(|overflow| {
                                (
                                    overflow.placement,
                                    overflow
                                        .overflows
                                        .into_iter()
                                        .filter(|overflow| *overflow > 0.0)
                                        .sum::<f64>(),
                                )
                            })
                            .collect();
                        placement.sort_by_key(|v| v.1 as i64);

                        let placement = placement.first().map(|v| v.0);
                        if placement.is_some() {
                            reset_placement = placement;
                        }
                    }
                    FallbackStrategy::InitialPlacement => {
                        reset_placement = Some(initial_placement);
                    }
                }
            }

            if placement != reset_placement.expect("Reset placement is not none.") {
                return MiddlewareReturn {
                    x: None,
                    y: None,
                    data: None,
                    reset: Some(Reset::Value(ResetValue {
                        placement: reset_placement,
                        rects: None,
                    })),
                };
            }
        }

        MiddlewareReturn {
            x: None,
            y: None,
            data: None,
            reset: None,
        }
    }
}

impl<'a, Element, Window> MiddlewareWithOptions<FlipOptions<'a, Element>>
    for Flip<'a, Element, Window>
{
    fn options(&self) -> &FlipOptions<'a, Element> {
        &self.options
    }
}
