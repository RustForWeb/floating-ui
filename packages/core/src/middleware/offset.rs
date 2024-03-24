use floating_ui_utils::{
    get_alignment, get_side, get_side_axis, Alignment, Axis, Coords, Placement, Side,
};
use serde::{Deserialize, Serialize};

use crate::{
    middleware::{ArrowData, ARROW_NAME},
    types::{
        Derivable, DerivableFn, Middleware, MiddlewareReturn, MiddlewareState,
        MiddlewareWithOptions,
    },
};

/// Name of the [`Offset`] middleware.
const OFFSET_NAME: &str = "offset";

fn convert_value_to_coords<Element, Window>(
    state: MiddlewareState<Element, Window>,
    options: &OffsetOptions,
) -> Coords {
    let MiddlewareState {
        placement,
        platform,
        elements,
        ..
    } = state;

    let rtl = platform.is_rtl(elements.floating).unwrap_or(false);
    let side = get_side(placement);
    let alignment = get_alignment(placement);
    let is_vertical = get_side_axis(placement) == Axis::Y;
    let main_axis_multi = match side {
        Side::Left | Side::Top => -1.0,
        Side::Right | Side::Bottom => 1.0,
    };
    let cross_axis_multi = match rtl && is_vertical {
        true => -1.0,
        false => 1.0,
    };

    let (main_axis, mut cross_axis, alignment_axis): (f64, f64, Option<f64>) = match options {
        OffsetOptions::Value(value) => (*value, 0.0, None),
        OffsetOptions::Values(values) => (
            values.main_axis.unwrap_or(0.0),
            values.cross_axis.unwrap_or(0.0),
            values.alignment_axis,
        ),
    };

    if let Some(alignment) = alignment {
        if let Some(alignment_axis) = alignment_axis {
            cross_axis = match alignment {
                Alignment::Start => alignment_axis,
                Alignment::End => alignment_axis * -1.0,
            };
        }
    }

    match is_vertical {
        true => Coords {
            x: cross_axis * cross_axis_multi,
            y: main_axis * main_axis_multi,
        },
        false => Coords {
            x: main_axis * main_axis_multi,
            y: cross_axis * cross_axis_multi,
        },
    }
}

/// Axes configuration for [`OffsetOptions`].
#[derive(Clone, Default, Debug)]
pub struct OffsetOptionsValues {
    /// The axis that runs along the side of the floating element. Represents the distance (gutter or margin) between the reference and floating element.
    ///
    /// Defaults to `0`.
    pub main_axis: Option<f64>,

    /// The axis that runs along the alignment of the floating element. Represents the skidding between the reference and floating element.
    ///
    /// Defaults to `0`.
    pub cross_axis: Option<f64>,

    /// The same axis as [`cross_axis`][`Self::cross_axis`] but applies only to aligned placements and inverts the [`End`][`floating_ui_utils::Alignment::End`] alignment.
    /// When set to a number, it overrides the [`cross_axis`][`Self::cross_axis`] value.
    ///
    /// A positive number will move the floating element in the direction of the opposite edge to the one that is aligned, while a negative number the reverse.
    ///
    /// Defaults to [`Option::None`].
    pub alignment_axis: Option<f64>,
}

/// Options for [`Offset`] middleware.
///
/// A number (shorthand for [`main_axis`][`OffsetOptionsValues::main_axis`] or distance) or an axes configuration ([`OffsetOptionsValues`]).
#[derive(Clone, Debug)]
pub enum OffsetOptions {
    Value(f64),
    Values(OffsetOptionsValues),
}

impl Default for OffsetOptions {
    fn default() -> Self {
        OffsetOptions::Value(0.0)
    }
}

/// Data stored by [`Offset`] middleware.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OffsetData {
    pub diff_coords: Option<Coords>,
    pub placement: Option<Placement>,
}

/// Modifies the placement by translating the floating element along the specified axes.
///
/// See <https://floating-ui.com/docs/offset> for the original documentation.
pub struct Offset<Element, Window> {
    options: Derivable<Element, Window, OffsetOptions>,
}

impl<Element, Window> Offset<Element, Window> {
    /// Constructs a new instance of this middleware.
    pub fn new(options: OffsetOptions) -> Self {
        Offset {
            options: options.into(),
        }
    }

    /// Constructs a new instance of this middleware with derivable options.
    pub fn new_derivable(options: DerivableFn<Element, Window, OffsetOptions>) -> Self {
        Offset {
            options: options.into(),
        }
    }
}

impl<Element, Window> Middleware<Element, Window> for Offset<Element, Window> {
    fn name(&self) -> &'static str {
        OFFSET_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let options = self.options.evaluate(state.clone());

        let MiddlewareState {
            x,
            y,
            placement,
            middleware_data,
            ..
        } = state;

        let data: OffsetData = middleware_data.get_as(self.name()).unwrap_or(OffsetData {
            diff_coords: None,
            placement: None,
        });

        let diff_coords = convert_value_to_coords(state, &options);

        // If the placement is the same and the arrow caused an alignment offset then we don't need to change the positioning coordinates.
        if let Some(data_placement) = data.placement {
            if placement == data_placement {
                let arrow_data: Option<ArrowData> = middleware_data.get_as(ARROW_NAME);
                if arrow_data.map_or(false, |arrow_data| arrow_data.alignment_offset.is_some()) {
                    return MiddlewareReturn {
                        x: None,
                        y: None,
                        data: None,
                        reset: None,
                    };
                }
            }
        }

        MiddlewareReturn {
            x: Some(x + diff_coords.x),
            y: Some(y + diff_coords.y),
            data: Some(
                serde_json::to_value(OffsetData {
                    diff_coords: Some(diff_coords),
                    placement: Some(placement),
                })
                .unwrap(),
            ),
            reset: None,
        }
    }
}

impl<Element, Window> MiddlewareWithOptions<Element, Window, OffsetOptions>
    for Offset<Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, OffsetOptions> {
        &self.options
    }
}
