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

fn convert_value_to_coords<Element: Clone, Window: Clone>(
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

/// Name of the [`Offset`] middleware.
pub const OFFSET_NAME: &str = "offset";

/// Axes configuration for [`OffsetOptions`].
#[derive(Clone, Default, Debug, PartialEq)]
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

impl OffsetOptionsValues {
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

    /// Set `alignment_axis` option.
    pub fn alignment_axis(mut self, value: f64) -> Self {
        self.alignment_axis = Some(value);
        self
    }
}

/// Options for [`Offset`] middleware.
///
/// A number (shorthand for [`main_axis`][`OffsetOptionsValues::main_axis`] or distance) or an axes configuration ([`OffsetOptionsValues`]).
#[derive(Clone, Debug, PartialEq)]
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
    pub diff_coords: Coords,
    pub placement: Placement,
}

/// Offset middleware.
///
/// Modifies the placement by translating the floating element along the specified axes.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/offset.html) for more documentation.
#[derive(PartialEq)]
pub struct Offset<'a, Element: Clone + 'static, Window: Clone> {
    options: Derivable<'a, Element, Window, OffsetOptions>,
}

impl<'a, Element: Clone, Window: Clone> Offset<'a, Element, Window> {
    /// Constructs a new instance of this middleware.
    pub fn new(options: OffsetOptions) -> Self {
        Offset {
            options: options.into(),
        }
    }

    /// Constructs a new instance of this middleware with derivable options.
    pub fn new_derivable(options: Derivable<'a, Element, Window, OffsetOptions>) -> Self {
        Offset { options }
    }

    /// Constructs a new instance of this middleware with derivable options function.
    pub fn new_derivable_fn(options: DerivableFn<'a, Element, Window, OffsetOptions>) -> Self {
        Offset {
            options: options.into(),
        }
    }
}

impl<Element: Clone + 'static, Window: Clone> Clone for Offset<'_, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Middleware<Element, Window>
    for Offset<'static, Element, Window>
{
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

        let data: Option<OffsetData> = middleware_data.get_as(self.name());

        let diff_coords = convert_value_to_coords(state, &options);

        // If the placement is the same and the arrow caused an alignment offset then we don't need to change the positioning coordinates.
        if let Some(data_placement) = data.map(|data| data.placement) {
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
                    diff_coords,
                    placement,
                })
                .expect("Data should be valid JSON."),
            ),
            reset: None,
        }
    }
}

impl<Element: Clone, Window: Clone> MiddlewareWithOptions<Element, Window, OffsetOptions>
    for Offset<'_, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, OffsetOptions> {
        &self.options
    }
}
