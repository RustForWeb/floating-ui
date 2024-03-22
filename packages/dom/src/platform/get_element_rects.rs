use floating_ui_core::{GetElementRectsArgs, Platform as CorePlatform};
use floating_ui_utils::{ElementRects, Rect};
use web_sys::Element;

use crate::{
    platform::Platform,
    utils::get_rect_relative_to_offset_parent::get_rect_relative_to_offset_parent,
};

pub fn get_element_rects(platform: &Platform, args: GetElementRectsArgs<Element>) -> ElementRects {
    // TODO: get_offset_parent should be allowed to return a window
    let offset_parent = platform
        .get_offset_parent(args.floating)
        .expect("Platform implements get_offset_parent.");
    let dimensions = platform.get_dimensions(args.floating);

    ElementRects {
        reference: get_rect_relative_to_offset_parent(
            args.reference.into(),
            // TODO: into can be removed if offset_parent is also ElementOrWindow
            (&offset_parent).into(),
            args.strategy,
        ),
        floating: Rect {
            x: 0.0,
            y: 0.0,
            width: dimensions.width,
            height: dimensions.height,
        },
    }
}
