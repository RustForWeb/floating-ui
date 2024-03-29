use floating_ui_utils::{Rect, SideObject, ALL_SIDES};
use serde::{Deserialize, Serialize};

use crate::{
    detect_overflow::{detect_overflow, DetectOverflowOptions},
    types::{
        Derivable, DerivableFn, Middleware, MiddlewareReturn, MiddlewareState,
        MiddlewareWithOptions,
    },
    ElementContext,
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
#[derive(Copy, Clone, Debug, Default)]
pub enum HideStrategy {
    #[default]
    ReferenceHidden,
    Escaped,
}

/// Options for [`Hide`] middleware.
#[derive(Clone, Debug)]
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

/// Data stored in [`HideData`] when using [`HideStrategy::ReferenceHidden`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HideDataReferenceHidden {
    pub reference_hidden: bool,
    pub reference_hidden_offsets: SideObject,
}

/// Data stored in [`HideData`] when using [`HideStrategy::Escaped`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HideDataEscaped {
    pub escaped: bool,
    pub escaped_offsets: SideObject,
}

/// Data stored by [`Hide`] middleware.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HideData {
    ReferenceHidden(HideDataReferenceHidden),
    Escaped(HideDataEscaped),
}

/// Provides data to hide the floating element in applicable situations,
/// such as when it is not in the same clipping context as the reference element.
///
/// See <https://floating-ui.com/docs/hide> for the original documentation.
pub struct Hide<'a, Element: Clone, Window: Clone> {
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
    pub fn new_derivable(options: DerivableFn<'a, Element, Window, HideOptions<Element>>) -> Self {
        Hide {
            options: options.into(),
        }
    }
}

impl<'a, Element: Clone, Window: Clone> Clone for Hide<'a, Element, Window> {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
        }
    }
}

impl<'a, Element: Clone, Window: Clone> Middleware<Element, Window> for Hide<'a, Element, Window> {
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
                        serde_json::to_value(HideData::ReferenceHidden(HideDataReferenceHidden {
                            reference_hidden: is_any_side_fully_clipped(&offsets),
                            reference_hidden_offsets: offsets,
                        }))
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
                        serde_json::to_value(HideData::Escaped(HideDataEscaped {
                            escaped: is_any_side_fully_clipped(&offsets),
                            escaped_offsets: offsets,
                        }))
                        .expect("Data should be valid JSON."),
                    ),
                    reset: None,
                }
            }
        }
    }
}

impl<'a, Element: Clone, Window: Clone> MiddlewareWithOptions<Element, Window, HideOptions<Element>>
    for Hide<'a, Element, Window>
{
    fn options(&self) -> &Derivable<Element, Window, HideOptions<Element>> {
        &self.options
    }
}
