use floating_ui_utils::{
    dom::{get_computed_style, get_frame_element, get_window, DomElementOrWindow},
    rect_to_client_rect, ClientRectObject, Coords, Rect,
};

use crate::{
    platform::get_scale::get_scale,
    types::ElementOrVirtual,
    utils::get_visual_offsets::{get_visual_offsets, should_add_visual_offsets},
};

pub fn get_bounding_client_rect(
    element_or_virtual: ElementOrVirtual,
    include_scale: bool,
    is_fixed_strategy: bool,
    offset_parent: Option<DomElementOrWindow>,
) -> ClientRectObject {
    let client_rect = match &element_or_virtual {
        ElementOrVirtual::Element(element) => element.get_bounding_client_rect().into(),
        ElementOrVirtual::VirtualElement(virtual_element) => {
            virtual_element.get_bounding_client_rect()
        }
    };
    let dom_element = element_or_virtual.clone().resolve();

    let scale = match include_scale {
        true => match &offset_parent {
            Some(offset_parent) => match offset_parent {
                DomElementOrWindow::Element(element) => get_scale((*element).into()),
                DomElementOrWindow::Window(_) => Coords::new(1.0),
            },
            None => get_scale(element_or_virtual),
        },
        false => Coords::new(1.0),
    };

    let visual_offsets = match should_add_visual_offsets(
        dom_element.as_ref(),
        is_fixed_strategy,
        offset_parent.clone(),
    ) {
        true => get_visual_offsets(dom_element.as_ref()),
        false => Coords::new(0.0),
    };

    let mut x = (client_rect.left + visual_offsets.x) / scale.x;
    let mut y = (client_rect.top + visual_offsets.y) / scale.y;
    let mut width = client_rect.width / scale.x;
    let mut height = client_rect.height / scale.y;

    if let Some(dom_element) = dom_element {
        let window = get_window(Some(&dom_element));
        let offset_window = match offset_parent {
            Some(DomElementOrWindow::Element(element)) => Some(get_window(Some(element))),
            Some(DomElementOrWindow::Window(window)) => Some(window.clone()),
            None => None,
        };

        if offset_parent.is_some() {
            let mut current_window = window;
            loop {
                let current_iframe = get_frame_element(&current_window);

                if let Some(current_iframe) = current_iframe.as_ref() {
                    if offset_window
                        .as_ref()
                        .is_some_and(|offset_window| offset_window != &current_window)
                    {
                        let iframe_scale = get_scale(current_iframe.into());
                        let iframe_rect = current_iframe.get_bounding_client_rect();
                        let css = get_computed_style(current_iframe);
                        let padding_left = css
                            .get_property_value("padding-left")
                            .expect("Computed style should have padding left.")
                            .parse::<f64>()
                            .expect("Padding left should be a number.");
                        let padding_top = css
                            .get_property_value("padding-right")
                            .expect("Computed style should have padding right.")
                            .parse::<f64>()
                            .expect("Padding right should be a number.");

                        let left = iframe_rect.left()
                            + (current_iframe.client_left() as f64 + padding_left) * iframe_scale.x;
                        let top = iframe_rect.top()
                            + (current_iframe.client_top() as f64 + padding_top) * iframe_scale.y;

                        x *= iframe_scale.x;
                        y *= iframe_scale.y;
                        width *= iframe_scale.x;
                        height *= iframe_scale.y;

                        x += left;
                        y += top;

                        current_window = get_window(Some(current_iframe));
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    rect_to_client_rect(Rect {
        x,
        y,
        width,
        height,
    })
}
