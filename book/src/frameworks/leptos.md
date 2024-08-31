# Leptos

This package provides [Leptos](https://leptos.dev/) bindings for `floating-ui-dom` - a library that provides anchor positioning for a floating element to position it next to a given reference element.

## Installation

```shell
cargo add floating-ui-leptos
```

-   [View on crates.io](https://crates.io/crates/floating-ui-leptos)
-   [View on docs.rs](https://docs.rs/floating-ui-leptos/latest/floating_ui_leptos/)
-   [View source](https://github.com/RustForWeb/floating-ui/tree/main/packages/leptos)

## Usage

`use_floating` is the main composable:

```rust,ignore
use floating_ui_leptos::{use_floating, UseFloatingOptions, UseFloatingReturn};
use leptos::*;

#[component]
pub fn Example() -> impl IntoView {
    let reference_ref = NodeRef::new();
    let floating_ref = NodeRef::new();

    let UseFloatingReturn {
        floating_styles,
    } = use_floating(reference_ref.into_reference(), floating_ref, UseFloatingOptions::default());

    view! {
        <button _ref=reference_ref>Button</button>
        <div
            _ref=floating_ref
            style:position=move || floating_styles.get().style_position()
            style:top=move || floating_styles.get().style_top()
            style:left=move || floating_styles.get().style_left()
            style:transform=move || floating_styles.get().style_transform()
            style:will-change=move || floating_styles.get().style_will_change()
        >
            Tooltip
        </div>
    }
}
```

This will position the floating `Tooltip` element at the bottom center of the `Button` element by default.

-   `reference` is the reference (or anchor) element that is being referred to for positioning.
-   `floating` is the floating element that is being positioned relative to the reference element.
-   `floating_styles` is a signal of positioning styles to apply to the floating element's `style` attribute.

### Disabling Transform

By default, the floating element is positioned using `transform` in the `floating_styles` struct instance. This is the most performant way to position elements, but can be disabled:

```rust,ignore
use_floating(reference_ref, floating_ref, UseFloatingOptions::default().transform(false.into()));
```

If you'd like to retain transform styles while allowing transform animations, create a wrapper, where the outermost node is the positioned one, and the inner is the actual styled element.

### Custom Position Styles

The composable returns the coordinates and positioning strategy directly if `floating_styles` is not suitable for full customization.

```rust,ignore
let UseFloatingReturn {
    x,
    y,
    strategy,
} = use_floating(reference_ref, floating_ref, UseFloatingOptions::default());
```

## Return Value

The composable [returns all the values from `compute_position`](./compute-position.md#return-value), plus some extras to work with Leptos. This includes data about the final placement and middleware data which are useful when rendering.

## Options

The composable accepts all the [options from `compute_position`](./compute-position.md#options), which allows you to customize the position. Here's an example:

```rust,ignore
use floating_ui_leptos::{
    use_floating, Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions, UseFloatingOptions
};

let middleware: MiddlewareVec = vec![
    Box::new(Offset::new(OffsetOptions::Value(10.0))),
    Box::new(Flip::new(FlipOptions::default())),
    Box::new(Shift::new(ShiftOptions::default())),
];

use_floating(
    reference_ref,
    floating_ref,
    UseFloatingOptions::default()
        .placement(Placement::Right.into())
        .middleware(middleware.into())
);
```

The composable also accepts `Signal` options:

```rust,ignore
use floating_ui_leptos::{
    use_floating, Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions, UseFloatingOptions
};
use leptos::*;

let placement = Signal::derive(move || Placement::Right);

let middleware: Signal<MiddlewareVec> = Signal::derive(move || vec![
    Box::new(Offset::new(OffsetOptions::Value(10.0))),
    Box::new(Flip::new(FlipOptions::default())),
    Box::new(Shift::new(ShiftOptions::default())),
]);

use_floating(
    reference_ref,
    floating_ref,
    UseFloatingOptions::default()
        .placement(placement.into())
        .middleware(middleware.into())
);
```

[Middleware](./middleware/README.md) can alter the positioning from the basic `placement`, act as visibility optimizers, or provide data to use.

The docs for the middleware that were passed are available here:

-   [`Offset`](./middleware/offset.md)
-   [`Flip`](./middleware/flip.md)
-   [`Shift`](./middleware/shift.md)

## Anchoring

The position is only calculated **once** on render, or when the `reference` or `floating` elements changed - for example, the floating element get mounted via conditional rendering.

To ensure the floating element remains anchored to its reference element in a variety of scenarios without detaching - such as when scrolling or resizing the page - you can pass the [`auto_update`](./auto-update.md) utility to the `while_elements_mounted` option:

```rust,ignore
use_floating(
    reference_ref,
    floating_ref,
    UseFloatingOptions::default().while_elements_mounted_auto_update(),
);
```

To pass options to `auto_update`:

```rust,ignore
use_floating(
    reference_ref,
    floating_ref,
    UseFloatingOptions::default().while_elements_mounted_auto_update_with_options(
        AutoUpdateOptions::default().animation_frame(true).into()
    ),
);
```

Ensure you are using conditional rendering (`Show`) for the floating element, not an opacity/visibility/display style. If you are using the latter, avoid the `while_elements_mounted` option.

### Manual Updating

While `auto_update` covers most cases where the position of the floating element must be updated, it does not cover every single one possible due to performance/platform limitations.

The composable returns an `update()` function to update the position at will:

```rust,ignore
let UseFloatingReturn {
    update,
} = use_floating(reference_ref, floating_ref, UseFloatingOptions::default());

view! {
    <button on:click=move || update()>Update</button>
}
```

<!-- ## Custom components -->

## Effects

TODO

## Arrow

TODO

## Virtual Element

TODO
