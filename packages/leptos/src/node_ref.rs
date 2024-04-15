use std::ops::Deref;

use leptos::{html::ElementDescriptor, HtmlElement, NodeRef};
use web_sys::Element;

pub trait NodeRefAsElement<T: ElementDescriptor + Clone + 'static> {
    fn get(&self) -> Option<HtmlElement<T>>;

    fn get_as_element(&self) -> Option<Element>;

    fn get_untracked_as_element(&self) -> Option<Element>;
}

impl NodeRefAsElement<leptos::html::AnyElement> for NodeRef<leptos::html::AnyElement> {
    fn get(&self) -> Option<HtmlElement<leptos::html::AnyElement>> {
        self.get()
    }

    fn get_as_element(&self) -> Option<Element> {
        self.get().map(|html_element| {
            let element: &web_sys::Element = html_element.deref();
            element.clone()
        })
    }

    fn get_untracked_as_element(&self) -> Option<Element> {
        self.get_untracked().map(|html_element| {
            let element: &web_sys::Element = html_element.deref();
            element.clone()
        })
    }
}

macro_rules! generate_html_tags {
    ($(
      $tag:ty
    ),* $(,)?) => {
        paste::paste! {
            $(
                impl NodeRefAsElement<leptos::html::[<$tag>]> for NodeRef<leptos::html::[<$tag>]> {
                    fn get(&self) -> Option<HtmlElement<leptos::html::[<$tag>]>> {
                        self.get()
                    }

                    fn get_as_element(&self) -> Option<Element> {
                        self.get().map(|html_element| {
                            let element: &web_sys::Element = html_element.deref();
                            element.clone()
                        })
                    }

                    fn get_untracked_as_element(&self) -> Option<Element> {
                        self.get_untracked().map(|html_element| {
                            let element: &web_sys::Element = html_element.deref();
                            element.clone()
                        })
                    }
                }
            )*
        }
    }
}

macro_rules! generate_math_tags {
    ($(
      $tag:ty
    ),* $(,)?) => {
        paste::paste! {
            $(
                impl NodeRefAsElement<leptos::math::[<$tag>]> for NodeRef<leptos::math::[<$tag>]> {
                    fn get(&self) -> Option<HtmlElement<leptos::math::[<$tag>]>> {
                        self.get()
                    }

                    fn get_as_element(&self) -> Option<Element> {
                        self.get().map(|html_element| {
                            let element: &web_sys::Element = html_element.deref();
                            element.clone()
                        })
                    }

                    fn get_untracked_as_element(&self) -> Option<Element> {
                        self.get_untracked().map(|html_element| {
                            let element: &web_sys::Element = html_element.deref();
                            element.clone()
                        })
                    }
                }
            )*
        }
    }
}

macro_rules! generate_svg_tags {
    ($(
      $tag:ty
    ),* $(,)?) => {
        paste::paste! {
            $(
                impl NodeRefAsElement<leptos::svg::[<$tag>]> for NodeRef<leptos::svg::[<$tag>]> {
                    fn get(&self) -> Option<HtmlElement<leptos::svg::[<$tag>]>> {
                        self.get()
                    }

                    fn get_as_element(&self) -> Option<Element> {
                        self.get().map(|html_element| {
                            let element: &web_sys::Element = html_element.deref();
                            element.clone()
                        })
                    }

                    fn get_untracked_as_element(&self) -> Option<Element> {
                        self.get_untracked().map(|html_element| {
                            let element: &web_sys::Element = html_element.deref();
                            element.clone()
                        })
                    }
                }
            )*
        }
    }
}

generate_html_tags![
    Html, Base, Head, Link, Meta, Style, Title, Body, Address, Article, Aside, Footer, Header,
    Hgroup, H1, H2, H3, H4, H5, H6, Main, Nav, Section, Blockquote, Dd, Div, Dl, Dt, Figcaption,
    Figure, Hr, Li, Ol, P, Pre, Ul, A, Abbr, B, Bdi, Bdo, Br, Cite, Code, Data, Dfn, Em, I, Kbd,
    Mark, Q, Rp, Rt, Ruby, S, Samp, Small, Span, Strong, Sub, Sup, Time, U, Var, Wbr, Area, Audio,
    Img, Map, Track, Video, Embed, Iframe, Object, Param, Picture, Portal, Source, Svg, Math,
    Canvas, Noscript, Script, Del, Ins, Caption, Col, Colgroup, Table, Tbody, Td, Tfoot, Th, Thead,
    Tr, Button, Datalist, Fieldset, Form, Input, Label, Legend, Meter, Optgroup, Option_, Output,
    Progress, Select, Textarea, Details, Dialog, Menu, Summary, Slot, Template,
];

generate_math_tags![
    Math,
    Mi,
    Mn,
    Mo,
    Ms,
    Mspace,
    Mtext,
    Menclose,
    Merror,
    Mfenced,
    Mfrac,
    Mpadded,
    Mphantom,
    Mroot,
    Mrow,
    Msqrt,
    Mstyle,
    Mmultiscripts,
    Mover,
    Mprescripts,
    Msub,
    Msubsup,
    Msup,
    Munder,
    Munderover,
    Mtable,
    Mtd,
    Mtr,
    Maction,
    Annotation,
    AnnotationXml,
    Semantics,
];

generate_svg_tags![
    A,
    Animate,
    AnimateMotion,
    AnimateTransform,
    Circle,
    ClipPath,
    Defs,
    Desc,
    Discard,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeDropShadow,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    ForeignObject,
    G,
    Hatch,
    Hatchpath,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Metadata,
    Mpath,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Script,
    Set,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Title,
    Tspan,
    Use,
    View,
];
