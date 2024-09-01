# Virtual Elements

Position a floating element relative to a custom reference area, useful for context menus, range selections, following the cursor, and more.

## Usage

A virtual element must implement the `VirtualElement` trait.

```rust,ignore
pub trait VirtualElement<Element>: Clone + PartialEq {
    fn get_bounding_client_rect(&self) -> ClientRectObject;

    fn get_client_rects(&self) -> Option<Vec<ClientRectObject>>;

    fn context_element(&self) -> Option<Element>;
}
```

A default implementation called `DefaultVirtualElement` is provided for convience.

```rust,ignore
let virtual_el: Box<dyn VirtualElement<Element>> = Box::new(
    DefaultVirtualElement::new(get_bounding_client_rect)
        .get_client_rects(get_client_rects)
        .context_element(context_element),
);
```

{{#tabs global="package" }}
{{#tab name="Core" }}

```rust,ignore
compute_position(virtual_el.into(), floating_el, ComputePositionConfig::new(platform))
```

{{#endtab }}
{{#tab name="DOM" }}

```rust,ignore
compute_position(virtual_el.into(), floating_el, ComputePositionConfig::default())
```

{{#endtab }}
{{#tab name="Leptos" }}

```rust,ignore
use_floating(virtual_el.into(), floating_el, UseFloatingOptions::default())
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use_floating(virtual_el.into(), floating_el, UseFloatingOptions::default())
```

{{#endtab }}
{{#endtabs }}

### `get_bounding_client_rect`

The most basic virtual element is a plain object that has a `get_bounding_client_rect` method, which mimics a real element's one:

```rust,ignore
// A virtual element that is 20 x 20 px starting from (0, 0)
let virtual_el: Box<dyn VirtualElement<Element>> = Box::new(
    DefaultVirtualElement::new(Rc::new(|| {
        ClientRectObject {
            x: 0.0,
            y: 0.0,
            top: 0.0,
            left: 0.0,
            bottom: 20.0,
            right: 20.0,
            width: 20.0,
            height: 20.0,
        }
    }))
);
```

<!-- A point reference, such as a mouse event, is one such use case: -->
<!-- TODO -->

### `context_element`

This option is useful if your `get_bounding_client_rect` method is derived from a real element, to ensure clipping and position update detection works as expected.

```rust,ignore
let virtual_el: Box<dyn VirtualElement<Element>> = Box::new(
    DefaultVirtualElement::new(get_bounding_client_rect)
        .context_element(
            web_sys::window()
                .expext("Window should exist.")
                .document()
                .expect("Document should exist.")
                .query_selector("#context")
                .expect("Document should be queried.")
                .expect("Element should exist."),
        ),
);
```

### `get_client_rects`

This option is useful when using [range selections](https://developer.mozilla.org/en-US/docs/Web/API/Range) and the `Inline` middleware.

```rust,ignore
let virtual_el: Box<dyn VirtualElement<Element>> = Box::new(
    DefaultVirtualElement::new(|| range.get_bounding_client_rect().into())
        .get_client_rects(|| ClientRectObject::from_dom_rect_list(
            range.get_client_rects().expect("Range should have client rects."),
        )),
);
```

## See Also

-   [Floating UI documentation](https://floating-ui.com/docs/virtual-elements)
