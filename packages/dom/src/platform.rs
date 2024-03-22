mod get_client_rects;
mod is_rtl;

use floating_ui_core::Platform as CorePlatform;
use floating_ui_utils::{ClientRectObject, Coords, Dimensions, ElementRects, Rect};
use web_sys::Element;

use self::get_client_rects::get_client_rects;
use self::is_rtl::is_rtl;

pub struct Platform {}

impl CorePlatform<Element> for Platform {
    fn get_element_rects(
        &self,
        args: floating_ui_core::GetElementRectsArgs<Element>,
    ) -> ElementRects {
        todo!()
    }

    fn get_clipping_rect(&self, args: floating_ui_core::GetClippingRectArgs<Element>) -> Rect {
        todo!()
    }

    fn get_dimensions(&self, element: &Element) -> Dimensions {
        todo!()
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
        None
    }

    fn get_document_element(&self, _element: &Element) -> Option<&Element> {
        None
    }

    fn get_client_rects(&self, element: &Element) -> Option<Vec<ClientRectObject>> {
        Some(get_client_rects(element))
    }

    fn is_rtl(&self, element: &Element) -> Option<bool> {
        Some(is_rtl(element))
    }

    fn get_scale(&self, _element: &Element) -> Option<Coords> {
        None
    }
}
