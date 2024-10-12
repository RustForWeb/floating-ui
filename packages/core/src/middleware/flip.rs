use floating_ui_utils::{
    get_alignment, get_alignment_sides, get_expanded_placements, get_opposite_axis_placements,
    get_opposite_placement, get_side, get_side_axis, Alignment, Axis, Placement,
};
use serde::{Deserialize, Serialize};

use crate::{
    detect_overflow::{detect_overflow, DetectOverflowOptions},
    middleware::arrow::{ArrowData, ARROW_NAME},
    types::{
        Derivable, DerivableFn, Middleware, MiddlewareReturn, MiddlewareState,
        MiddlewareWithOptions, Reset, ResetValue,
    },
};

/// Name of the [`Flip`] middleware.
pub const FLIP_NAME: &str = "flip";

/// Fallback strategy used by [`Flip`] middleware.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum FallbackStrategy {
    #[default]
    BestFit,
    InitialPlacement,
}

/// Options for [`Flip`] middleware.
#[derive(Clone, Debug, PartialEq)]
pub struct FlipOptions<Element: Clone> {
    /// Options for [`detect_overflow`].
    ///
    /// Defaults to [`DetectOverflowOptions::default`].
    pub detect_overflow: Option<DetectOverflowOptions<Element>>,

    /// The axis that runs along the side of the floating element. Determines whether overflow along this axis is checked to perform a flip.
    ///
    /// Defaults to `true`.
    pub main_axis: Option<bool>,

    /// The axis that runs along the alignment of the floating element. Determines whether overflow along this axis is checked to perform a flip.
    ///
    /// Defaults to `true`.
    pub cross_axis: Option<bool>,

    /// Placements to try sequentially if the preferred `placement` does not fit.
    ///
    /// Defaults to the opposite placement.
    pub fallback_placements: Option<Vec<Placement>>,

    /// What strategy to use when no placements fit.
    ///
    /// Defaults to [`FallbackStrategy::BestFit`].
    pub fallback_strategy: Option<FallbackStrategy>,

    /// Whether to allow fallback to the perpendicular axis of the preferred placement, and if so, which side direction along the axis to prefer.
    ///
    /// Defaults to [`Option::None`] (disallow fallback).
    pub fallback_axis_side_direction: Option<Alignment>,

    /// Whether to flip to placements with the opposite alignment if they fit better.
    ///
    /// Defaults to `true`.
    pub flip_alignment: Option<bool>,
}

impl<Element: Clone> FlipOptions<Element> {
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

    /// Set `fallback_placements` option.
    pub fn fallback_placements(mut self, value: Vec<Placement>) -> Self {
        self.fallback_placements = Some(value);
        self
    }

    /// Set `fallback_strategy` option.
    pub fn fallback_strategy(mut self, value: FallbackStrategy) -> Self {
        self.fallback_strategy = Some(value);
        self
    }

    /// Set `fallback_axis_side_direction` option.
    pub fn fallback_axis_side_direction(mut self, value: Alignment) -> Self {
        self.fallback_axis_side_direction = Some(value);
        self
    }

    /// Set `flip_alignment` option.
    pub fn flip_alignment(mut self, value: bool) -> Self {
        self.flip_alignment = Some(value);
        self
    }
}

impl<Element: Clone> Default for FlipOptions<Element> {
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

/// An overflow stored in [`FlipData`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlipDataOverflow {
    pub placement: Placement,
    pub overflows: Vec<f64>,
}

/// Data stored by [`Flip`] middleware.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlipData {
    pub index: usize,
    pub overflows: Vec<FlipDataOverflow>,
}

/// Flip middleware.
///
/// Optimizes the visibility of the floating element by flipping the `placement` in order to keep it in view when the preferred placement(s) will overflow the clipping boundary.
/// Alternative to [`AutoPlacement`][`crate::middleware::AutoPlacement`].
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/flip.html) for more documentation.
#[derive(PartialEq)]
pub struct Flip<'a, Element: Clone + 'static, Window: Clone> {
    options: Derivable<'a, Element, Window, FlipOptions<Element>>,
}

