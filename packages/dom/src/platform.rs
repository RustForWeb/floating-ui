pub mod convert_offset_parent_relative_rect_to_viewport_relative_rect;
pub mod get_client_length;
pub mod get_client_rects;
pub mod get_clipping_rect;
pub mod get_dimensions;
pub mod get_element_rects;
pub mod get_offset_parent;
pub mod get_scale;
pub mod is_rtl;

use floating_ui_core::{
    ConvertOffsetParentRelativeRectToViewportRelativeRectArgs, GetClippingRectArgs,
    GetElementRectsArgs, Platform as CorePlatform,
};
use floating_ui_utils::dom::get_document_element;
use floating_ui_utils::{
    ClientRectObject, Coords, Dimensions, ElementRects, Length, OwnedElementOrWindow, Rect,
};
use web_sys::{Element, Window};

use crate::types::ElementOrVirtual;

use self::convert_offset_parent_relative_rect_to_viewport_relative_rect::convert_offset_parent_relative_rect_to_viewport_relative_rect;
use self::get_client_length::get_client_length;
use self::get_client_rects::get_client_rects;
use self::get_clipping_rect::get_clipping_rect;
use self::get_dimensions::get_dimensions;
use self::get_element_rects::get_element_rects;
use self::get_offset_parent::get_offset_parent;
use self::get_scale::get_scale;
use self::is_rtl::is_rtl;

#[derive(Debug)]
pub struct Platform {}

impl CorePlatform<Element, Window> for Platform {
    fn get_element_rects(&self, args: GetElementRectsArgs<Element>) -> ElementRects {
        get_element_rects(self, args)
    }

    fn get_clipping_rect(&self, args: GetClippingRectArgs<Element>) -> Rect {
        get_clipping_rect(self, args)
    }

    fn get_dimensions(&self, element: &Element) -> Dimensions {
        get_dimensions(element)
    }

    fn convert_offset_parent_relative_rect_to_viewport_relative_rect(
        &self,
        args: ConvertOffsetParentRelativeRectToViewportRelativeRectArgs<Element, Window>,
    ) -> Option<Rect> {
        Some(convert_offset_parent_relative_rect_to_viewport_relative_rect(args))
    }

    fn get_offset_parent(
        &self,
        element: &Element,
    ) -> Option<OwnedElementOrWindow<Element, Window>> {
        Some(get_offset_parent(element, None))
    }

    fn get_document_element(&self, element: &Element) -> Option<Element> {
        Some(get_document_element(Some(element.into())))
    }

    fn get_client_rects(&self, element: ElementOrVirtual) -> Option<Vec<ClientRectObject>> {
        Some(get_client_rects(element))
    }

    fn is_rtl(&self, element: &Element) -> Option<bool> {
        Some(is_rtl(element))
    }

    fn get_scale(&self, element: &Element) -> Option<Coords> {
        Some(get_scale(element.into()))
    }

    fn get_client_length(&self, element: &Element, length: Length) -> Option<f64> {
        Some(get_client_length(element, length))
    }
}
