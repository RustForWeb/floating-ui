# Platform

Use Floating UI's positioning logic on any platform that can execute Rust.

Floating UI's core is essentially a bunch of mathematical calculations performed on rectangles. These calculations are pure and agnostic, allowing Floating UI to work on any platform that can execute Rust.

To make it work with a given platform, methods are used to allow it to hook into measurement APIs, for instance, to measure the bounding box of a given element.

Possible platforms other than the DOM include Native, Canvas/WebGL, etc.

<!-- This is Floating UI running in a pure `<canvas />` element! -->
<!-- TODO: demo -->

## Custom Platform Struct

If you're building a platform from scratch, e.g. your own tiny custom DOM platform, you'll be using the `floating-ui-core` package - see [Methods](#methods).

If you're extending or customizing the existing DOM methods, and are using `floating-ui-dom`, this is accessible via the `Platform` import - see [Extending the DOM Platform](#extending-the-dom-platform).

## Shadow DOM Fix

TODO

## Concepts

The library works largely with a `Rect`:

```rust,ignore
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
```

This data can come from anywhere, and the library will perform the right computations. `x` and `y` represent the coordinates of the element relative to another one.

## Methods

A `platform` is a struct implementing the `Platform` trait, which consists of 3 required and 7 optional methods. These methods allow the platform to interface with Floating UI's logic.

The `Platform<Element, Window>` trait has two generic types that reflect the element and window of the platform. The DOM platform uses `web_sys::Element` and `web_sys::Window`.

### Required Methods

#### `get_element_rects`

Takes in the elements and the positioning `strategy` and returns the element `Rect` struct instances.

```rust,ignore
pub fn get_element_rects(
    GetElementRectsArgs {
        reference,
        floating,
        strategy,
    }: GetElementRectsArgs<Element>
) -> ElementRects {
    ElementRects {
        reference: Rect {x: 0.0, y: 0.0, width: 0.0, height: 0.0},
        floating: Rect {x: 0.0, y: 0.0, width: 0.0, height: 0.0},
    }
}
```

**`reference`**

The `x` and `y` values of a reference `Rect` should be its coordinates relative to the floating element's `offset_parent` element if required rather than the viewport.

**`floating`**

Both `x` and `y` are not relevant initially, so you can set these both of these to `0.0`.

#### `get_dimensions`

```rust,ignore
pub fn get_dimensions(element: &Element) -> Dimensions {
    Dimensions {
        width: 0.0,
        height: 0.0,
    }
}
```

#### `get_clipping_rect`

Returns the `Rect` (r**elative to the viewport**) whose outside bounds will clip the given element. For instance, the viewport itself.

```rust,ignore
pub fn get_clipping_rect(
    GetClippingRectArgs {
        element,
        boundary,
        root_boundary,
        strategy,
    }: GetClippingRectArgs<Element>,
) -> Rect {
    Rect {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    }
}
```

### Optional Methods

Depending on the platform you're working with, these may or may not be necessary.

#### `convert_offset_parent_relative_rect_to_viewport_relative_rect`

This function will take a `Rect` that is relative to a given `offset_parent` element and convert its `x` and `y` values such that it is instead relative to the viewport.

```rust,ignore
pub fn convert_offset_parent_relative_rect_to_viewport_relative_rect(
    ConvertOffsetParentRelativeRectToViewportRelativeRectArgs {
        elements,
        rect,
        offset_parent,
        strategy,
    }: ConvertOffsetParentRelativeRectToViewportRelativeRectArgs<Element, Window>,
) -> Rect {
    rect
}
```

#### `get_offset_parent`

Returns the `offset_parent` of a given element.

```rust,ignore
pub fn get_offset_parent(
    element: &Element,
    polyfill: Option<Polyfill>,
) -> OwnedElementOrWindow<Element, Window> {
    OwnedElementOrWindow::Window(window)
}
```

The polyfill parameter exists only for `floating-ui-dom` and is optional to fix the
[Shadow DOM Bug](#shadow-dom-fix).

#### `get_document_element`

Returns the document element.

```rust,ignore
fn get_document_element(element: &Element) -> Element {
    document_element
}
```

#### `get_client_rects`

Returns a vector of `ClientRect`s.

```rust,ignore
fn get_client_rects(element: ElementOrVirtual) -> Vec<ClientRectObject> {
    vec![]
}
```

#### `is_rtl`

Determines if an element is in RTL layout.

```rust,ignore
fn is_rtl(element: &Element) -> bool {
    false
}
```

#### `get_scale`

Determines the scale of an element.

```rust,ignore
fn get_scale(element: &Element) -> Coords {
    Coords {
        x: 1.0,
        y: 1.0
    }
}
```

#### `get_client_length`

Returns the client width or height of an element.

```rust,ignore
fn get_client_length(element: &Element, length: Length) -> f64 {
    match length {
        Length::Width => 0.0,
        Length::Height => 0.0,
    }
}
```

## Usage

All these methods are passed in the implementation of the `Platform` trait.

```rust,ignore
use floating_ui_core::{compute_position, ComputePositionConfig, Platform};

#[derive(Debug)]
pub struct CustomPlatform {}

impl Platform<Element, Window> for CustomPlatform {
    // Required

    fn get_element_rects(&self, args: GetElementRectsArgs<Element>) -> ElementRects {
        get_element_rects(args)
    }

    fn get_dimensions(&self, element: &Element) -> Dimensions {
        get_dimensions(element)
    }

    fn get_clipping_rect(&self, args: GetClippingRectArgs<Element>) -> Rect {
        get_clipping_rect(args)
    }

    // Optional (pass `None` if not implemented)

    fn convert_offset_parent_relative_rect_to_viewport_relative_rect(
        &self,
        args: ConvertOffsetParentRelativeRectToViewportRelativeRectArgs<Element, Window>,
    ) -> Option<Rect> {
        Some(convert_offset_parent_relative_rect_to_viewport_relative_rect(args))
    }

    fn get_offset_parent(
        &self,
        element: &Element,
    ) -> Option<OwnedElementOrWindow<Element, Window>> {
        Some(get_offset_parent(element, None))
    }

    fn get_document_element(&self, element: &Element) -> Option<Element> {
        Some(get_document_element(element))
    }

    fn get_client_rects(&self, element: ElementOrVirtual) -> Option<Vec<ClientRectObject>> {
        Some(get_client_rects(element))
    }

    fn is_rtl(&self, element: &Element) -> Option<bool> {
        Some(is_rtl(element))
    }

    fn get_scale(&self, element: &Element) -> Option<Coords> {
        Some(get_scale(element))
    }

    fn get_client_length(
        &self,
        element: &Element,
        length: floating_ui_utils::Length,
    ) -> Option<f64> {
        Some(get_client_length(element, length))
    }
}

const PLATFORM: CustomPlatform = CustomPlatform {};

compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::new(&PLATFORM),
);
```

### Extending the DOM Platform

```rust,ignore
use floating_ui_core::{compute_position, ComputePositionConfig, Platform};
use floating_ui_dom::Platform as DomPlatform;
use web_sys::{Element, Window};

#[derive(Debug)]
struct CustomPlatform {
    dom_platform: DomPlatform,
}

impl Platform<Element, Window> for CustomPlatform {
    // Use existing DOM methods.
    fn get_element_rects(&self, args: GetElementRectsArgs<Element>) -> ElementRects {
        self.dom_platform.get_element_rects(self, args)
    }

    // Overwrite methods with your own.
    fn get_dimensions(&self, element: &Element) -> Dimensions {
        Dimensions {
            width: 0.0,
            height: 0.0,
        }
    }

    // Etc.
}

const PLATFORM: CustomPlatform = CustomPlatform {
    dom_platform: DomPlatform {}
};

compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::new(&PLATFORM),
);
```

## See Also

-   [Floating UI documentation](https://floating-ui.com/docs/platform)
