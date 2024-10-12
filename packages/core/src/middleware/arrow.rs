use floating_ui_utils::{
    clamp, get_alignment, get_alignment_axis, get_axis_length, get_padding_object, Axis, Coords,
    OwnedElementOrWindow, Padding, Side,
};
use serde::{Deserialize, Serialize};

use crate::types::{
    Derivable, DerivableFn, Middleware, MiddlewareReturn, MiddlewareState, MiddlewareWithOptions,
};

/// Name of the [`Arrow`] middleware.
pub const ARROW_NAME: &str = "arrow";

/// Options for [`Arrow`].
#[derive(Clone, Debug, PartialEq)]
pub struct ArrowOptions<Element: Clone> {
    /// The arrow element to be positioned.
    pub element: Element,

    /// The padding between the arrow element and the floating element edges.
    /// Useful when the floating element has rounded corners.
    ///
    /// Defaults to `0` on all sides.
    pub padding: Option<Padding>,
}

impl<Element: Clone> ArrowOptions<Element> {
    pub fn new(element: Element) -> Self {
        ArrowOptions {
            element,
            padding: None,
        }
    }

    /// Set `element` option.
    pub fn element(mut self, value: Element) -> Self {
        self.element = value;
        self
    }

    /// Set `padding` option.
    pub fn padding(mut self, value: Padding) -> Self {
        self.padding = Some(value);
        self
    }
}

/// Data stored by [`Arrow`] middleware.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArrowData {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub center_offset: f64,
    pub alignment_offset: Option<f64>,
}

/// Arrow middleware.
///
/// Provides data to position an inner element of the floating element so that it appears centered to the reference element.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/arrow.html) for more documentation.
#[derive(PartialEq)]
pub struct Arrow<'a, Element: Clone + 'static, Window: Clone> {
    options: Derivable<'a, Element, Window, ArrowOptions<Element>>,
}

impl<'a, Element: Clone + 'static, Window: Clone> Arrow<'a, Element, Window> {
    /// Constructs a new instance of this middleware.
    pub fn new(options: ArrowOptions<Element>) -> Self {
        Arrow {
            options: options.into(),
        }
    }

    /// Constructs a new instance of this middleware with derivable options.
    pub fn new_derivable(options: Derivable<'a, Element, Window, ArrowOptions<Element>>) -> Self {
        Arrow { options }
    }

    /// Constructs a new instance of this middleware with derivable options function.
    pub fn new_derivable_fn(
        options: DerivableFn<'a, Element, Window, ArrowOptions<Element>>,
    ) -> Self {
        Arrow {
            options: options.into(),
        }
    }
}

impl<Element: Clone + 'static, Window: Clone> Clone for Arrow<'_, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Middleware<Element, Window>
    for Arrow<'static, Element, Window>
{
    fn name(&self) -> &'static str {
        ARROW_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let options = self.options.evaluate(state.clone());

        let MiddlewareState {
            x,
            y,
            placement,
            middleware_data,
            elements,
            rects,
            platform,
            ..
        } = state;

        let data: Option<ArrowData> = middleware_data.get_as(self.name());

        let padding_object = get_padding_object(options.padding.unwrap_or(Padding::All(0.0)));
        let coords = Coords { x, y };
        let axis = get_alignment_axis(placement);
        let length = get_axis_length(axis);
        let arrow_dimensions = platform.get_dimensions(&options.element);
        let min_prop = match axis {
            Axis::X => Side::Left,
            Axis::Y => Side::Top,
        };
        let max_prop = match axis {
            Axis::X => Side::Right,
            Axis::Y => Side::Bottom,
        };

        let start_diff = coords.axis(axis) - rects.reference.axis(axis);
        let end_diff = rects.reference.length(length) + rects.reference.axis(axis)
            - coords.axis(axis)
            - rects.floating.length(length);

        let arrow_offset_parent = platform.get_offset_parent(&options.element);
        let client_size = arrow_offset_parent
            .and_then(|arrow_offset_parent| match arrow_offset_parent {
                OwnedElementOrWindow::Element(element) => {
                    platform.get_client_length(&element, length)
                }
                OwnedElementOrWindow::Window(_) => {
                    platform.get_client_length(elements.floating, length)
                }
            })
            .unwrap_or(rects.floating.length(length));

        let center_to_reference = end_diff / 2.0 - start_diff / 2.0;

        // If the padding is large enough that it causes the arrow to no longer be centered, modify the padding so that it is centered.
        let largest_possible_padding =
            client_size / 2.0 - arrow_dimensions.length(length) / 2.0 - 1.0;
        let min_padding = padding_object.side(min_prop).min(largest_possible_padding);
        let max_padding = padding_object.side(max_prop).min(largest_possible_padding);

        // Make sure the arrow doesn't overflow the floating element if the center point is outside the floating element's bounds.
        let min = min_padding;
        let max = client_size - arrow_dimensions.length(length) - max_padding;
        let center =
            client_size / 2.0 - arrow_dimensions.length(length) / 2.0 + center_to_reference;
        let offset = clamp(min, center, max);

        // If the reference is small enough that the arrow's padding causes it to to point to nothing for an aligned placement, adjust the offset of the floating element itself.
        // To ensure `shift()` continues to take action, a single reset is performed when this is true.
        let should_add_offset = data.is_none()
            && get_alignment(placement).is_some()
            && center != offset
            && rects.reference.length(length) / 2.0
                - (match center < min {
                    true => min_padding,
                    false => max_padding,
                })
                - arrow_dimensions.length(length) / 2.0
                < 0.0;
        let alignment_offset = match should_add_offset {
            true => match center < min {
                true => center - min,
                false => center - max,
            },
            false => 0.0,
        };

        MiddlewareReturn {
            x: match axis {
                Axis::X => Some(coords.axis(axis) + alignment_offset),
                Axis::Y => None,
            },
            y: match axis {
                Axis::X => None,
                Axis::Y => Some(coords.axis(axis) + alignment_offset),
            },
            data: Some(
                serde_json::to_value(ArrowData {
                    x: match axis {
                        Axis::X => Some(offset),
                        Axis::Y => None,
                    },
                    y: match axis {
                        Axis::X => None,
                        Axis::Y => Some(offset),
                    },
                    center_offset: center - offset - alignment_offset,
                    alignment_offset: match should_add_offset {
                        true => Some(alignment_offset),
                        false => None,
                    },
                })
                .expect("Data should be valid JSON."),
            ),
            reset: None,
        }
    }
}

impl<Element: Clone, Window: Clone> MiddlewareWithOptions<Element, Window, ArrowOptions<Element>>
    for Arrow<'_, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, ArrowOptions<Element>> {
        &self.options
    }
}
