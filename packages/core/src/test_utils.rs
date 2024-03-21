use floating_ui_utils::{Dimensions, ElementRects, Rect};

use crate::types::{Element, GetClippingRectArgs, GetElementRectsArgs, Platform};

// TODO
pub const REFERENCE: Element = false;
pub const FLOATING: Element = false;
pub const REFERENCE_RECT: Rect = Rect {
    x: 0,
    y: 0,
    width: 100,
    height: 100,
};
pub const FLOATING_RECT: Rect = Rect {
    x: 0,
    y: 0,
    width: 50,
    height: 50,
};

pub struct TestPlatform {}

impl Platform for TestPlatform {
    fn get_element_rects(&self, _args: GetElementRectsArgs) -> ElementRects {
        ElementRects {
            reference: REFERENCE_RECT,
            floating: FLOATING_RECT,
        }
    }

    fn get_clipping_rect(&self, _args: GetClippingRectArgs) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: 1000,
            height: 1000,
        }
    }

    fn get_dimensions(&self, _element: Element) -> Dimensions {
        Dimensions {
            width: 10,
            height: 10,
        }
    }
}

pub const PLATFORM: TestPlatform = TestPlatform {};
