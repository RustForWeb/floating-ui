# Shift

Shifts the floating element to keep it in view.

This prevents the floating element from overflowing along its axis of alignment, thereby preserving the side it's placed on.

Type: `Visibility Optimizer`

<!-- TODO: demo -->

## Usage

{{#tabs global="package" }}
{{#tab name="Core" }}

```rust,ignore
use floating_ui_core::{compute_position, ComputePositionConfig, Shift, ShiftOptions};

compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::new(platform)
        .middleware(vec![
            Box::new(Shift::new(ShiftOptions::default())),
        ]),
);
```

{{#endtab }}
{{#tab name="DOM" }}

```rust,ignore
use floating_ui_dom::{compute_position, ComputePositionConfig, Shift, ShiftOptions};

compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::default()
        .middleware(vec![
            Box::new(Shift::new(ShiftOptions::default())),
        ]),
);
```

{{#endtab }}
{{#tab name="Leptos" }}

```rust,ignore
use floating_ui_leptos::{use_floating, Shift, ShiftOptions, UseFloatingOptions};

use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .middleware(vec![
            Box::new(Shift::new(ShiftOptions::default())),
        ].into()),
);
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use floating_ui_yew::{use_floating, Shift, ShiftOptions, UseFloatingOptions};

use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .middleware(vec![
            Box::new(Shift::new(ShiftOptions::default())),
        ]),
);
```

{{#endtab }}
{{#endtabs }}

## Options

These are the options you can pass to `Shift`.

```rust,ignore
pub struct ShiftOptions<Element, Window> {
    pub detect_overflow: Option<DetectOverflowOptions<Element>>,
    pub main_axis: Option<bool>,
    pub cross_axis: Option<bool>,
    pub limiter: Option<Box<dyn Limiter<Element, Window>>>,
}

pub trait Limiter<Element, Window>: Clone + PartialEq {
    fn compute(&self, state: MiddlewareState<Element, Window>) -> Coords;
}
```

### `main_axis`

Default: `true`

This is the main axis in which shifting is applied.

-   `x`-axis for `Top` and `Bottom` placements
-   `y`-axis for `Left` and `Right` placements

```rust,ignore
Shift::new(ShiftOptions::default().main_axis(false))
```

<!-- TODO: demo -->

### `cross_axis`

Default: `false`

This is the cross axis in which shifting is applied, the opposite axis of `main_axis`.

Enabling this can lead to the floating element **overlapping** the reference element, which may not be desired and is often replaced by the `Flip` middleware.

```rust,ignore
Shift::new(ShiftOptions::default().cross_axis(true))
```

<!-- TODO: demo -->

### `limiter`

Default: no-op

This accepts a struct instance that **limits** the shifting done, in order to prevent detachment or “overly-eager” behavior. The behavior is to stop shifting once the opposite edges of the elements are aligned.

```rust,ignore
Shift::new(ShiftOptions::default().limiter(
    Box::new(LimitShift::new(
        LimitShiftOptions::default()
    )),
))
```

This struct itself takes options.

```rust,ignore
pub struct LimitShiftOptions<'a, Element, Window> {
    pub offset: Option<Derivable<'a, Element, Window, LimitShiftOffset>>,
    pub main_axis: Option<bool>,
    pub cross_axis: Option<bool>,
}

pub enum LimitShiftOffset {
    Value(f64),
    Values(LimitShiftOffsetValues),
}

pub struct LimitShiftOffsetValues {
    pub main_axis: Option<f64>,
    pub cross_axis: Option<f64>,
}
```

#### `main_axis`

Default: `true`

Whether to apply limiting on the main axis.

```rust,ignore
Shift::new(ShiftOptions::default().limiter(
    Box::new(LimitShift::new(
        LimitShiftOptions::default().main_axis(false)
    )),
))
```

#### `cross_axis`

Default: `true`

Whether to apply limiting on the cross axis.

```rust,ignore
Shift::new(ShiftOptions::default().limiter(
    Box::new(LimitShift::new(
        LimitShiftOptions::default().cross_axis(false)
    )),
))
```

#### `offset`

Default: `0.0`

This will offset when the limiting starts. A positive number will start limiting earlier, while negative later.

```rust,ignore
Shift::new(ShiftOptions::default().limiter(
    Box::new(LimitShift::new(
        // Start limiting 5px earlier
        LimitShiftOptions::default().offset(LimitShiftOffset::Value(5.0)),
    )),
))
```

<!-- This can also take a function, which provides the `Rect`s of each element to read their dimensions: -->
<!-- TODO: derivable fn -->

You may also pass a struct instance to configure both axes:

```rust,ignore
Shift::new(ShiftOptions::default().limiter(
    Box::new(LimitShift::new(
        LimitShiftOptions::default().offset(LimitShiftOffset::Values(
            LimitShitOffsetValues::default()
                .main_axis(10.0)
                .cross_axis(5.0),
        )),
    )),
))
```

<!-- TODO: derivable fn with values return -->

### `detect_overflow`

All of [`detect_overflow`](../detect-overflow.md)'s options can be passed. For instance:

```rust,ignore
Shift::new(ShiftOptions::default().detect_overflow(
    DetectOvverflowOptions::default().padding(Padding::All(5.0)),
))
```

<!-- If you find the padding does not get applied on the right side, see Handling large content -->

<!-- ### Deriving Options From State -->
<!-- TODO -->

## Data

The following data is available in `middleware_data` under the `SHIFT_NAME` key:

```rust,ignore
middleware_data.get_as::<ShiftData>(SHIFT_NAME)
```

```rust,ignore
pub struct ShiftData {
    pub x: f64,
    pub y: f64,
}
```

`x` and `y` represent how much the floating element has been shifted along that axis. The values are offsets, and therefore can be negative.

## See Also

-   [Floating UI documentation](https://floating-ui.com/docs/shift)
