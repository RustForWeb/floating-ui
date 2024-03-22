pub mod get_client_rects;
pub mod get_dimensions;
pub mod get_element_rects;
pub mod get_scale;
pub mod is_rtl;

use floating_ui_core::Platform as CorePlatform;
use floating_ui_utils::dom::get_document_element;
use floating_ui_utils::{ClientRectObject, Coords, Dimensions, ElementRects, Rect};
use web_sys::Element;

use self::get_client_rects::get_client_rects;
use self::get_dimensions::get_dimensions;
use self::get_element_rects::get_element_rects;
use self::get_scale::get_scale;
use self::is_rtl::is_rtl;

pub struct Platform {}

impl CorePlatform<Element> for Platform {
    fn get_element_rects(
        &self,
        args: floating_ui_core::GetElementRectsArgs<Element>,
    ) -> ElementRects {
        get_element_rects(self, args)
    }

    fn get_clipping_rect(&self, args: floating_ui_core::GetClippingRectArgs<Element>) -> Rect {
        todo!()
    }

    fn get_dimensions(&self, element: &Element) -> Dimensions {
        get_dimensions(element)
    }

    fn convert_offset_parent_relative_rect_to_viewport_relative_rect(
        &self,
        _args: floating_ui_core::ConvertOffsetParentRelativeRectToViewportRelativeRectArgs<
            &Element,
        >,
    ) -> Option<Rect> {
        None
    }

    fn get_offset_parent(&self, _element: &Element) -> Option<&Element> {
        None
    }

    fn is_element(&self, _value: &Element) -> Option<bool> {
        // TODO
        Some(true)
    }

    fn get_document_element(&self, element: &Element) -> Option<Element> {
        Some(get_document_element(element.into()))
    }

    fn get_client_rects(&self, element: &Element) -> Option<Vec<ClientRectObject>> {
        Some(get_client_rects(element))
    }

    fn is_rtl(&self, element: &Element) -> Option<bool> {
        Some(is_rtl(element))
    }

    fn get_scale(&self, element: &Element) -> Option<Coords> {
        Some(get_scale(element.into()))
    }
}
