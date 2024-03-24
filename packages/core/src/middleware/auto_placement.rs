use std::marker::PhantomData;

use floating_ui_utils::{
    get_alignment, get_alignment_sides, get_opposite_alignment_placement, get_side, Alignment,
    Placement, ALL_PLACEMENTS,
};
use serde::{Deserialize, Serialize};

use crate::{
    detect_overflow::{detect_overflow, DetectOverflowOptions},
    types::{
        Middleware, MiddlewareReturn, MiddlewareState, MiddlewareWithOptions, Reset, ResetValue,
    },
};

pub fn get_placement_list(
    alignment: Option<Alignment>,
    auto_alignment: bool,
    allowed_placements: Vec<Placement>,
) -> Vec<Placement> {
    let allowed_placements_sorted_by_alignment: Vec<Placement> = match alignment {
        Some(alignment) => {
            let mut list = vec![];

            list.append(
                &mut allowed_placements
                    .clone()
                    .into_iter()
                    .filter(|placement| get_alignment(*placement) == Some(alignment))
                    .collect(),
            );

            list.append(
                &mut allowed_placements
                    .clone()
                    .into_iter()
                    .filter(|placement| get_alignment(*placement) != Some(alignment))
                    .collect(),
            );

            list
        }
        None => allowed_placements
            .into_iter()
            .filter(|placement| get_alignment(*placement).is_none())
            .collect(),
    };

    allowed_placements_sorted_by_alignment
        .into_iter()
        .filter(|placement| match alignment {
            Some(alignment) => {
                get_alignment(*placement) == Some(alignment)
                    || (match auto_alignment {
                        true => get_opposite_alignment_placement(*placement) != *placement,
                        false => false,
                    })
            }
            None => true,
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct AutoPlacementOptions<'a, Element> {
    pub detect_overflow: Option<DetectOverflowOptions<'a, Element>>,
    pub cross_axis: Option<bool>,
    pub alignment: Option<Alignment>,
    pub auto_alignment: Option<bool>,
    pub allowed_placements: Option<Vec<Placement>>,
}

impl<'a, Element> Default for AutoPlacementOptions<'a, Element> {
    fn default() -> Self {
        Self {
            detect_overflow: Default::default(),
            cross_axis: Default::default(),
            alignment: Default::default(),
            auto_alignment: Default::default(),
            allowed_placements: Default::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoPlacementDataOverflow {
    pub placement: Placement,
    pub overflows: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoPlacementData {
    pub index: usize,
    pub overflows: Vec<AutoPlacementDataOverflow>,
}

pub struct AutoPlacement<'a, Element, Window> {
    window: PhantomData<Window>,

    options: AutoPlacementOptions<'a, Element>,
}

impl<'a, Element, Window> AutoPlacement<'a, Element, Window> {
    pub fn new(options: AutoPlacementOptions<'a, Element>) -> Self {
        AutoPlacement {
            window: PhantomData,
            options,
        }
    }
}

impl<'a, Element, Window> Middleware<Element, Window> for AutoPlacement<'a, Element, Window> {
    fn name(&self) -> &'static str {
        "autoPlacement"
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let MiddlewareState {
            rects,
            middleware_data,
            placement,
            platform,
            elements,
            ..
        } = state;

        // TODO: support options fn

        let data: AutoPlacementData =
            middleware_data
                .get_as(self.name())
                .unwrap_or(AutoPlacementData {
                    index: 0,
                    overflows: vec![],
                });

        let cross_axis = self.options.cross_axis.unwrap_or(false);
        let alignment = self.options.alignment;
        let allowed_placements = self
            .options
            .allowed_placements
            .clone()
            .unwrap_or(Vec::from(ALL_PLACEMENTS));
        let auto_alignment = self.options.auto_alignment.unwrap_or(true);

        let placements = match alignment.is_some() || self.options.allowed_placements.is_none() {
            true => get_placement_list(alignment, auto_alignment, allowed_placements),
            false => allowed_placements,
        };

        let overflow = detect_overflow(
            MiddlewareState {
                elements: elements.clone(),
                ..state
            },
            self.options.detect_overflow.clone().unwrap_or_default(),
        );

        let current_index = data.index;
        let current_placement = placements.get(current_index);

        if let Some(current_placement) = current_placement {
            let current_placement = *current_placement;

            let alignment_sides =
                get_alignment_sides(current_placement, rects, platform.is_rtl(elements.floating));

            // Make `compute_coords` start from the right place.
            if placement != current_placement {
                return MiddlewareReturn {
                    x: None,
                    y: None,
                    data: None,
                    reset: Some(Reset::Value(ResetValue {
                        placement: Some(placements[0]),
                        rects: None,
                    })),
                };
            }

            let current_overflows = vec![
                overflow.get_side(get_side(current_placement)),
                overflow.get_side(alignment_sides.0),
                overflow.get_side(alignment_sides.1),
            ];

            let mut all_overflows = data.overflows.clone();
            all_overflows.push(AutoPlacementDataOverflow {
                placement,
                overflows: current_overflows,
            });

            let next_placement = placements.get(current_index + 1);

            // There are more placements to check.
            if let Some(next_placement) = next_placement {
                return MiddlewareReturn {
                    x: None,
                    y: None,
                    data: Some(
                        serde_json::to_value(AutoPlacementData {
                            index: current_index + 1,
                            overflows: all_overflows.clone(),
                        })
                        .unwrap(),
                    ),
                    reset: Some(Reset::Value(ResetValue {
                        placement: Some(*next_placement),
                        rects: None,
                    })),
                };
            }

            let mut placements_sorted_by_most_space: Vec<_> = all_overflows
                .clone()
                .into_iter()
                .map(|overflow| {
                    let alignment = get_alignment(overflow.placement);

                    (
                        overflow.placement,
                        match alignment.is_some() && cross_axis {
                            // Check along the main axis and main cross axis side.
                            true => overflow.overflows[0..2].iter().sum(),
                            // Check only the main axis.
                            false => overflow.overflows[0],
                        },
                        overflow.overflows,
                    )
                })
                .collect();

            placements_sorted_by_most_space.sort_by_key(|v| v.1 as i64);

            let placements_that_fit_on_each_side: Vec<_> = placements_sorted_by_most_space
                .clone()
                .into_iter()
                .filter(|overflow| {
                    // Aligned placements should not check their opposite cross axis side.
                    overflow.2[0..match get_alignment(overflow.0) {
                        Some(_) => 2,
                        None => 3,
                    }]
                        .iter()
                        .all(|v| *v <= 0.0)
                })
                .collect();

            let reset_placement = placements_that_fit_on_each_side
                .first()
                .map(|v| v.0)
                .unwrap_or(placements_sorted_by_most_space[0].0);

            if reset_placement != placement {
                return MiddlewareReturn {
                    x: None,
                    y: None,
                    data: Some(
                        serde_json::to_value(AutoPlacementData {
                            index: current_index + 1,
                            overflows: all_overflows,
                        })
                        .unwrap(),
                    ),
                    reset: Some(Reset::Value(ResetValue {
                        placement: Some(reset_placement),
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

impl<'a, Element, Window> MiddlewareWithOptions<AutoPlacementOptions<'a, Element>>
    for AutoPlacement<'a, Element, Window>
{
    fn options(&self) -> &AutoPlacementOptions<'a, Element> {
        &self.options
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_placement() {
        assert_eq!(
            get_placement_list(
                None,
                false,
                vec![
                    Placement::Top,
                    Placement::Bottom,
                    Placement::Left,
                    Placement::Right,
                    Placement::TopStart,
                    Placement::RightEnd,
                ]
            ),
            vec![
                Placement::Top,
                Placement::Bottom,
                Placement::Left,
                Placement::Right,
            ]
        )
    }

    #[test]
    fn test_start_alignment_without_auto_alignment() {
        assert_eq!(
            get_placement_list(
                Some(Alignment::Start),
                false,
                vec![
                    Placement::Top,
                    Placement::Bottom,
                    Placement::Left,
                    Placement::Right,
                    Placement::TopStart,
                    Placement::RightEnd,
                    Placement::LeftStart,
                ]
            ),
            vec![Placement::TopStart, Placement::LeftStart]
        )
    }

    #[test]
    fn test_start_alignment_with_auto_alignment() {
        assert_eq!(
            get_placement_list(
                Some(Alignment::Start),
                true,
                vec![
                    Placement::Top,
                    Placement::Bottom,
                    Placement::Left,
                    Placement::Right,
                    Placement::TopStart,
                    Placement::RightEnd,
                    Placement::LeftStart,
                ]
            ),
            vec![
                Placement::TopStart,
                Placement::LeftStart,
                Placement::RightEnd,
            ]
        )
    }

    #[test]
    fn test_end_alignment_without_auto_alignment() {
        assert_eq!(
            get_placement_list(
                Some(Alignment::End),
                false,
                vec![
                    Placement::Top,
                    Placement::Bottom,
                    Placement::Left,
                    Placement::Right,
                    Placement::TopStart,
                    Placement::RightEnd,
                    Placement::LeftStart,
                ]
            ),
            vec![Placement::RightEnd,]
        )
    }

    #[test]
    fn test_end_alignment_with_auto_alignment() {
        assert_eq!(
            get_placement_list(
                Some(Alignment::End),
                true,
                vec![
                    Placement::Top,
                    Placement::Bottom,
                    Placement::Left,
                    Placement::Right,
                    Placement::TopStart,
                    Placement::RightEnd,
                    Placement::LeftStart,
                ]
            ),
            vec![
                Placement::RightEnd,
                Placement::TopStart,
                Placement::LeftStart
            ]
        )
    }
}
