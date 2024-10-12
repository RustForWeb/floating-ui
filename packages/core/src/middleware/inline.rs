use std::rc::Rc;

use floating_ui_utils::{
    get_padding_object, get_side_axis, rect_to_client_rect, Axis, ClientRectObject, Coords,
    DefaultVirtualElement, ElementOrVirtual, Padding, Rect, Side,
};

use crate::types::{
    Derivable, DerivableFn, GetElementRectsArgs, Middleware, MiddlewareReturn, MiddlewareState,
    MiddlewareWithOptions, Reset, ResetRects, ResetValue,
};

fn get_bounding_rect(rects: Vec<ClientRectObject>) -> Rect {
    let min_x = rects
        .iter()
        .map(|rect| rect.left)
        .reduce(f64::min)
        .unwrap_or(f64::INFINITY);
    let min_y = rects
        .iter()
        .map(|rect| rect.top)
        .reduce(f64::min)
        .unwrap_or(f64::INFINITY);
    let max_x = rects
        .iter()
        .map(|rect| rect.right)
        .reduce(f64::max)
        .unwrap_or(f64::NEG_INFINITY);
    let max_y = rects
        .iter()
        .map(|rect| rect.bottom)
        .reduce(f64::max)
        .unwrap_or(f64::NEG_INFINITY);
    Rect {
        x: min_x,
        y: min_y,
        width: max_x - min_x,
        height: max_y - min_y,
    }
}

fn get_rects_by_line(rects: Vec<ClientRectObject>) -> Vec<ClientRectObject> {
    let mut sorted_rects = rects.clone();
    sorted_rects.sort_by(|a, b| a.y.total_cmp(&b.y));

    let mut groups: Vec<Vec<ClientRectObject>> = vec![];
    let mut prev_rect: Option<ClientRectObject> = None;
    for rect in sorted_rects {
        if prev_rect.is_none()
            || prev_rect.is_some_and(|prev_rect| rect.y - prev_rect.y > prev_rect.height / 2.0)
        {
            groups.push(vec![rect.clone()]);
        } else {
            groups
                .last_mut()
                .expect("Last group should exist.")
                .push(rect.clone());
        }
        prev_rect = Some(rect);
    }

    groups
        .into_iter()
        .map(|rects| rect_to_client_rect(get_bounding_rect(rects)))
        .collect()
}

/// Name of the [`Inline`] middleware.
pub const INLINE_NAME: &str = "inline";

/// Options for [`Inline`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InlineOptions {
    /// Viewport-relative `x` coordinate to choose a `ClientRect`.
    ///
    /// Defaults to [`None`].
    pub x: Option<f64>,

    /// Viewport-relative `y` coordinate to choose a `ClientRect`.
    ///
    /// Defaults to [`None`].
    pub y: Option<f64>,

    /// Represents the padding around a disjoined rect when choosing it.
    ///
    /// Defaults to `2` on all sides.
    pub padding: Option<Padding>,
}

impl InlineOptions {
    /// Set `x` option.
    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }

    /// Set `y` option.
    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }

    /// Set `x` and `y` options using [`Coords`].
    pub fn coords(mut self, value: Coords) -> Self {
        self.x = Some(value.x);
        self.y = Some(value.y);
        self
    }

    /// Set `padding` option.
    pub fn padding(mut self, value: Padding) -> Self {
        self.padding = Some(value);
        self
    }
}

/// Inline middleware.
///
/// Provides improved positioning for inline reference elements that can span over multiple lines, such as hyperlinks or range selections.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/inline.html) for more documentation.
#[derive(PartialEq)]
pub struct Inline<'a, Element: Clone + 'static, Window: Clone> {
    options: Derivable<'a, Element, Window, InlineOptions>,
}

impl<'a, Element: Clone + 'static, Window: Clone> Inline<'a, Element, Window> {
    /// Constructs a new instance of this middleware.
    pub fn new(options: InlineOptions) -> Self {
        Inline {
            options: options.into(),
        }
    }

    /// Constructs a new instance of this middleware with derivable options.
    pub fn new_derivable(options: Derivable<'a, Element, Window, InlineOptions>) -> Self {
        Inline { options }
    }

    /// Constructs a new instance of this middleware with derivable options function.
    pub fn new_derivable_fn(options: DerivableFn<'a, Element, Window, InlineOptions>) -> Self {
        Inline {
            options: options.into(),
        }
    }
}

