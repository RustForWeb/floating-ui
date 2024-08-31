use floating_ui_utils::{
    get_padding_object, rect_to_client_rect, Coords, ElementOrVirtual, OwnedElementOrWindow,
    Padding, Rect, SideObject,
};

use crate::types::{
    Boundary, ConvertOffsetParentRelativeRectToViewportRelativeRectArgs, ElementContext, Elements,
    GetClippingRectArgs, MiddlewareState, RootBoundary,
};

/// Options for [`detect_overflow`].
#[derive(Clone, Debug, PartialEq)]
pub struct DetectOverflowOptions<Element> {
    /// The clipping element(s) or area in which overflow will be checked.
    ///
    /// Defaults to [`Boundary::ClippingAncestors`].
    pub boundary: Option<Boundary<Element>>,

    /// The root clipping area in which overflow will be checked.
    ///
    /// Defaults to [`RootBoundary::Viewport`].
    pub root_boundary: Option<RootBoundary>,

    /// The element in which overflow is being checked relative to a boundary.
    ///
    /// Defaults to [`ElementContext::Floating`].
    pub element_context: Option<ElementContext>,

    /// Whether to check for overflow using the alternate element's boundary (only when [`boundary`][`Self::boundary`] is [`Boundary::ClippingAncestors`]).
    ///
    /// Defaults to `false`.
    pub alt_boundary: Option<bool>,

    /// Virtual padding for the resolved overflow detection offsets.
    ///
    /// Defaults to `0` on all sides.
    pub padding: Option<Padding>,
}

impl<Element> DetectOverflowOptions<Element> {
    /// Set `boundary` option.
    pub fn boundary(mut self, value: Boundary<Element>) -> Self {
        self.boundary = Some(value);
        self
    }

    /// Set `root_boundary` option.
    pub fn root_boundary(mut self, value: RootBoundary) -> Self {
        self.root_boundary = Some(value);
        self
    }

    /// Set `element_context` option.
    pub fn element_context(mut self, value: ElementContext) -> Self {
        self.element_context = Some(value);
        self
    }

    /// Set `alt_boundary` option.
    pub fn alt_boundary(mut self, value: bool) -> Self {
        self.alt_boundary = Some(value);
        self
    }

    /// Set `padding` option.
    pub fn padding(mut self, value: Padding) -> Self {
        self.padding = Some(value);
        self
    }
}

impl<Element> Default for DetectOverflowOptions<Element> {
    fn default() -> Self {
        Self {
            boundary: Default::default(),
            root_boundary: Default::default(),
            element_context: Default::default(),
            alt_boundary: Default::default(),
            padding: Default::default(),
        }
    }
}

/// Resolves with an object of overflow side offsets that determine how much the element is overflowing a given clipping boundary on each side.
/// - positive = overflowing the boundary by that number of pixels
/// - negative = how many pixels left before it will overflow
/// - `0` = lies flush with the boundary
///
/// See [the Rust Floating UI book](https://floating-ui.rustforweb.org/detect-overflow.html) for more documentation.
pub fn detect_overflow<Element: Clone + 'static, Window: Clone + 'static>(
    state: MiddlewareState<Element, Window>,
    options: DetectOverflowOptions<Element>,
) -> SideObject {
    let MiddlewareState {
        x,
        y,
        platform,
        rects,
        elements,
        strategy,
        ..
    } = state;

    let boundary = options.boundary.unwrap_or(Boundary::ClippingAncestors);
    let root_boundary = options.root_boundary.unwrap_or(RootBoundary::Viewport);
    let element_context = options.element_context.unwrap_or(ElementContext::Floating);
    let alt_boundary = options.alt_boundary.unwrap_or(false);
    let padding = options.padding.unwrap_or(Padding::All(0.0));

    let padding_object = get_padding_object(padding);
    let alt_context = match element_context {
        ElementContext::Reference => ElementContext::Floating,
        ElementContext::Floating => ElementContext::Reference,
    };
    let element = match alt_boundary {
        true => elements.get_element_context(alt_context),
        false => elements.get_element_context(element_context),
    };

    let document_element = platform.get_document_element(elements.floating);
    let context_element: Option<Element>;

    let element = match element {
        ElementOrVirtual::Element(element) => element,
        ElementOrVirtual::VirtualElement(virtual_element) => {
            context_element = virtual_element.context_element();

            context_element
                .as_ref()
                .or(document_element.as_ref())
                .expect("Element should exist.")
        }
    };

    let clipping_client_rect =
        rect_to_client_rect(platform.get_clipping_rect(GetClippingRectArgs {
            element,
            boundary,
            root_boundary,
            strategy,
        }));

    let rect = match element_context {
        ElementContext::Reference => rects.reference.clone(),
        ElementContext::Floating => Rect {
            x,
            y,
            width: rects.floating.width,
            height: rects.floating.height,
        },
    };

    let offset_parent = platform.get_offset_parent(elements.floating);
    let offset_scale = match offset_parent.as_ref() {
        Some(offset_parent) => match offset_parent {
            OwnedElementOrWindow::Element(element) => {
                platform.get_scale(element).unwrap_or(Coords::new(1.0))
            }
            OwnedElementOrWindow::Window(_) => Coords::new(1.0),
        },
        None => Coords::new(1.0),
    };

    let element_client_rect = rect_to_client_rect(
        platform
            .convert_offset_parent_relative_rect_to_viewport_relative_rect(
                ConvertOffsetParentRelativeRectToViewportRelativeRectArgs {
                    elements: Some(Elements {
                        reference: elements.reference,
                        floating: elements.floating,
                    }),
                    rect: rect.clone(),
                    offset_parent: offset_parent
                        .as_ref()
                        .map(|offset_parent| offset_parent.into()),
                    strategy,
                },
            )
            .unwrap_or(rect),
    );

    SideObject {
        top: (clipping_client_rect.top - element_client_rect.top + padding_object.top)
            / offset_scale.y,
        right: (element_client_rect.right - clipping_client_rect.right + padding_object.right)
            / offset_scale.x,
        bottom: (element_client_rect.bottom - clipping_client_rect.bottom + padding_object.bottom)
            / offset_scale.y,
        left: (clipping_client_rect.left - element_client_rect.left + padding_object.left)
            / offset_scale.x,
    }
}
