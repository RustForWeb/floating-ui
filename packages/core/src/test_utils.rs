use floating_ui_utils::{Dimensions, ElementRects, Rect};

use crate::types::{GetClippingRectArgs, GetElementRectsArgs, Platform};

#[derive(Clone, Debug)]
pub struct Element {}

#[derive(Clone, Debug)]
pub struct Window {}

pub const REFERENCE: Element = Element {};
pub const FLOATING: Element = Element {};
pub const REFERENCE_RECT: Rect = Rect {
    x: 0.0,
    y: 0.0,
    width: 100.0,
    height: 100.0,
};
pub const FLOATING_RECT: Rect = Rect {
    x: 0.0,
    y: 0.0,
    width: 50.0,
    height: 50.0,
};

#[derive(Debug)]
pub struct TestPlatform {}

impl Platform<Element, Window> for TestPlatform {
    fn get_element_rects(&self, _args: GetElementRectsArgs<Element>) -> ElementRects {
        ElementRects {
            reference: REFERENCE_RECT,
            floating: FLOATING_RECT,
        }
    }

    fn get_clipping_rect(&self, _args: GetClippingRectArgs<Element>) -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            width: 1000.0,
            height: 1000.0,
        }
    }

    fn get_dimensions(&self, _element: &Element) -> Dimensions {
        Dimensions {
            width: 10.0,
            height: 10.0,
        }
    }
}

pub const PLATFORM: TestPlatform = TestPlatform {};
