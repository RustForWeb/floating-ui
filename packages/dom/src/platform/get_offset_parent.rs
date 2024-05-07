use floating_ui_utils::dom::{
    get_computed_style, get_containing_block, get_parent_node, get_window, is_containing_block,
    is_element, is_html_element, is_last_traversable_node, is_table_element,
};
use floating_ui_utils::OwnedElementOrWindow;
use web_sys::Window;
use web_sys::{wasm_bindgen::JsCast, Element, HtmlElement};

use crate::utils::is_static_positioned::is_static_positioned;
use crate::utils::is_top_layer::is_top_layer;

pub type Polyfill = Box<dyn Fn(&HtmlElement) -> Option<Element>>;

pub fn get_true_offset_parent(element: &Element, polyfill: &Option<Polyfill>) -> Option<Element> {
    if !is_html_element(element)
        || get_computed_style(element)
            .get_property_value("position")
            .expect("Computed style should have position.")
            == "fixed"
    {
        None
    } else {
        let element = element.unchecked_ref::<HtmlElement>();

        if let Some(polyfill) = polyfill {
            polyfill(element)
        } else {
            element.offset_parent()
        }
    }
}

/// Gets the closest ancestor positioned element. Handles some edge cases, such as table ancestors and cross browser bugs.
pub fn get_offset_parent(
    element: &Element,
    polyfill: Option<Polyfill>,
) -> OwnedElementOrWindow<Element, Window> {
    let window = get_window(Some(element));

    if is_top_layer(element) {
        return OwnedElementOrWindow::Window(window);
    }

    if !is_html_element(element) {
        let mut svg_offset_parent = Some(get_parent_node(element));
        while let Some(parent) = svg_offset_parent.as_ref() {
            if is_last_traversable_node(parent) {
                break;
            }

            if is_element(parent) {
                let element = parent.unchecked_ref::<Element>();
                if !is_static_positioned(element) {
                    return OwnedElementOrWindow::Element(element.clone());
                }
            }
            svg_offset_parent = Some(get_parent_node(parent))
        }
        return OwnedElementOrWindow::Window(window);
    }

    let mut offset_parent = get_true_offset_parent(element, &polyfill);

    while let Some(parent) = offset_parent.as_ref() {
        if is_table_element(parent) && is_static_positioned(parent) {
            offset_parent = get_true_offset_parent(parent, &polyfill);
        } else {
            break;
        }
    }

    if let Some(parent) = offset_parent.as_ref() {
        if is_last_traversable_node(parent)
            && is_static_positioned(parent)
            && !is_containing_block(parent)
        {
            return OwnedElementOrWindow::Window(window);
        }
    }

    offset_parent
        .map(OwnedElementOrWindow::Element)
        .or(get_containing_block(element)
            .map(|element| OwnedElementOrWindow::Element(element.into())))
        .unwrap_or(OwnedElementOrWindow::Window(window))
}
