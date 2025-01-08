use floating_ui_utils::ClientRectObject;

pub fn rects_are_equal(a: &ClientRectObject, b: &ClientRectObject) -> bool {
    a.x == b.x && a.y == b.y && a.width == b.width && a.height == b.height
}
