# Offset

Translates the floating element along the specified axes.

This lets you add distance (margin or spacing) between the reference and floating element, slightly alter the placement, or even create
custom placements.

Type: `Placement Modifier`

<!-- TOOD: demo -->

## Usage

{{#tabs global="package" }}
{{#tab name="Core" }}

```rust,ignore
use floating_ui_core::{compute_position, ComputePositionConfig, Offset, OffsetOptions};

compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::new(platform)
        .middleware(vec![
            Box::new(Offset::new(OffsetOptions::default())),
        ]),
);
```

{{#endtab }}
{{#tab name="DOM" }}

```rust,ignore
use floating_ui_dom::{compute_position, ComputePositionConfig, Offset, OffsetOptions};

compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::default()
        .middleware(vec![
            Box::new(Offset::new(OffsetOptions::default())),
        ]),
);
```

{{#endtab }}
{{#tab name="Leptos" }}

```rust,ignore
use floating_ui_leptos::{use_floating, Offset, OffsetOptions, UseFloatingOptions};

use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .middleware(vec![
            Box::new(Offset::new(OffsetOptions::default())),
        ].into()),
);
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use floating_ui_yew::{use_floating, Offset, OffsetOptions, UseFloatingOptions};

use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .middleware(vec![
            Box::new(Offset::new(OffsetOptions::default())),
        ]),
);
```

{{#endtab }}
{{#endtabs }}

The value(s) passed are [logical](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_logical_properties_and_values), meaning their effect on the physical result is dependent on the placement, writing direction (e.g. RTL), or alignment.

## Order

`Offset` should generally be placed at the beginning of your middleware vector.

## Options

These are the options you can pass to `Offset`.

```rust,ignore
pub enum OffsetOptions {
    Value(f64),
    Values(OffsetOptionsValues),
}

pub struct OffsetOptionsValues {
    pub main_axis: Option<f64>,
    pub cross_axis: Option<f64>,
    pub alignment_axis: Option<f64>,
}
```

A single number represents the distance (gutter or margin) between the floating element and the reference element. This is shorthand for `main_axis`.

```rust,ignore
Offset::new(OffsetOptions::Value(10.0))
```

A struct instance can also be passed, which enables you to individually configure each axis.

### `main_axis`

Default: `0.0`

The axis that runs along the side of the floating element. Represents the distance (gutter or margin) between the floating element and the reference element.

```rust,ignore
Offset::new(OffsetOptions::Values(
    OffsetOptionsValues::default().main_axis(10.0)
))
```

<!-- Here's how it looks on the four sides: TOOD: demo -->

### `cross_axis`

Default: `0.0`

The axis that runs along the alignment of the floating element. Represents the skidding between the floating element and the reference element.

```rust,ignore
Offset::new(OffsetOptions::Values(
    OffsetOptionsValues::default().cross_axis(20.0)
))
```

<!-- Here's how it looks on the four sides: TOOD: demo -->

### `alignment_axis`

Default: `None`

The same axis as `cross_axis` but applies only to aligned placements and inverts the `End` alignment. When set to a number, it overrides the `cross_axis` value.

A positive number will move the floating element in the direction of the opposite edge to the one that is aligned, while a negative number the reverse.

```rust,ignore
Offset::new(OffsetOptions::Values(
    OffsetOptionsValues::default().alignment_axis(20.0)
))
```

<!-- Here's how it differentiates from crossAxis: TODO: demo -->

<!-- ## Creating Custom Placements

While you can only choose 12 different placements as part of the core library, you can use the `Offset` middleware to create **any** placement you want.

For example, although the library doesn't provide a placement for centering on both axes, offset enables this via the function option by allowing you to read the rects:

```rust,ignore
Offset::new_derivable_fn()
``` -->

## See Also

-   [Floating UI documentation](https://floating-ui.com/docs/offset)
