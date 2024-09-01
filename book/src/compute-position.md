# Compute Position

Computes coordinates to position a floating element next to another element.

{{#tabs global="package" }}
{{#tab name="Core" }}

```rust,ignore
use floating_ui_core::compute_position;
```

-   [View on crates.io](https://crates.io/crates/floating-ui-core)
-   [View on docs.rs](https://docs.rs/floating-ui-core/latest/floating_ui_core/)
-   [View source](https://github.com/RustForWeb/floating-ui/tree/main/packages/core)

{{#endtab }}
{{#tab name="DOM" }}

```rust,ignore
use floating_ui_dom::compute_position;
```

-   [View on crates.io](https://crates.io/crates/floating-ui-dom)
-   [View on docs.rs](https://docs.rs/floating-ui-dom/latest/floating_ui_dom/)
-   [View source](https://github.com/RustForWeb/floating-ui/tree/main/packages/dom)

{{#endtab }}
{{#tab name="Leptos" }}

<div class="warning">

**Non-framework API**

[`use_floating()`](./frameworks/leptos.md) should be used instead with a framework. If you are using a base package, change the package with the package switcher above.

</div>

{{#endtab }}
{{#tab name="Yew" }}

<div class="warning">

**Non-framework API**

[`use_floating()`](./frameworks/yew.md) should be used instead with a framework. If you are using a base package, change the package with the package switcher above.

</div>

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="package" }}
{{#tab name="Core" }}

At its most basic, the function accepts two elements:

-   **Reference element** - also known as the anchor element, this is the element that is being _referred_ to for positioning. Often this is a button that triggers a floating popover like a tooltip or menu.
-   **Floating element** - this is the element that floats next to the reference element, remaining anchored to it. This is the popover or tooltip itself.

```rust,ignore
# use floating_ui_utils::Rect;
#
let reference_el = Rect {width: 100, height: 100, x 50, y: 50};
let floating_el = Rect {width: 200, height: 200, x 0, y: 0};
```

Then, call `compute_position()` with them as arguments, ensuring you pass the required [platform methods](./platform.md).

The first argument is the reference element to anchor to, and the second argument is the floating element to be positioned.

```rust,ignore
use floating_ui_core::{compute_position, ComputePositionConfig, ComputePositionReturn};
use floating_ui_utils::Rect;

let reference_el = Rect {width: 100, height: 100, x 50, y: 50};
let floating_el = Rect {width: 200, height: 200, x 0, y: 0};

let ComputePositionReturn {x, y} = compute_position(reference_el, floating_el, ComputePositionConfig::new(
    // See https://floating-ui.rustforweb.org/platform.html
    platform
));

// Paint the screen.
```

`compute_position()` returns the coordinates that can be used to apply styles to the floating element.

By default, the floating element will be placed at the bottom center of the reference element

{{#endtab }}
{{#tab name="DOM" }}

At its most basic, the function accepts two elements:

-   **Reference element** - also known as the anchor element, this is the element that is being _referred_ to for positioning. Often this is a button that triggers a floating popover like a tooltip or menu.
-   **Floating element** - this is the element that floats next to the reference element, remaining anchored to it. This is the popover or tooltip itself.

```html
<button id="button">My reference element</button>
<div id="tooltip">My floating element</div>
```

First, give the floating element initial CSS styles so that it becomes an absolutely-positioned element that floats on top of the UI with layout ready for being measured:

```css
#tooltip {
    /* Float on top of the UI */
    position: absolute;

    /* Avoid layout interference */
    width: max-content;
    top: 0;
    left: 0;
}
```

Then, ensuring that these two elements are rendered onto the document and can be measured, call `compute_position()` with them as arguments.

The first argument is the reference element to anchor to, and the second argument is the floating element to be positioned.

```rust,ignore
use floating_ui_dom::{compute_position, ComputePositionConfig, ComputePositionReturn};

let window = web_sys::window().expect("Window should exist.");
let document = window.document().expect("Window should have document.");

let button = document
    .get_element_by_id("button")
    .expect("Button should exist.");
let tooltip = document
    .get_element_by_id("tooltip")
    .expect("Tooltip should exist.")
    .unchecked_into::<web_sys::HtmlElement>();

let ComputePositionReturn {x, y} = compute_position(button, &tooltip, ComputePositionConfig::default());

let style = tooltip.style();
style.set_property("left", &format!("{x}px")).expect("Property left should be set.");
style.set_property("top", &format!("{y}px")).expect("Property top should be set");
```

Then, ensuring that these two elements are rendered onto the document and can be measured, call `compute_position()` with them as arguments.

The first argument is the reference element to anchor to, and the second argument is the floating element to be positioned.

By default, the floating element will be placed at the bottom center of the reference element.

### Initial Layout

To ensure positioning works smoothly, the dimensions of the floating element should not change before and after being positioned.

Certain CSS styles must be applied **before** `compute_position()` is called:

```css
#floating {
    position: absolute;
    width: max-content;
    top: 0;
    left: 0;
}
```

-   `position: absolute`

This makes the element float on top of the UI with intrinsic dimensions based on the content, instead of acting as a block. The `top` and `left` coordinates can then take effect.

-   `width: max-content` (or a fixed value)
-   `top: 0`
-   `left: 0`

These properties prevent the floating element from resizing when it overflows a container, removing layout interference that can cause incorrect measurements.

This lets you place the floating element anywhere in the DOM tree and have it be positioned correctly, regardless of the CSS styles of any ancestor containers.

## Anchoring

Since `compute_position()` is only a single function call, it only positions the floating element once.

To ensure it remains anchored to the reference element in a variety of scenarios, such as when resizing or scrolling, wrap the calculation in [`auto_update`](./auto-update.md):

```rust,ignore
use floating_ui_dom::{auto_update, compute_position, AutoUpdateOptions, ComputePositionConfig, ComputePositionReturn};

// When the floating element is mounted to the DOM:
let cleanup = auto_update(reference_el, floating_el, Rc::new(|| {
    let ComputePositionReturn {x, y} = compute_position(reference_el, floating_el, ComputePositionConfig::default());
    // ...
}), AutoUpdateOptions::default());

// ...later, when it's removed from the DOM:
cleanup();
```

{{#endtab }}
{{#tab name="Leptos" }}

<div class="warning">

**Non-framework API**

[`use_floating()`](./frameworks/leptos.md) should be used instead with a framework. If you are using a base package, change the package with the package switcher above.

</div>

{{#endtab }}
{{#tab name="Yew" }}

<div class="warning">

**Non-framework API**

[`use_floating()`](./frameworks/yew.md) should be used instead with a framework. If you are using a base package, change the package with the package switcher above.

</div>

{{#endtab }}
{{#endtabs }}

## Options

Passed as a third argument, this is the struct instance to configure the behavior.

```rust,ignore
compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::default(),
);
```

```rust,ignore
pub struct ComputePositionConfig<Element, Window> {
    pub placement: Option<Placement>,
    pub strategy: Option<Strategy>,
    pub middleware: Option<Vec<Box<dyn Middleware<Element, Window>>>>,
}
```

### `placement`

Where to place the floating element relative to its reference element. By default, this is `Placement::Bottom`.

12 placements are available:

```rust,ignore
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
```

```rust,ignore
compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::default().placement(Placement::BottomStart),
);
```

The `Start` and `End` alignments are [logical](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_logical_properties_and_values) and will adapt to the writing direction (e.g. RTL) as expected.

**Note**

You aren't limited to just these 12 placements though. [`Offset`](./middleware/offset.md) allows you to create any placement.

### `strategy`

This is the type of CSS position property to use. By default, this is `Strategy::Absolute`.

Two strategies are available:

```rust,ignore
pub enum Strategy {
    Absolute,
    Fixed,
}
```

```rust,ignore
compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::default().strategy(Strategy::Fixed),
);
```

Ensure your initial layout matches the strategy:

```css
#tooltip {
    position: fixed;
}
```

These strategies are differentiated as follows:

-   `Strategy::Absolute` - the floating element is positioned relative to its nearest positioned ancestor. With most layouts, this usually requires the browser to do the least work when updating the position.
-   `Strategy::Fixed` - the floating element is positioned relative to its nearest containing block (usually the viewport). This is useful when the reference element is also fixed to reduce jumpiness with positioning while scrolling. It will in many cases also [“break” the floating element out of a clipping ancestor](https://floating-ui.com/docs/misc#clipping).

### `middleware`

When you want granular control over how the floating element is positioned, middleware are used. They read the current coordinates, optionally alter them, and/or provide data for rendering. They compose and work together to produce the final coordinates which you receive as `x` and `y` parameters.

The following are included in the package:

**Placement Modifiers**

These middleware alter the base placement coordinates.

-   [`Offset`](./middleware/offset.md) modifies the placement to add distance or margin between the reference and floating elements.

-   [`Inline`](./middleware/inline.md) positions the floating element relative to individual client rects rather than the bounding box for better precision.

**Visibility Optimizers**

These middleware alter the coordinates to ensure the floating element stays on screen optimally.

-   [`Shift`](./middleware/shift.md) prevents the floating element from overflowing a clipping container by shifting it to stay in view.

-   [`Flip`](./middleware/flip.md) prevents the floating element from overflowing a clipping container by flipping it to the opposite placement to stay in view.

-   [`AutoPlacement`](./middleware/auto-placement.md) automatically chooses a placement for you using a “most space” strategy.

-   [`Size`](./middleware/size.md) resizes the floating element, for example so it will not overflow a clipping container, or to match the width of the reference element.

**Data Providers**

These middleware only provide data and do not alter the coordinates.

-   [`Arrow`](./middleware/arrow.md) provides data to position an inner element of the floating element such that it is centered to its reference element.

-   [`Hide`](./middleware/hide.md) provides data to hide the floating element in applicable situations when it no longer appears attached to its reference element due to different clipping contexts.

**Custom**

You can also craft your own custom middleware to extend the behavior of the library. Read [middleware](./middleware/README.md) to learn how to create your own.

**Conditional**

Middleware can be called conditionally. Often this is useful for higher-level wrapper functions to avoid needing the consumer to import middleware themselves:

```rust,ignore
struct Options {
    enable_flip: bool,
    arrow_el: Option<Element>,
}

fn wrapper(reference_el: Element, floating_el: Element, options: Options) -> ComputePositionReturn {
    let mut middleware = vec![];

    if options.enable_flip {
        middleware.push(Box::new(Flip::new(FlipOptions::default())));
    }
    if let Some(arrow_el) = options.arrow_el {
        middleware.push(Box::new(Arrow::new(ArrowOptions::new(options.arrow_el))));
    }

    compute_position(reference_el, floating_el, ComputePositionConfig::default().middleware(middleware))
}
```

## Return Value

`compute_position()` returns the following struct:

```rust,ignore
pub struct ComputePositionReturn {
    pub x: f64,
    pub y: f64,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: MiddlewareData,
}
```

### `x`

The x-coordinate of the floating element.

### `y`

The y-coordinate of the floating element.

### `placement`

The final placement of the floating element, which may be different from the initial or preferred one passed in due to middleware modifications. This allows you to know which side the floating element is placed at.

### `strategy`

The CSS position property to use.

### `middleware_data`

The data returned by any middleware used.

## See Also

-   [Floating UI documentation](https://floating-ui.com/docs/computePosition)
