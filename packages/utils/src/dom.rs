use web_sys::{window, CssStyleDeclaration, Element, Node, Window};

// pub fn get_node_name()

pub fn get_window(node: &Node) -> Window {
    match node.owner_document() {
        Some(document) => document.default_view(),
        None => window(),
    }
    .expect("Window should exist.")
}

pub enum NodeOrWindow {
    Node(Node),
    Window(Window),
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

pub fn get_computed_style(element: &Element) -> CssStyleDeclaration {
    get_window(element)
        .get_computed_style(element)
        .expect("Valid element.")
        .expect("CSSStyleDeclaration should exist.")
}
