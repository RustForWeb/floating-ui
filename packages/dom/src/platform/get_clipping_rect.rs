use floating_ui_core::{GetClippingRectArgs, RootBoundary};
use floating_ui_utils::{
    dom::{
        get_computed_style, get_document_element, get_node_name, get_overflow_ancestors,
        get_parent_node, is_containing_block, is_last_traversable_node, is_overflow_element,
        is_top_layer, OverflowAncestor,
    },
    rect_to_client_rect, ClientRectObject, Rect, Strategy,
};
use web_sys::{wasm_bindgen::JsCast, CssStyleDeclaration, Element, Node};

use crate::{
    platform::{get_scale::get_scale, Platform},
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
        ElementOrRootBoundary::RootBoundary(RootBoundary::Viewport) => {
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

fn has_fixed_position_ancestor(element: &Element, stop_node: &Node) -> bool {
    let parent_node = get_parent_node(element);
    if &parent_node == stop_node
        || !parent_node.is_instance_of::<Element>()
        || is_last_traversable_node(&parent_node)
    {
        false
    } else {
        let element = parent_node.unchecked_into::<Element>();
        get_computed_style(&element)
            .get_property_value("position")
            .expect("Computed style should have position.")
            == "fixed"
            || has_fixed_position_ancestor(&element, stop_node)
    }
}

fn get_clipping_element_ancestors(element: &Element) -> Vec<Element> {
    // TODO: cache

    let mut result: Vec<Element> = get_overflow_ancestors(element, vec![], false)
        .into_iter()
        .filter_map(|ancestor| match ancestor {
            OverflowAncestor::Element(element) => {
                match get_node_name((&element).into()) == "body" {
                    true => None,
                    false => Some(element),
                }
            }
            OverflowAncestor::Window(_) => None,
        })
        .collect();
    let mut current_containing_block_computed_style: Option<CssStyleDeclaration> = None;
    let element_is_fixed = get_computed_style(element)
        .get_property_value("position")
        .expect("Computed style should have position.")
        == "fixed";
    let mut current_node: Node = match element_is_fixed {
        true => get_parent_node(element),
        false => element.clone().into(),
    };

    // https://developer.mozilla.org/en-US/docs/Web/CSS/Containing_block#identifying_the_containing_block
    while current_node.is_instance_of::<Element>() && !is_last_traversable_node(&current_node) {
        let current_element = current_node.unchecked_ref::<Element>();
        let computed_style = get_computed_style(current_element);
        let current_node_is_containing = is_containing_block(current_element.into());

        let position = computed_style
            .get_property_value("position")
            .expect("Computed style should have position");

        if !current_node_is_containing && position == "fixed" {
            current_containing_block_computed_style = None;
        }

        let should_drop_current_node = match element_is_fixed {
            true => {
                !current_node_is_containing && current_containing_block_computed_style.is_none()
            }
            false => {
                (!current_node_is_containing
                    && position == "static"
                    && current_containing_block_computed_style
                        .as_ref()
                        .is_some_and(|style| {
                            let positon = style
                                .get_property_value("position")
                                .expect("Computed style should have position");

                            positon == "absolute" || positon == "fixed"
                        }))
                    || (is_overflow_element(current_element)
                        && !current_node_is_containing
                        && has_fixed_position_ancestor(element, current_element))
            }
        };

        if should_drop_current_node {
            result.retain(|ancestor| ancestor != current_element);
        } else {
            current_containing_block_computed_style = Some(computed_style);
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
        floating_ui_core::Boundary::ClippingAncestors => match is_top_layer(element) {
            true => vec![],
            false => get_clipping_element_ancestors(element),
        },
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

    let init =
        get_client_rect_from_clipping_ancestor(element, clipping_ancestors[0].clone(), strategy);
    let clipping_rect = clipping_ancestors
        .into_iter()
        .fold(init, |mut acc, clipping_ancestor| {
            let rect = get_client_rect_from_clipping_ancestor(element, clipping_ancestor, strategy);

            acc.top = acc.top.max(rect.top);
            acc.right = acc.right.min(rect.right);
            acc.bottom = acc.bottom.min(rect.bottom);
            acc.left = acc.left.max(rect.left);

            acc
        });

    Rect {
        x: clipping_rect.left,
        y: clipping_rect.top,
        width: clipping_rect.right - clipping_rect.left,
        height: clipping_rect.bottom - clipping_rect.top,
    }
}
