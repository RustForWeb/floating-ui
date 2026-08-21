use floating_ui_core::{GetClippingRectArgs, RootBoundary};
use floating_ui_utils::{
    ClientRectObject, Rect, Strategy,
    dom::{
        OverflowAncestor, get_computed_style, get_document_element, get_node_name,
        get_overflow_ancestors, get_parent_node, is_containing_block, is_last_traversable_node,
        is_top_layer,
    },
    rect_to_client_rect,
};
use web_sys::{CssStyleDeclaration, Element, Node, wasm_bindgen::JsCast};

use crate::{
    platform::{Platform, get_scale::get_scale},
    types::Boundary,
    utils::{
        get_bounding_client_rect::get_bounding_client_rect, get_document_rect::get_document_rect,
        get_viewport_rect::get_viewport_rect, get_visual_offsets::get_visual_offsets,
    },
};

#[derive(Clone, Debug)]
enum ElementOrRootBoundary {
    Element(Element),
    RootBoundary(RootBoundary),
}

fn get_inner_bounding_client_rect(element: &Element, strategy: Strategy) -> Rect {
    let client_rect =
        get_bounding_client_rect(element.into(), true, strategy == Strategy::Fixed, None);
    let top = client_rect.top + element.client_top() as f64;
    let left = client_rect.left + element.client_left() as f64;
    let scale = get_scale(element.into());

    Rect {
        x: left * scale.x,
        y: top * scale.y,
        width: element.client_width() as f64 * scale.x,
        height: element.client_height() as f64 * scale.y,
    }
}

fn get_client_rect_from_clipping_ancestor(
    element: &Element,
    clipping_ancestor: ElementOrRootBoundary,
    strategy: Strategy,
) -> ClientRectObject {
    let rect = match clipping_ancestor {
        ElementOrRootBoundary::Element(element) => {
            get_inner_bounding_client_rect(&element, strategy)
        }
        ElementOrRootBoundary::RootBoundary(RootBoundary::Viewport)
        | ElementOrRootBoundary::RootBoundary(RootBoundary::LayoutViewport) => {
            get_viewport_rect(&get_document_element(Some(element.into())), strategy)
        }
        ElementOrRootBoundary::RootBoundary(RootBoundary::Document) => {
            get_document_rect(&get_document_element(Some(element.into())))
        }
        ElementOrRootBoundary::RootBoundary(RootBoundary::Rect(rect)) => {
            let visual_offsets = get_visual_offsets(Some(element));
            Rect {
                x: rect.x - visual_offsets.x,
                y: rect.y - visual_offsets.y,
                width: rect.width,
                height: rect.height,
            }
        }
    };

    rect_to_client_rect(rect)
}

fn get_clipping_element_ancestors(element: &Element) -> Vec<Element> {
    // TODO: cache

    let mut result: Vec<Element> = get_overflow_ancestors(element, vec![], false)
        .into_iter()
        .filter_map(|ancestor| match ancestor {
            OverflowAncestor::Element(element) => {
                (get_node_name((&element).into()) != "body").then_some(element)
            }
            OverflowAncestor::Window(_) => None,
            OverflowAncestor::VisualViewport(_) => None,
        })
        .collect();
    let mut last_kept_computed_style: Option<CssStyleDeclaration> = None;
    let element_is_fixed = get_computed_style(element)
        .get_property_value("position")
        .expect("Computed style should have position.")
        == "fixed";
    let mut current_node: Node = if element_is_fixed {
        get_parent_node(element)
    } else {
        element.clone().into()
    };

    // https://developer.mozilla.org/en-US/docs/Web/CSS/Containing_block#identifying_the_containing_block
    while current_node.is_instance_of::<Element>() && !is_last_traversable_node(&current_node) {
        let current_element = current_node.unchecked_ref::<Element>();
        let computed_style = get_computed_style(current_element);
        let current_node_is_containing = is_containing_block(current_element.into());

        // Position of the containing block chain below the current node. A fixed
        // element whose containing block hasn't been found yet is a fixed chain.
        let last_position =
            if let Some(last_kept_computed_style) = last_kept_computed_style.as_ref() {
                last_kept_computed_style
                    .get_property_value("position")
                    .expect("Computed style should have position")
            } else {
                if element_is_fixed { "fixed" } else { "" }.to_owned()
            };

        // A non-containing ancestor does not clip the element when the chain
        // below it escapes it: a fixed chain escapes all ancestors up to the
        // next containing block, an absolute chain escapes static ancestors.
        let should_drop_current_node = !current_node_is_containing
            && (last_position == "fixed"
                || (last_position == "absolute"
                    && computed_style
                        .get_property_value("position")
                        .expect("Computed style should have position")
                        == "static"));

        if should_drop_current_node {
            // Drop non-containing blocks.
            result.retain(|ancestor| ancestor != current_element);
        } else {
            // The kept node carries the chain position for the next iteration.
            last_kept_computed_style = Some(computed_style);
        }

        current_node = get_parent_node(&current_node);
    }

    // TODO: cache

    result
}

pub fn get_clipping_rect(
    _platform: &Platform,
    GetClippingRectArgs {
        element,
        boundary,
        root_boundary,
        strategy,
    }: GetClippingRectArgs<Element>,
) -> Rect {
    // TODO: cache

    let clipping_element_ancestors = match boundary {
        Boundary::ClippingAncestors => {
            if is_top_layer(element) {
                vec![]
            } else {
                get_clipping_element_ancestors(element)
            }
        }
        _ => vec![],
    };

    let element_clipping_ancestors: Vec<Element> = clipping_element_ancestors
        .into_iter()
        .chain(match boundary {
            Boundary::Element(element) => vec![element],
            Boundary::Elements(elements) => elements,
            _ => vec![],
        })
        .collect();

    let clipping_ancestors: Vec<ElementOrRootBoundary> = element_clipping_ancestors
        .into_iter()
        .map(ElementOrRootBoundary::Element)
        .chain(vec![ElementOrRootBoundary::RootBoundary(root_boundary)])
        .collect();

    let first_rect =
        get_client_rect_from_clipping_ancestor(element, clipping_ancestors[0].clone(), strategy);
    let mut top = first_rect.top;
    let mut right = first_rect.right;
    let mut bottom = first_rect.bottom;
    let mut left = first_rect.left;

    for clipping_ancestor in clipping_ancestors.into_iter().skip(1) {
        let rect = get_client_rect_from_clipping_ancestor(element, clipping_ancestor, strategy);
        top = top.max(rect.top);
        right = right.min(rect.right);
        bottom = bottom.min(rect.bottom);
        left = left.max(rect.left);
    }

    Rect {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    }
}