impl<Element: Clone, Window: Clone> Clone for Inline<'_, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}

impl<Element: Clone + PartialEq + 'static, Window: Clone + PartialEq + 'static>
    Middleware<Element, Window> for Inline<'static, Element, Window>
{
    fn name(&self) -> &'static str {
        INLINE_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let options = self.options.evaluate(state.clone());

        let MiddlewareState {
            placement,
            strategy,
            elements,
            rects,
            platform,
            ..
        } = state;

        // A MouseEvent's client{X,Y} coords can be up to 2 pixels off a ClientRect's bounds,
        // despite the event listener being triggered. A padding of 2 seems to handle this issue.
        let padding = options.padding.unwrap_or(Padding::All(2.0));

        let native_client_rects = platform
            .get_client_rects(elements.reference)
            .unwrap_or(vec![]);

        let client_rects = get_rects_by_line(native_client_rects.clone());
        let fallback = rect_to_client_rect(get_bounding_rect(native_client_rects));
        let padding_object = get_padding_object(padding);

        let get_bounding_client_rect = move || {
            // There are two rects and they are disjoined.
            if client_rects.len() == 2 && client_rects[0].left > client_rects[1].right {
                if let Some(x) = options.x {
                    if let Some(y) = options.y {
                        return client_rects
                            .clone()
                            .into_iter()
                            .find(|rect| {
                                x > rect.left - padding_object.left
                                    && x < rect.right + padding_object.right
                                    && y > rect.top - padding_object.top
                                    && rect.y < rect.bottom + padding_object.bottom
                            })
                            .unwrap_or(fallback.clone());
                    }
                }
            }

            // There are 2 or more connected rects.
            if client_rects.len() >= 2 {
                if get_side_axis(placement) == Axis::Y {
                    let first_rect = client_rects.first().expect("Enough elements exist.");
                    let last_rect = client_rects.last().expect("Enough elements exist.");
                    let is_top = placement.side() == Side::Top;

                    let top = first_rect.top;
                    let bottom = last_rect.bottom;
                    let left = match is_top {
                        true => first_rect.left,
                        false => last_rect.left,
                    };
                    let right = match is_top {
                        true => first_rect.right,
                        false => last_rect.right,
                    };
                    let width = right - left;
                    let height = bottom - top;

                    return ClientRectObject {
                        x: left,
                        y: top,
                        width,
                        height,
                        top,
                        right,
                        bottom,
                        left,
                    };
                }

                let is_left_side = placement.side() == Side::Left;
                let max_right = client_rects
                    .iter()
                    .map(|rect| rect.right)
                    .reduce(f64::max)
                    .expect("Enough elements exist.");
                let min_left = client_rects
                    .iter()
                    .map(|rect| rect.left)
                    .reduce(f64::min)
                    .expect("Enough elements exist.");
                let measure_rects: Vec<&ClientRectObject> = client_rects
                    .iter()
                    .filter(|rect| match is_left_side {
                        true => rect.left == min_left,
                        false => rect.right == max_right,
                    })
                    .collect();

                let top = measure_rects.first().expect("Enough elements exist.").top;
                let bottom = measure_rects.last().expect("Enough elements exist.").bottom;
                let left = min_left;
                let right = max_right;
                let width = right - left;
                let height = bottom - top;

                return ClientRectObject {
                    x: left,
                    y: top,
                    width,
                    height,
                    top,
                    right,
                    bottom,
                    left,
                };
            }

            fallback.clone()
        };

        let reset_rects = platform.get_element_rects(GetElementRectsArgs {
            reference: ElementOrVirtual::VirtualElement(Box::new(DefaultVirtualElement::new(
                Rc::new(get_bounding_client_rect),
            ))),
            floating: elements.floating,
            strategy,
        });

        if rects.reference.x != reset_rects.reference.x
            || rects.reference.y != reset_rects.reference.y
            || rects.reference.width != reset_rects.reference.width
            || rects.reference.height != reset_rects.reference.height
        {
            MiddlewareReturn {
                x: None,
                y: None,
                data: None,
                reset: Some(Reset::Value(ResetValue {
                    placement: None,
                    rects: Some(ResetRects::Value(reset_rects)),
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

impl<Element: Clone, Window: Clone> MiddlewareWithOptions<Element, Window, InlineOptions>
    for Inline<'_, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, InlineOptions> {
        &self.options
    }
}
