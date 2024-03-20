pub enum Alignment {
    Start,
    End,
}

pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

pub enum AlignedPlacement {
    TopStart,
    TopEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
    LeftStart,
    LeftEnd,
}

pub enum Placement {
    Top,
    TopStart,
    TopEnd,
    Right,
    RightStart,
    RightEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
}

pub enum Strategy {
    Absolute,
    Fixed,
}

pub enum Axis {
    X,
    Y,
}

pub struct Coords {
    x: isize,
    y: isize,
}

pub enum Length {
    Width,
    Height,
}

pub struct Dimensions {
    width: isize,
    height: isize,
}

pub struct SideObject {
    top: isize,
    right: isize,
    bottom: isize,
    left: isize,
}

pub struct Rect {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
}

pub enum Padding {
    All(isize),
    PerSide(SideObject),
}

pub struct ClientRectObject {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    top: isize,
    right: isize,
    bottom: isize,
    left: isize,
}

pub struct ElementRects {
    reference: Rect,
    floating: Rect,
}
