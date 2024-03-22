use floating_ui_utils::dom::{
    get_computed_style, get_containing_block, get_node_name, get_window, is_containing_block,
    is_html_element, is_table_element, OwnedElementOrWindow,
};
use web_sys::{wasm_bindgen::JsCast, Element, HtmlElement};

use crate::utils::is_top_layer::is_top_layer;

pub fn get_true_offset_parent<Polyfill>(
    element: &Element,
    polyfill: &Option<Polyfill>,
) -> Option<Element>
where
    Polyfill: Fn(&HtmlElement) -> Option<Element>,
{
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

// Gets the closest ancestor positioned element. Handles some edge cases, such as table ancestors and cross browser bugs.
pub fn get_offset_parent<Polyfill>(
    element: &Element,
    polyfill: Option<Polyfill>,
) -> OwnedElementOrWindow
where
    Polyfill: Fn(&HtmlElement) -> Option<Element>,
{
    let window = get_window(Some(element));

    if is_top_layer(element) {
        return OwnedElementOrWindow::Window(window);
    }

    let mut offset_parent = get_true_offset_parent(element, &polyfill);

    while let Some(parent) = offset_parent.as_ref() {
        if is_table_element(parent)
            && get_computed_style(parent)
                .get_property_value("position")
                .expect("Computed style should have position.")
                == "static"
        {
            offset_parent = get_true_offset_parent(parent, &polyfill);
        } else {
            break;
        }
    }

    if let Some(parent) = offset_parent.as_ref() {
        let node_name = get_node_name(parent.into());

        if node_name == "html"
            || node_name == "body"
                && get_computed_style(parent)
                    .get_property_value("position")
                    .expect("Computed style should have position.")
                    == "static"
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
