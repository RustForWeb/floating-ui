use floating_ui_utils::{
    get_alignment, get_alignment_sides, get_opposite_alignment_placement, get_side, Alignment,
    Placement, ALL_PLACEMENTS,
};

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

#[derive(Clone, Debug, Default)]
pub struct AutoPlacementOptions {
    detect_overflow: Option<DetectOverflowOptions>,
    cross_axis: Option<bool>,
    alignment: Option<Alignment>,
    auto_alignment: Option<bool>,
    allowed_placements: Option<Vec<Placement>>,
}

pub struct AutoPlacement {
    options: AutoPlacementOptions,
}

impl AutoPlacement {
    pub fn new(options: AutoPlacementOptions) -> Self {
        AutoPlacement { options }
    }
}

impl Middleware for AutoPlacement {
    fn name(&self) -> String {
        "autoPlacement".into()
    }

    fn compute(&self, state: MiddlewareState) -> MiddlewareReturn {
        let MiddlewareState {
            rects,
            middleware_data,
            placement,
            platform,
            elements,
            ..
        } = state;

        // TODO: support options fn

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
            state,
            self.options.detect_overflow.clone().unwrap_or_default(),
        );

        // TODO
        // let current_index = middleware_data
        let current_index = 0;
        let current_placement = placements[current_index];

        // if current_placement == null { return {} }

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

        // let all_overflows =

        let next_placement = placements.get(current_index + 1);

        // There are more placements to check.
        if let Some(next_placement) = next_placement {
            return MiddlewareReturn {
                x: None,
                y: None,
                data: None, // TODO
                reset: Some(Reset::Value(ResetValue {
                    placement: Some(*next_placement),
                    rects: None,
                })),
            };
        }

        // let placements_sorted_by_most_space = all_overflows
        // TODO

        MiddlewareReturn {
            x: None,
            y: None,
            data: None,
            reset: None,
        }
    }
}

impl MiddlewareWithOptions<AutoPlacementOptions> for AutoPlacement {
    fn options(&self) -> &AutoPlacementOptions {
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
