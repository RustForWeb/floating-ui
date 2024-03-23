#[cfg(feature = "dom")]
pub mod dom;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Alignment {
    Start,
    End,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Strategy {
    Absolute,
    Fixed,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

impl Coords {
    pub fn new(value: f64) -> Self {
        Self { x: value, y: value }
    }

    pub fn update_axis<F>(&mut self, axis: Axis, update: F)
    where
        F: Fn(f64) -> f64,
    {
        match axis {
            Axis::X => {
                self.x = update(self.x);
            }
            Axis::Y => {
                self.y = update(self.y);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Length {
    Width,
    Height,
}

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug)]
pub struct SideObject {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl SideObject {
    pub fn get_side(&self, side: Side) -> f64 {
        match side {
            Side::Top => self.top,
            Side::Right => self.right,
            Side::Bottom => self.bottom,
            Side::Left => self.left,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PartialSideObject {
    pub top: Option<f64>,
    pub right: Option<f64>,
    pub bottom: Option<f64>,
    pub left: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn get_length(&self, length: Length) -> f64 {
        match length {
            Length::Width => self.width,
            Length::Height => self.height,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Padding {
    All(f64),
    PerSide(PartialSideObject),
}

#[derive(Clone, Debug)]
pub struct ClientRectObject {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Clone, Debug)]
pub struct ElementRects {
    pub reference: Rect,
    pub floating: Rect,
}

pub enum ElementOrVirtual<'a, Element> {
    Element(&'a Element),
    VirtualElement(&'a dyn VirtualElement<Element>),
}

impl<'a, Element> ElementOrVirtual<'a, Element> {
    pub fn unwrap(&self) -> Option<&'a Element> {
        match self {
            ElementOrVirtual::Element(element) => Some(element),
            ElementOrVirtual::VirtualElement(virtal_element) => virtal_element.context_element(),
        }
    }
}

impl<'a, Element> From<&'a Element> for ElementOrVirtual<'a, Element> {
    fn from(value: &'a Element) -> Self {
        ElementOrVirtual::Element(value)
    }
}

pub trait VirtualElement<Element> {
    fn get_bounding_client_rect(&self) -> ClientRectObject;

    fn context_element(&self) -> Option<&Element>;
}

#[derive(Clone, Debug)]
pub enum ElementOrWindow<'a, Element, Window> {
    Element(&'a Element),
    Window(&'a Window),
}

impl<'a, Element, Window> From<&'a OwnedElementOrWindow<Element, Window>>
    for ElementOrWindow<'a, Element, Window>
{
    fn from(value: &'a OwnedElementOrWindow<Element, Window>) -> Self {
        match value {
            OwnedElementOrWindow::Element(element) => ElementOrWindow::Element(element),
            OwnedElementOrWindow::Window(window) => ElementOrWindow::Window(window),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OwnedElementOrWindow<Element, Window> {
    Element(Element),
    Window(Window),
}

pub const ALL_PLACEMENTS: [Placement; 12] = [
    Placement::Top,
    Placement::TopStart,
    Placement::TopEnd,
    Placement::Right,
    Placement::RightStart,
    Placement::RightEnd,
    Placement::Bottom,
    Placement::BottomStart,
    Placement::BottomEnd,
    Placement::Left,
    Placement::LeftStart,
    Placement::LeftEnd,
];

pub fn get_side(placement: Placement) -> Side {
    match placement {
        Placement::Top => Side::Top,
        Placement::TopStart => Side::Top,
        Placement::TopEnd => Side::Top,
        Placement::Right => Side::Right,
        Placement::RightStart => Side::Right,
        Placement::RightEnd => Side::Right,
        Placement::Bottom => Side::Bottom,
        Placement::BottomStart => Side::Bottom,
        Placement::BottomEnd => Side::Bottom,
        Placement::Left => Side::Left,
        Placement::LeftStart => Side::Left,
        Placement::LeftEnd => Side::Left,
    }
}

pub fn get_alignment(placement: Placement) -> Option<Alignment> {
    match placement {
        Placement::Top => None,
        Placement::TopStart => Some(Alignment::Start),
        Placement::TopEnd => Some(Alignment::End),
        Placement::Right => None,
        Placement::RightStart => Some(Alignment::Start),
        Placement::RightEnd => Some(Alignment::End),
        Placement::Bottom => None,
        Placement::BottomStart => Some(Alignment::Start),
        Placement::BottomEnd => Some(Alignment::End),
        Placement::Left => None,
        Placement::LeftStart => Some(Alignment::Start),
        Placement::LeftEnd => Some(Alignment::End),
    }
}

pub fn get_placement(side: Side, alignment: Option<Alignment>) -> Placement {
    match (side, alignment) {
        (Side::Top, None) => Placement::Top,
        (Side::Top, Some(Alignment::Start)) => Placement::TopStart,
        (Side::Top, Some(Alignment::End)) => Placement::TopEnd,
        (Side::Right, None) => Placement::Right,
        (Side::Right, Some(Alignment::Start)) => Placement::RightStart,
        (Side::Right, Some(Alignment::End)) => Placement::RightEnd,
        (Side::Bottom, None) => Placement::Bottom,
        (Side::Bottom, Some(Alignment::Start)) => Placement::BottomStart,
        (Side::Bottom, Some(Alignment::End)) => Placement::BottomEnd,
        (Side::Left, None) => Placement::Left,
        (Side::Left, Some(Alignment::Start)) => Placement::LeftStart,
        (Side::Left, Some(Alignment::End)) => Placement::LeftEnd,
    }
}

pub fn get_opposite_axis(axis: Axis) -> Axis {
    match axis {
        Axis::X => Axis::Y,
        Axis::Y => Axis::X,
    }
}

pub fn get_axis_length(axis: Axis) -> Length {
    match axis {
        Axis::X => Length::Width,
        Axis::Y => Length::Height,
    }
}

pub fn get_side_axis(placement: Placement) -> Axis {
    match get_side(placement) {
        Side::Top => Axis::Y,
        Side::Right => Axis::X,
        Side::Bottom => Axis::Y,
        Side::Left => Axis::X,
    }
}

pub fn get_alignment_axis(placement: Placement) -> Axis {
    get_opposite_axis(get_side_axis(placement))
}

pub fn get_alignment_sides(
    placement: Placement,
    rects: &ElementRects,
    rtl: Option<bool>,
) -> (Side, Side) {
    let alignment = get_alignment(placement);
    let alignment_axis = get_alignment_axis(placement);
    let length = get_axis_length(alignment_axis);

    let mut main_alignment_side = match (alignment_axis, alignment) {
        (Axis::X, Some(Alignment::Start)) => match rtl {
            Some(true) => Side::Left,
            _ => Side::Right,
        },
        (Axis::X, _) => match rtl {
            Some(true) => Side::Right,
            _ => Side::Left,
        },
        (Axis::Y, Some(Alignment::Start)) => Side::Bottom,
        (Axis::Y, _) => Side::Top,
    };

    if rects.reference.get_length(length) > rects.floating.get_length(length) {
        main_alignment_side = get_opposite_side(main_alignment_side);
    }

    (main_alignment_side, get_opposite_side(main_alignment_side))
}

pub fn get_expanded_placements(placement: Placement) -> (Placement, Placement, Placement) {
    let opposite_placement = get_opposite_placement(placement);

    (
        get_opposite_alignment_placement(placement),
        opposite_placement,
        get_opposite_alignment_placement(opposite_placement),
    )
}

pub fn get_opposite_alignment_placement(placement: Placement) -> Placement {
    match placement {
        Placement::Top => Placement::Top,
        Placement::TopStart => Placement::TopEnd,
        Placement::TopEnd => Placement::TopStart,
        Placement::Right => Placement::Right,
        Placement::RightStart => Placement::RightEnd,
        Placement::RightEnd => Placement::RightStart,
        Placement::Bottom => Placement::Bottom,
        Placement::BottomStart => Placement::BottomEnd,
        Placement::BottomEnd => Placement::BottomStart,
        Placement::Left => Placement::Left,
        Placement::LeftStart => Placement::LeftEnd,
        Placement::LeftEnd => Placement::LeftStart,
    }
}

pub fn get_side_list(side: Side, is_start: bool, rtl: Option<bool>) -> Vec<Side> {
    match side {
        Side::Top | Side::Bottom => match rtl {
            Some(true) => match is_start {
                true => vec![Side::Right, Side::Left],
                false => vec![Side::Left, Side::Right],
            },
            _ => match is_start {
                true => vec![Side::Left, Side::Right],
                false => vec![Side::Right, Side::Left],
            },
        },
        Side::Right | Side::Left => match is_start {
            true => vec![Side::Top, Side::Bottom],
            false => vec![Side::Bottom, Side::Top],
        },
    }
}

pub fn get_opposite_side(side: Side) -> Side {
    match side {
        Side::Top => Side::Bottom,
        Side::Right => Side::Left,
        Side::Bottom => Side::Top,
        Side::Left => Side::Right,
    }
}

pub fn get_opposite_axis_placements(
    placement: Placement,
    flip_alignment: bool,
    direction: Option<Alignment>,
    rtl: Option<bool>,
) -> Vec<Placement> {
    let alignment = get_alignment(placement);
    let side_list = get_side_list(
        get_side(placement),
        direction.is_some_and(|d| d == Alignment::Start),
        rtl,
    );

    let mut list: Vec<Placement> = side_list
        .into_iter()
        .map(|side| get_placement(side, alignment))
        .collect();

    if flip_alignment {
        let mut opposite_list: Vec<Placement> = list
            .clone()
            .into_iter()
            .map(get_opposite_alignment_placement)
            .collect();

        list.append(&mut opposite_list);
    }

    list
}

pub fn get_opposite_placement(placement: Placement) -> Placement {
    match placement {
        Placement::Top => Placement::Bottom,
        Placement::TopStart => Placement::BottomStart,
        Placement::TopEnd => Placement::BottomEnd,
        Placement::Right => Placement::Left,
        Placement::RightStart => Placement::LeftStart,
        Placement::RightEnd => Placement::LeftEnd,
        Placement::Bottom => Placement::Top,
        Placement::BottomStart => Placement::TopStart,
        Placement::BottomEnd => Placement::TopEnd,
        Placement::Left => Placement::Right,
        Placement::LeftStart => Placement::RightStart,
        Placement::LeftEnd => Placement::RightEnd,
    }
}

pub fn expand_padding_object(padding: PartialSideObject) -> SideObject {
    SideObject {
        top: padding.top.unwrap_or(0.0),
        right: padding.right.unwrap_or(0.0),
        bottom: padding.bottom.unwrap_or(0.0),
        left: padding.left.unwrap_or(0.0),
    }
}

pub fn get_padding_object(padding: Padding) -> SideObject {
    match padding {
        Padding::All(padding) => SideObject {
            top: padding,
            right: padding,
            bottom: padding,
            left: padding,
        },
        Padding::PerSide(padding) => expand_padding_object(padding),
    }
}

pub fn rect_to_client_rect(rect: Rect) -> ClientRectObject {
    ClientRectObject {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        top: rect.y,
        right: rect.x + rect.width,
        bottom: rect.y + rect.height,
        left: rect.x,
    }
}
