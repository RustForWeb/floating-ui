use std::ptr;

use floating_ui_utils::{get_side_axis, Alignment, Axis, Rect, Side};

use crate::{
    detect_overflow::{detect_overflow, DetectOverflowOptions},
    types::{
        Derivable, DerivableFn, Middleware, MiddlewareReturn, MiddlewareState,
        MiddlewareWithOptions, ResetRects, ResetValue,
    },
};

use super::SHIFT_NAME;

/// Name of the [`Size`] middleware.
pub const SIZE_NAME: &str = "size";

/// State passed to [`SizeOptions::apply`].
#[derive(Clone)]
pub struct ApplyState<'a, Element: Clone + 'static, Window: Clone> {
    pub state: MiddlewareState<'a, Element, Window>,
    pub available_width: f64,
    pub available_height: f64,
}

pub type ApplyFn<Element, Window> = dyn Fn(ApplyState<Element, Window>);

/// Options for [`Size`] middleware.
#[derive(Clone)]
pub struct SizeOptions<'a, Element: Clone + 'static, Window: Clone> {
    /// Options for [`detect_overflow`].
    ///
    /// Defaults to [`DetectOverflowOptions::default`].
    pub detect_overflow: Option<DetectOverflowOptions<Element>>,

    /// Function that is called to perform style mutations to the floating element to change its size.
    pub apply: Option<&'a ApplyFn<Element, Window>>,
}

impl<'a, Element: Clone, Window: Clone> SizeOptions<'a, Element, Window> {
    pub fn new() -> Self {
        SizeOptions {
            detect_overflow: None,
            apply: None,
        }
    }

    /// Set `detect_overflow` option.
    pub fn detect_overflow(mut self, value: DetectOverflowOptions<Element>) -> Self {
        self.detect_overflow = Some(value);
        self
    }

    /// Set `apply` option.
    pub fn apply(mut self, value: &'a ApplyFn<Element, Window>) -> Self {
        self.apply = Some(value);
        self
    }
}

impl<'a, Element: Clone, Window: Clone> Default for SizeOptions<'a, Element, Window> {
    fn default() -> Self {
        Self {
            detect_overflow: Default::default(),
            apply: Default::default(),
        }
    }
}

impl<'a, Element: Clone + PartialEq, Window: Clone + PartialEq> PartialEq
    for SizeOptions<'a, Element, Window>
{
    fn eq(&self, other: &Self) -> bool {
        self.detect_overflow == other.detect_overflow
            && match (self.apply, other.apply) {
                (Some(a), Some(b)) => ptr::eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

/// Provides data that allows you to change the size of the floating element -
/// for instance, prevent it from overflowing the clipping boundary or match the width of the reference element.
///
/// See <https://floating-ui.com/docs/size> for the original documentation.
#[derive(PartialEq)]
pub struct Size<'a, Element: Clone + 'static, Window: Clone> {
    options: Derivable<'a, Element, Window, SizeOptions<'a, Element, Window>>,
}

impl<'a, Element: Clone + 'static, Window: Clone> Size<'a, Element, Window> {
    /// Constructs a new instance of this middleware.
    pub fn new(options: SizeOptions<'a, Element, Window>) -> Self {
        Size {
            options: options.into(),
        }
    }

    /// Constructs a new instance of this middleware with derivable options.
    pub fn new_derivable(
        options: Derivable<'a, Element, Window, SizeOptions<'a, Element, Window>>,
    ) -> Self {
        Size { options }
    }

    /// Constructs a new instance of this middleware with derivable options function.
    pub fn new_derivable_fn(
        options: DerivableFn<'a, Element, Window, SizeOptions<'a, Element, Window>>,
    ) -> Self {
        Size {
            options: options.into(),
        }
    }
}

impl<'a, Element: Clone, Window: Clone> Clone for Size<'a, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Middleware<Element, Window>
    for Size<'static, Element, Window>
{
    fn name(&self) -> &'static str {
        SIZE_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let options = self.options.evaluate(state.clone());

        let MiddlewareState {
            placement,
            elements,
            rects,
            platform,
            ..
        } = state;

        let overflow = detect_overflow(
            MiddlewareState {
                elements: elements.clone(),
                ..state
            },
            options.detect_overflow.unwrap_or_default(),
        );
        let side = placement.side();
        let alignment = placement.alignment();
        let is_y_axis = get_side_axis(placement) == Axis::Y;
        let Rect { width, height, .. } = rects.floating;

        let height_side;
        let width_side;

        match side {
            Side::Top | Side::Bottom => {
                height_side = side;
                width_side = match alignment {
                    Some(alignment) => match alignment
                        == match platform.is_rtl(elements.floating) {
                            Some(true) => Alignment::Start,
                            _ => Alignment::End,
                        } {
                        true => Side::Left,
                        false => Side::Right,
                    },
                    None => Side::Right,
                };
            }
            Side::Right | Side::Left => {
                width_side = side;
                height_side = match alignment {
                    Some(Alignment::End) => Side::Top,
                    _ => Side::Bottom,
                };
            }
        }

        let maximum_clipping_height = height - overflow.top - overflow.bottom;
        let maximum_clipping_width = width - overflow.left - overflow.right;

        let overflow_available_height =
            maximum_clipping_height.min(height - overflow.side(height_side));
        let overflow_available_width =
            maximum_clipping_width.min(width - overflow.side(width_side));

        let no_shift = state.middleware_data.get(SHIFT_NAME).is_none();

        let mut available_height = overflow_available_height;
        let mut available_width = overflow_available_width;

        if is_y_axis {
            available_width = match alignment.is_some() || no_shift {
                true => overflow_available_width.min(maximum_clipping_width),
                false => maximum_clipping_width,
            };
        } else {
            available_height = match alignment.is_some() || no_shift {
                true => overflow_available_height.min(maximum_clipping_height),
                false => maximum_clipping_height,
            }
        }

        if no_shift && alignment.is_none() {
            let x_min = overflow.left.max(0.0);
            let x_max = overflow.right.max(0.0);
            let y_min = overflow.top.max(0.0);
            let y_max = overflow.bottom.max(0.0);

            if is_y_axis {
                available_width = width
                    - 2.0
                        * (match x_min != 0.0 || x_max != 0.0 {
                            true => x_min + x_max,
                            false => overflow.left.max(overflow.right),
                        });
            } else {
                available_height = height
                    - 2.0
                        * (match y_min != 0.0 || y_max != 0.0 {
                            true => y_min + y_max,
                            false => overflow.top.max(overflow.bottom),
                        });
            }
        }

        if let Some(apply) = options.apply {
            apply(ApplyState {
                state: MiddlewareState {
                    elements: elements.clone(),
                    ..state
                },
                available_width,
                available_height,
            });
        }

        let next_dimensions = platform.get_dimensions(elements.floating);

        if width != next_dimensions.width || height != next_dimensions.height {
            MiddlewareReturn {
                x: None,
                y: None,
                data: None,
                reset: Some(crate::Reset::Value(ResetValue {
                    placement: None,
                    rects: Some(ResetRects::True),
                })),
            }
        } else {
            MiddlewareReturn {
                x: None,
                y: None,
                data: None,
                reset: None,
            }
        }
    }
}

impl<'a, Element: Clone, Window: Clone>
    MiddlewareWithOptions<Element, Window, SizeOptions<'a, Element, Window>>
    for Size<'a, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, SizeOptions<'a, Element, Window>> {
        &self.options
    }
}
