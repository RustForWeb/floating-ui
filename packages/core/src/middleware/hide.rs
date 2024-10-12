use floating_ui_utils::{Rect, SideObject, ALL_SIDES};
use serde::{Deserialize, Serialize};

use crate::{
    detect_overflow::{detect_overflow, DetectOverflowOptions},
    types::{
        Derivable, DerivableFn, ElementContext, Middleware, MiddlewareReturn, MiddlewareState,
        MiddlewareWithOptions,
    },
};

fn get_side_offsets(overflow: SideObject, rect: &Rect) -> SideObject {
    SideObject {
        top: overflow.top - rect.height,
        right: overflow.right - rect.width,
        bottom: overflow.bottom - rect.height,
        left: overflow.left - rect.width,
    }
}

fn is_any_side_fully_clipped(overflow: &SideObject) -> bool {
    ALL_SIDES.into_iter().any(|side| overflow.side(side) >= 0.0)
}

/// Name of the [`Hide`] middleware.
pub const HIDE_NAME: &str = "hide";

/// Fallback strategy used by [`Hide`] middleware.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum HideStrategy {
    #[default]
    ReferenceHidden,
    Escaped,
}

/// Options for [`Hide`] middleware.
#[derive(Clone, Debug, PartialEq)]
pub struct HideOptions<Element: Clone> {
    /// Options for [`detect_overflow`].
    ///
    /// Defaults to [`DetectOverflowOptions::default`].
    pub detect_overflow: Option<DetectOverflowOptions<Element>>,

    /// The strategy used to determine when to hide the floating element.
    ///
    /// Defaults to [`HideStrategy::ReferenceHidden`].
    pub strategy: Option<HideStrategy>,
}

impl<Element: Clone> HideOptions<Element> {
    /// Set `detect_overflow` option.
    pub fn detect_overflow(mut self, value: DetectOverflowOptions<Element>) -> Self {
        self.detect_overflow = Some(value);
        self
    }

    /// Set `strategy` option.
    pub fn strategy(mut self, value: HideStrategy) -> Self {
        self.strategy = Some(value);
        self
    }
}

impl<Element: Clone> Default for HideOptions<Element> {
    fn default() -> Self {
        Self {
            detect_overflow: Default::default(),
            strategy: Default::default(),
        }
    }
}

/// Data stored by [`Hide`] middleware.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HideData {
    pub reference_hidden: Option<bool>,
    pub reference_hidden_offsets: Option<SideObject>,
    pub escaped: Option<bool>,
    pub escaped_offsets: Option<SideObject>,
}

/// Hide middleware.
///
/// Provides data to hide the floating element in applicable situations,
/// such as when it is not in the same clipping context as the reference element.
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/middleware/hide.html) for more documentation.
#[derive(PartialEq)]
pub struct Hide<'a, Element: Clone + 'static, Window: Clone> {
    options: Derivable<'a, Element, Window, HideOptions<Element>>,
}

impl<'a, Element: Clone, Window: Clone> Hide<'a, Element, Window> {
    /// Constructs a new instance of this middleware.
    pub fn new(options: HideOptions<Element>) -> Self {
        Hide {
            options: options.into(),
        }
    }

    /// Constructs a new instance of this middleware with derivable options.
    pub fn new_derivable(options: Derivable<'a, Element, Window, HideOptions<Element>>) -> Self {
        Hide { options }
    }

    /// Constructs a new instance of this middleware with derivable options function.
    pub fn new_derivable_fn(
        options: DerivableFn<'a, Element, Window, HideOptions<Element>>,
    ) -> Self {
        Hide {
            options: options.into(),
        }
    }
}

impl<Element: Clone + 'static, Window: Clone> Clone for Hide<'_, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Middleware<Element, Window>
    for Hide<'static, Element, Window>
{
    fn name(&self) -> &'static str {
        HIDE_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        let options = self.options.evaluate(state.clone());

        let MiddlewareState {
            elements, rects, ..
        } = state;

        let strategy = options.strategy.unwrap_or_default();

        match strategy {
            HideStrategy::ReferenceHidden => {
                let overflow = detect_overflow(
                    MiddlewareState {
                        elements: elements.clone(),
                        ..state
                    },
                    options
                        .detect_overflow
                        .unwrap_or_default()
                        .element_context(ElementContext::Reference),
                );

                let offsets = get_side_offsets(overflow, &rects.reference);

                MiddlewareReturn {
                    x: None,
                    y: None,
                    data: Some(
                        serde_json::to_value(HideData {
                            reference_hidden: Some(is_any_side_fully_clipped(&offsets)),
                            reference_hidden_offsets: Some(offsets),
                            escaped: None,
                            escaped_offsets: None,
                        })
                        .expect("Data should be valid JSON."),
                    ),
                    reset: None,
                }
            }
            HideStrategy::Escaped => {
                let overflow = detect_overflow(
                    MiddlewareState {
                        elements: elements.clone(),
                        ..state
                    },
                    options
                        .detect_overflow
                        .unwrap_or_default()
                        .alt_boundary(true),
                );

                let offsets = get_side_offsets(overflow, &rects.floating);

                MiddlewareReturn {
                    x: None,
                    y: None,
                    data: Some(
                        serde_json::to_value(HideData {
                            reference_hidden: None,
                            reference_hidden_offsets: None,
                            escaped: Some(is_any_side_fully_clipped(&offsets)),
                            escaped_offsets: Some(offsets),
                        })
                        .expect("Data should be valid JSON."),
                    ),
                    reset: None,
                }
            }
        }
    }
}

impl<Element: Clone, Window: Clone> MiddlewareWithOptions<Element, Window, HideOptions<Element>>
    for Hide<'_, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, HideOptions<Element>> {
        &self.options
    }
}
