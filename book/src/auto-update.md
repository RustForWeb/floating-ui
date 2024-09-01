# Auto Update

Automatically updates the position of the floating element when necessary to ensure it stays anchored.

To ensure the floating element remains anchored to its reference element, such as when scrolling and resizing the screen, its position needs to be continually updated when necessary.

To solve this, `auto_update()` adds listeners that will automatically call an update function which invokes `compute_position()` when necessary. <!-- Updates typically take only ~1ms. -->

## Usage

It's important that this function is only called/set-up when the floating element is open on the screen, and cleaned up when it's removed. Otherwise, it can cause severe performance degradation, especially with many floating elements being created.

{{#tabs global="package" }}
{{#tab name="Core" }}

<div class="warning">

**Unavailable API**

This is a DOM API and is therefore not available for `floating-ui-core`. If you are not using that package, change the package with the package switcher above.

</div>

{{#endtab }}
{{#tab name="DOM" }}

Call `auto_update()` **only** when the floating element is open or mounted:

```rust,ignore
use std::rc::Rc;

use floating_ui_dom::{auto_update, compute_position, AutoUpdateOptions, ComputePositionConfig, ComputePositionReturn};

// This closure will get called repeatedly.
let update = Rc::new(move || {
    let ComputeReturn {x, y, ..} = compute_position(reference_el, floating_el, ComputePositionConfig::default());
    // ...
});

// Mount the floating element to the DOM, such as on hover or click.
web_sys::window()
    .expect("Window should exist.")
    .document()
    .expect("Document should exist.")
    .body()
    .expect("Body should exist.")
    .append_with_node_1(floating_el)
    .expect("Element should be appended.");

// Start auto updates.
let cleanup = auto_update(
    reference_el,
    floating_el,
    update,
    AutoUpdateOptions::default(),
);
```

Then, when the user unhovers or clicks away, unmount the floating element and ensure you call the cleanup function to stop the auto updates:

```rust,ignore
// Remove the floating element from the DOM, such as on blur or outside click.
floating_el.remove();

// Stop auto updates.
cleanup();
```

{{#endtab }}
{{#tab name="Leptos" }}

If you're conditionally rendering the floating element (recommended), use the `while_elements_mounted` option:

```rust,ignore
use floating_ui_leptos::{use_floating, UseFloatingOptions};

use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default().while_elements_mounted_auto_update(),
);
```

`while_elements_mounted` automatically handles calling and cleaning up `auto_update` based on the presence of the reference and floating element.

See the documentation on [docs.rs](https://docs.rs/floating-ui-leptos/latest/floating_ui_leptos/struct.UseFloatingOptions.html#method.while_elements_mounted) for all alternatives of the `while_elements_mounted` convenience method.

{{#endtab }}
{{#tab name="Yew" }}

If you're conditionally rendering the floating element (recommended), use the `while_elements_mounted` option:

```rust,ignore
use floating_ui_yew::{use_auto_update, use_floating, UseFloatingOptions};

let auto_update = use_auto_update();

use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default().while_elements_mounted((*auto_update).clone()),
);
```

`while_elements_mounted` automatically handles calling and cleaning up `auto_update` based on the presence of the reference and floating element.

See the documentation on [docs.rs](https://docs.rs/floating-ui-yew/latest/floating_ui_yew/#functions) for all alternatives of the `use_auto_update` hook.

{{#endtab }}
{{#endtabs }}

## Options

These are the options you can pass as a fourth argument to `auto_update()`.

```rust,ignore
pub struct AutoUpdateOptions {
    pub ancestor_scroll: Option<bool>,
    pub ancestor_resize: Option<bool>,
    pub element_resize: Option<bool>,
    pub layout_shift: Option<bool>,
    pub animation_frame: Option<bool>,
}
```

### `ancestor_scroll`

Default: `true`

Whether to update the position when an overflow ancestor is scrolled.

```rust,ignore
auto_update(
    reference_el,
    floating_el,
    update,
    AutoUpdateOptions::default().ancestor_scroll(false),
);
```

### `ancestor_resize`

Default: `true`

Whether to update the position when an overflow ancestor is resized. This uses the `resize` event.

```rust,ignore
auto_update(
    reference_el,
    floating_el,
    update,
    AutoUpdateOptions::default().ancestor_resize(false),
);
```

### `element_resize`

Default: `true`

Whether to update the position when either the reference or floating elements resized. This uses a `ResizeObserver`.

```rust,ignore
auto_update(
    reference_el,
    floating_el,
    update,
    AutoUpdateOptions::default().element_resize(false),
);
```

### `layout_shift`

Default: `true`

Whether to update the position of the floating element if the reference element moved on the screen as the result of layout shift. This uses a `IntersectionObserver`.

```rust,ignore
auto_update(
    reference_el,
    floating_el,
    update,
    AutoUpdateOptions::default().layout_shift(false),
);
```

### `animation_frame`

Default: `false`

Whether to update the position of the floating element on every animation frame if required. While optimized for performance, it should be used sparingly in the following cases:

-   The reference element is animating on the screen with `transform`s.
-   Ensure a nested floating element is anchored when it's outside of ancestor floating elements' scrolling contexts.

```rust,ignore
auto_update(
    reference_el,
    floating_el,
    update,
    AutoUpdateOptions::default().animation_frame(true),
);
```

## See Also

-   [Floating UI documentation](https://floating-ui.com/docs/autoUpdate)