impl<'a, Element: Clone + 'static, Window: Clone> Flip<'a, Element, Window> {
    /// Constructs a new instance of this middleware.
    pub fn new(options: FlipOptions<Element>) -> Self {
        Flip {
            options: options.into(),
        }
    }

    /// Constructs a new instance of this middleware with derivable options.
    pub fn new_derivable(options: Derivable<'a, Element, Window, FlipOptions<Element>>) -> Self {
        Flip { options }
    }

    /// Constructs a new instance of this middleware with derivable options function.
    pub fn new_derivable_fn(
        options: DerivableFn<'a, Element, Window, FlipOptions<Element>>,
    ) -> Self {
        Flip {
            options: options.into(),
        }
    }
}

impl<Element: Clone + 'static, Window: Clone> Clone for Flip<'_, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Middleware<Element, Window>
    for Flip<'static, Element, Window>
{
    fn name(&self) -> &'static str {
        FLIP_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let options = self.options.evaluate(state.clone());

        let MiddlewareState {
            placement,
            initial_placement,
            middleware_data,
            elements,
            rects,
            platform,
            ..
        } = state;

        let data: FlipData = middleware_data.get_as(self.name()).unwrap_or(FlipData {
            index: 0,
            overflows: vec![],
        });

        let check_main_axis = options.main_axis.unwrap_or(true);
        let check_cross_axis = options.cross_axis.unwrap_or(true);
        let specified_fallback_placements = options.fallback_placements.clone();
        let fallback_strategy = options.fallback_strategy.unwrap_or_default();
        let fallback_axis_side_direction = options.fallback_axis_side_direction;
        let flip_alignment = options.flip_alignment.unwrap_or(true);

        // If a reset by the arrow was caused due to an alignment offset being added,
        // we should skip any logic now since `flip()` has already done its work.
        let arrow_data: Option<ArrowData> = middleware_data.get_as(ARROW_NAME);
        if arrow_data.map_or(false, |arrow_data| arrow_data.alignment_offset.is_some()) {
            return MiddlewareReturn {
                x: None,
                y: None,
                data: None,
                reset: None,
            };
        }

        let side = get_side(placement);
        let initial_side_axis = get_side_axis(initial_placement);
        let is_base_placement = get_alignment(initial_placement).is_none();
        let rtl = platform.is_rtl(elements.floating);

        let has_specified_fallback_placements = specified_fallback_placements.is_some();
        let mut placements =
            specified_fallback_placements.unwrap_or(match is_base_placement || !flip_alignment {
                true => vec![get_opposite_placement(initial_placement)],
                false => get_expanded_placements(initial_placement),
            });

        let has_fallback_axis_side_direction = fallback_axis_side_direction.is_some();

        if !has_specified_fallback_placements && has_fallback_axis_side_direction {
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
            options.detect_overflow.unwrap_or_default(),
        );

        let mut overflows: Vec<f64> = Vec::new();
        let mut overflows_data = data.overflows;

        if check_main_axis {
            overflows.push(overflow.side(side));
        }
        if check_cross_axis {
            let sides = get_alignment_sides(placement, rects, rtl);
            overflows.push(overflow.side(sides.0));
            overflows.push(overflow.side(sides.1));
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
                        .expect("Data should be valid JSON."),
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
            reset_placement.sort_by(|a, b| a.overflows[1].total_cmp(&b.overflows[1]));

            let mut reset_placement = reset_placement.first().map(|overflow| overflow.placement);

            // Otherwise fallback.
            if reset_placement.is_none() {
                match fallback_strategy {
                    FallbackStrategy::BestFit => {
                        let mut placement: Vec<(Placement, f64)> = overflows_data
                            .into_iter()
                            .filter(|overflow| {
                                if has_fallback_axis_side_direction {
                                    let current_side_axis = get_side_axis(overflow.placement);

                                    // Create a bias to the `y` side axis due to horizontal reading directions favoring greater width.
                                    current_side_axis == initial_side_axis
                                        || current_side_axis == Axis::Y
                                } else {
                                    true
                                }
                            })
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
                        placement.sort_by(|a, b| a.1.total_cmp(&b.1));

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

impl<Element: Clone, Window: Clone> MiddlewareWithOptions<Element, Window, FlipOptions<Element>>
    for Flip<'_, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, FlipOptions<Element>> {
        &self.options
    }
}
