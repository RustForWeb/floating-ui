use web_sys::{window, CssStyleDeclaration, Element, Node, Window};

#[derive(Clone, Debug)]
pub enum NodeOrWindow<'a> {
    Node(&'a Node),
    Window(&'a Window),
}

impl<'a> From<&'a Node> for NodeOrWindow<'a> {
    fn from(value: &'a Node) -> Self {
        NodeOrWindow::Node(value)
    }
}

impl<'a> From<&'a Element> for NodeOrWindow<'a> {
    fn from(value: &'a Element) -> Self {
        NodeOrWindow::Node(value)
    }
}

impl<'a> From<&'a Window> for NodeOrWindow<'a> {
    fn from(value: &'a Window) -> Self {
        NodeOrWindow::Window(value)
    }
}

impl<'a> From<&'a ElementOrWindow<'a>> for NodeOrWindow<'a> {
    fn from(value: &'a ElementOrWindow) -> Self {
        match value {
            ElementOrWindow::Element(element) => NodeOrWindow::Node(element),
            ElementOrWindow::Window(window) => NodeOrWindow::Window(window),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ElementOrWindow<'a> {
    Element(&'a Element),
    Window(&'a Window),
}

impl<'a> From<&'a Element> for ElementOrWindow<'a> {
    fn from(value: &'a Element) -> Self {
        ElementOrWindow::Element(value)
    }
}

impl<'a> From<&'a Window> for ElementOrWindow<'a> {
    fn from(value: &'a Window) -> Self {
        ElementOrWindow::Window(value)
    }
}

pub fn get_node_name(node_or_window: NodeOrWindow) -> String {
    match node_or_window {
        NodeOrWindow::Node(node) => node.node_name(),
        NodeOrWindow::Window(_) => "#document".into(),
    }
}

pub fn get_window(node: &Node) -> Window {
    match node.owner_document() {
        Some(document) => document.default_view(),
        None => window(),
    }
    .expect("Window should exist.")
}

pub fn get_document_element(node_or_window: NodeOrWindow) -> Element {
    let document = match node_or_window {
        NodeOrWindow::Node(node) => node.owner_document(),
        NodeOrWindow::Window(window) => window.document(),
    }
    .expect("Document should exist.");

    document
        .document_element()
        .expect("Document element should exist.")
}

// pub fn is_shadow_root

pub fn is_overflow_element(element: &Element) -> bool {
    let style = get_computed_style(element);
    let overflow = style.get_property_value("overflow").unwrap_or("".into());
    let overflow_x = style.get_property_value("overflow-x").unwrap_or("".into());
    let overflow_y = style.get_property_value("overflow-y").unwrap_or("".into());
    let display = style.get_property_value("display").unwrap_or("".into());

    let overflow_combined = format!("{}{}{}", overflow, overflow_x, overflow_y);

    ["auto", "scroll", "overlay", "hidden", "clip"]
        .into_iter()
        .any(|s| overflow_combined.contains(s))
        && !["inline", "contents"].into_iter().any(|s| display == s)
}

pub fn is_table_element(element: &Element) -> bool {
    let node_name = get_node_name(element.into());
    ["table", "td", "th"].into_iter().any(|s| node_name == s)
}

pub fn get_computed_style(element: &Element) -> CssStyleDeclaration {
    get_window(element)
        .get_computed_style(element)
        .expect("Valid element.")
        .expect("CSSStyleDeclaration should exist.")
}

#[derive(Clone, Debug)]
pub struct NodeScroll {
    pub scroll_left: f64,
    pub scroll_top: f64,
}

pub fn get_node_scroll(element_or_window: ElementOrWindow) -> NodeScroll {
    match element_or_window {
        ElementOrWindow::Element(element) => NodeScroll {
            scroll_left: element.scroll_left() as f64,
            scroll_top: element.scroll_top() as f64,
        },
        ElementOrWindow::Window(window) => NodeScroll {
            scroll_left: window
                .page_x_offset()
                .expect("Window should have pageXOffset."),
            scroll_top: window
                .page_y_offset()
                .expect("Window should have pageYOffset."),
        },
    }
}
