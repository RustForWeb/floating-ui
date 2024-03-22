use floating_ui_utils::{
    get_padding_object, rect_to_client_rect, Coords, Padding, Rect, SideObject,
};

use crate::{
    types::{
        Boundary, ConvertOffsetParentRelativeRectToViewportRelativeRectArgs, ElementContext,
        MiddlewareState, RootBoundary,
    },
    Elements, GetClippingRectArgs,
};

#[derive(Debug)]
pub struct DetectOverflowOptions<'a, Element> {
    pub boundary: Option<Boundary<'a, Element>>,
    pub root_boundary: Option<RootBoundary>,
    pub element_context: Option<ElementContext>,
    pub alt_boundary: Option<bool>,
    pub padding: Option<Padding>,
}

impl<'a, Element> Clone for DetectOverflowOptions<'a, Element> {
    fn clone(&self) -> Self {
        Self {
            boundary: self.boundary.clone(),
            root_boundary: self.root_boundary.clone(),
            element_context: self.element_context,
            alt_boundary: self.alt_boundary,
            padding: self.padding.clone(),
        }
    }
}

impl<'a, Element> Default for DetectOverflowOptions<'a, Element> {
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

pub fn detect_overflow<Element>(
    state: MiddlewareState<Element>,
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

    let clipping_client_rect =
        rect_to_client_rect(platform.get_clipping_rect(GetClippingRectArgs {
            element: match platform.is_element(element).unwrap_or(false) {
                true => Some(element),
                false => None, // TODO
            },
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
    let offset_scale = match offset_parent {
        Some(offset_parent) => match platform.is_element(offset_parent).unwrap_or(false) {
            true => platform
                .get_scale(offset_parent)
                .unwrap_or(Coords { x: 1.0, y: 1.0 }),
            false => Coords { x: 1.0, y: 1.0 },
        },
        None => Coords { x: 1.0, y: 1.0 },
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
                    offset_parent,
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
