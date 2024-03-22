use web_sys::{
    css, wasm_bindgen::JsCast, window, CssStyleDeclaration, Document, Element, HtmlElement, Node,
    ShadowRoot, Window,
};

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

#[derive(Clone, Debug)]
pub enum OwnedElementOrWindow {
    Element(Element),
    Window(Window),
}

pub fn get_node_name(node_or_window: NodeOrWindow) -> String {
    match node_or_window {
        NodeOrWindow::Node(node) => node.node_name().to_lowercase(),
        NodeOrWindow::Window(_) => "#document".into(),
    }
}

pub fn get_window(node: Option<&Node>) -> Window {
    match node {
        Some(node) => match node.owner_document() {
            Some(document) => document.default_view(),
            None => window(),
        },
        None => window(),
    }
    .expect("Window should exist.")
}

pub fn get_document_element(node_or_window: Option<NodeOrWindow>) -> Element {
    let document = match node_or_window {
        Some(NodeOrWindow::Node(node)) => node.owner_document(),
        Some(NodeOrWindow::Window(window)) => window.document(),
        None => get_window(None).document(),
    }
    .expect("Node or window should have document.");

    document
        .document_element()
        .expect("Document should have document element.")
}

pub fn is_html_element(node: &Node) -> bool {
    node.is_instance_of::<HtmlElement>()
}

const OVERFLOW_VALUES: [&str; 5] = ["auto", "scroll", "overlay", "hidden", "clip"];
const DISPLAY_VALUES: [&str; 2] = ["inline", "contents"];

pub fn is_overflow_element(element: &Element) -> bool {
    let style = get_computed_style(element);
    let overflow = style.get_property_value("overflow").unwrap_or("".into());
    let overflow_x = style.get_property_value("overflow-x").unwrap_or("".into());
    let overflow_y = style.get_property_value("overflow-y").unwrap_or("".into());
    let display = style.get_property_value("display").unwrap_or("".into());

    let overflow_combined = format!("{}{}{}", overflow, overflow_x, overflow_y);

    OVERFLOW_VALUES
        .into_iter()
        .any(|s| overflow_combined.contains(s))
        && !DISPLAY_VALUES.into_iter().any(|s| display == s)
}

pub fn is_table_element(element: &Element) -> bool {
    let node_name = get_node_name(element.into());
    ["table", "td", "th"].into_iter().any(|s| node_name == s)
}

const WILL_CHANGE_VALUES: [&str; 3] = ["transform", "perspective", "filter"];
const CONTAIN_VALUES: [&str; 4] = ["paint", "layout", "strict", "content"];

pub fn is_containing_block(element: &Element) -> bool {
    let webkit = is_web_kit();
    let css = get_computed_style(element);

    css.get_property_value("transform").unwrap_or("none".into()) != "none"
        || css
            .get_property_value("perspective")
            .unwrap_or("none".into())
            != "none"
        || css
            .get_property_value("container-type")
            .map(|value| value != "normal")
            .unwrap_or(false)
        || (!webkit
            && css
                .get_property_value("backdrop-filter")
                .map(|value| value != "none")
                .unwrap_or(false))
        || (!webkit
            && css
                .get_property_value("filter")
                .map(|value| value != "none")
                .unwrap_or(false))
        || css
            .get_property_value("will-change")
            .map(|value| WILL_CHANGE_VALUES.into_iter().any(|v| v == value))
            .unwrap_or(false)
        || css
            .get_property_value("contain")
            .map(|value| CONTAIN_VALUES.into_iter().any(|v| v == value))
            .unwrap_or(false)
}

pub fn get_containing_block(element: &Element) -> Option<HtmlElement> {
    let mut current_node = get_parent_node(element);

    while !is_last_traversable_node(&current_node) {
        if let Ok(element) = current_node.dyn_into::<HtmlElement>() {
            if is_containing_block(&element) {
                return Some(element);
            }

            current_node = get_parent_node(&element);
        } else {
            break;
        }
    }

    None
}

pub fn is_web_kit() -> bool {
    css::supports_with_value("-webkit-backdrop-filter", "none").unwrap_or(false)
}

pub fn is_last_traversable_node(node: &Node) -> bool {
    let node_name = get_node_name(node.into());
    ["html", "body", "#document"]
        .into_iter()
        .any(|s| node_name == s)
}

pub fn get_computed_style(element: &Element) -> CssStyleDeclaration {
    get_window(Some(element))
        .get_computed_style(element)
        .expect("Valid element.")
        .expect("Element should have computed style.")
}

#[derive(Clone, Debug)]
pub struct NodeScroll {
    pub scroll_left: f64,
    pub scroll_top: f64,
}

impl NodeScroll {
    pub fn new(value: f64) -> Self {
        Self {
            scroll_left: value,
            scroll_top: value,
        }
    }
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
                .expect("Window should have page x offset."),
            scroll_top: window
                .page_y_offset()
                .expect("Window should have page y offset."),
        },
    }
}

pub fn get_parent_node(node: &Node) -> Node {
    if get_node_name(node.into()) == "html" {
        return node.clone();
    }

    let element = node.dyn_ref::<Element>();

    let result: Node;
    if let Some(slot) = element.and_then(|element| element.assigned_slot()) {
        result = slot.into();
    } else if let Some(parent_node) = node.parent_node() {
        result = parent_node;
    } else if let Some(shadow_root) = node.dyn_ref::<ShadowRoot>() {
        result = shadow_root.host().into();
    } else {
        result = get_document_element(Some(node.into())).into();
    }

    match node.dyn_ref::<ShadowRoot>() {
        Some(shadow_root) => shadow_root.host().into(),
        None => result,
    }
}

pub fn get_nearest_overflow_ancestor(node: &Node) -> HtmlElement {
    let parent_node = get_parent_node(node);

    if is_last_traversable_node(&parent_node) {
        node.owner_document()
            .as_ref()
            .or(node.dyn_ref::<Document>())
            .expect("Node should be document or have owner document.")
            .body()
            .expect("Document should have body.")
    } else if is_html_element(&parent_node)
        && is_overflow_element(parent_node.unchecked_ref::<Element>())
    {
        parent_node.unchecked_into()
    } else {
        get_nearest_overflow_ancestor(&parent_node)
    }
}

#[derive(Clone, Debug)]
pub enum OverflowAncestor {
    Element(Element),
    Window(Window),
    // TODO
    // VisualViewport(VisualViewport)
}

pub fn get_overflow_ancestors(
    node: &Node,
    mut list: Vec<OverflowAncestor>,
    traverse_iframe: bool,
) -> Vec<OverflowAncestor> {
    let scrollable_ancestor = get_nearest_overflow_ancestor(node);
    let is_body = node
        .owner_document()
        .and_then(|document| document.body())
        .is_some_and(|body| scrollable_ancestor == body);
    let window = get_window(Some(&scrollable_ancestor));

    if is_body {
        let frame_element = window
            .frame_element()
            .expect("Window should have frame element option.");

        list.push(OverflowAncestor::Window(window));
        // TODO: visual viewport

        if is_overflow_element(&scrollable_ancestor) {
            list.push(OverflowAncestor::Element(scrollable_ancestor.into()));
        }

        if let Some(frame_element) = frame_element {
            if traverse_iframe {
                list.append(&mut get_overflow_ancestors(&frame_element, vec![], true))
            }
        }

        list
    } else {
        let mut other_list = get_overflow_ancestors(&scrollable_ancestor, vec![], traverse_iframe);

        list.push(OverflowAncestor::Element(scrollable_ancestor.into()));
        list.append(&mut other_list);

        list
    }
}
