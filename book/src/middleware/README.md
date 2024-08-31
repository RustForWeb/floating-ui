# Middleware

Objects that change the positioning of the floating element, executed in order as a queue.

Middleware allow you to customize the behavior of the positioning and be as granular as you want, adding your own custom logic.

`compute_position()` starts with initial positioning via placement - then middleware are executed as an in-between "middle" step of the initial placement computation and eventual return of data for rendering.

Each middleware is executed in order:

{{#tabs global="package" }}
{{#tab name="Core" }}

```rust,ignore
compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::new(platform)
        .position(Placement::Right)
        .middleware(vec![]),
);
```

{{#endtab }}
{{#tab name="DOM" }}

```rust,ignore
compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::default()
        .position(Placement::Right)
        .middleware(vec![]),
);
```

{{#endtab }}
{{#tab name="Leptos" }}

```rust,ignore
use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .position(Placement::Right.into())
        .middleware(vec![].into()),
);
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .position(Placement::Right)
        .middleware(vec![]),
);
```

{{#endtab }}
{{#endtabs }}

## Example

```rust,ignore
use floating_ui_core::{Middleware, MiddlewareReturn, MiddlewareState};

const SHIFT_BY_ONE_PIXEL_NAME: &str = "shiftByOnePixel";

#[derive(Clone, PartialEq)]
struct ShiftByOnePixel {}

impl ShiftByOnePixel {
    pub fn new() -> Self {
        ShiftByOnePixel {}
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Middleware<Element, Window>
    for ShiftByOnePixel
{
    fn name(&self) -> &'static str {
        SHIFT_BY_ONE_PIXEL_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        MiddlewareReturn {
            x: Some(state.x + 1.0),
            y: Some(state.y + 1.0),
            data: None,
            reset: None,
        }
    }
}
```

This (not particularly useful) middleware adds `1` pixel to the coordinates. To use this middleware, add it to your `middleware` vector:

{{#tabs global="package" }}
{{#tab name="Core" }}

```rust,ignore
compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::new(platform)
        .position(Placement::Right)
        .middleware(vec![
            Box::new(ShiftByOnePixel::new())
        ]),
);
```

{{#endtab }}
{{#tab name="DOM" }}

```rust,ignore
compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::default()
        .position(Placement::Right)
        .middleware(vec![
            Box::new(ShiftByOnePixel::new())
        ]),
);
```

{{#endtab }}
{{#tab name="Leptos" }}

```rust,ignore
use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .position(Placement::Right.into())
        .middleware(vec![
            Box::new(ShiftByOnePixel::new())
        ].into()),
);
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .position(Placement::Right)
        .middleware(vec![
            Box::new(ShiftByOnePixel::new())
        ]),
);
```

{{#endtab }}
{{#endtabs }}

Here, `compute_position()` will compute coordinates that will place the floating element to the right center of the reference element, lying flush with it.

Middleware are then executed, resulting in these coordinates getting shifted by one pixel. Then that data is returned for rendering.

### Shape

A middleware is a struct that implements the `Middleware` trait. It has a `name` and a `compute` method The `compute` method provides the logic of the middleware, which returns new positioning coordinates or useful data.

### Data

Any data can be passed via an optional `data` field of the struct instance that is returned from `compute`. This will be accessible to the consumer via the `middleware_data` field:

```rust,ignore
use floating_ui_core::{Middleware, MiddlewareReturn, MiddlewareState};
use serde::{Deserialize, Serialize};

const SHIFT_BY_ONE_PIXEL_NAME: &str = "shiftByOnePixel";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ShiftByOnePixelData {
    pub amount: f64,
}

#[derive(Clone, PartialEq)]
struct ShiftByOnePixel {}

impl ShiftByOnePixel {
    pub fn new() -> Self {
        ShiftByOnePixel {}
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Middleware<Element, Window>
    for ShiftByOnePixel
{
    fn name(&self) -> &'static str {
        SHIFT_BY_ONE_PIXEL_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        MiddlewareReturn {
            x: Some(state.x + 1.0),
            y: Some(state.y + 1.0),
            data: Some(
                serde_json::to_value(ShiftByOnePixelData {
                    amount: 1.0,
                })
                .expect("Data should be valid JSON."),
            ),
            reset: None,
        }
    }
}
```

{{#tabs global="package" }}
{{#tab name="Core" }}

```rust,ignore
let ComputePositionReturn {
    middleware_data,
    ..
} = compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::new(platform)
        .position(Placement::Right)
        .middleware(vec![
            Box::new(ShiftByOnePixel::new())
        ]),
);

if let Some(data) = middleware_data.get_as::<ShiftByOnePixelData>(SHIFT_BY_ONE_PIXEL_NAME) {
    log::info!("{:#?}", data);
}
```

{{#endtab }}
{{#tab name="DOM" }}

```rust,ignore
let ComputePositionReturn {
    middleware_data,
    ..
} = compute_position(
    reference_el,
    floating_el,
    ComputePositionConfig::default()
        .position(Placement::Right)
        .middleware(vec![
            Box::new(ShiftByOnePixel::new())
        ]),
);

if let Some(data) = middleware_data.get_as::<ShiftByOnePixelData>(SHIFT_BY_ONE_PIXEL_NAME) {
    log::info!("{:#?}", data);
}
```

{{#endtab }}
{{#tab name="Leptos" }}

```rust,ignore
let UseFloatingReturn {
    middleware_data,
    ..
} = use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .position(Placement::Right.into())
        .middleware(vec![
            Box::new(ShiftByOnePixel::new())
        ].into()),
);

if let Some(data) = middleware_data.get_as::<ShiftByOnePixelData>(SHIFT_BY_ONE_PIXEL_NAME) {
    log::info!("{:#?}", data);
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
let UseFloatingReturn {
    middleware_data,
    ..
} = use_floating(
    reference_el,
    floating_el,
    UseFloatingOptions::default()
        .position(Placement::Right)
        .middleware(vec![
            Box::new(ShiftByOnePixel::new())
        ]),
);

if let Some(data) = middleware_data.get_as::<ShiftByOnePixelData>(SHIFT_BY_ONE_PIXEL_NAME) {
    log::info!("{:#?}", data);
}
```

{{#endtab }}
{{#endtabs }}

### Options

Options can be passed to the middleware and stored in the struct:

```rust,ignore
use floating_ui_core::{Middleware, MiddlewareReturn, MiddlewareState};

const SHIFT_BY_ONE_PIXEL_NAME: &str = "shiftByOnePixel";

#[derive(Clone, Debug, Default, PartialEq)]
struct ShiftByOnePixelOptions {
    amount: f64,
}

impl ShiftByOnePixelOptions {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = value;
        self
    }
}

#[derive(Clone, PartialEq)]
struct ShiftByOnePixel {
    options: ShiftByOnePixelOptions,
}

impl ShiftByOnePixel {
    pub fn new(options: ShiftByOnePixelOptions) -> Self {
        ShiftByOnePixel {
            options,
        }
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Middleware<Element, Window>
    for ShiftByOnePixel
{
    fn name(&self) -> &'static str {
        SHIFT_BY_ONE_PIXEL_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        MiddlewareReturn {
            x: Some(state.x + self.options.amount),
            y: Some(state.y + self.options.amount),
            data: None,
            reset: None,
        }
    }
}
```

The options can be passed to the middleware to configure the behavior:

```rust,ignore
let middleware = vec![Box::new(ShiftByOnePixel::new(ShiftByOnePixelOptions::default().amount(10)))];
```

## Middleware State

A struct instance is passed to `compute` containing useful data about the middleware lifecycle being executed.

In the previous examples, we used `x` and `y` out of the `compute` parameter struct. These are only two fields that get passed into middleware, but there are many more.

The fields passed are below:

```rust,ignore
pub struct MiddlewareState<'a, Element: Clone + 'static, Window: Clone> {
    pub x: f64,
    pub y: f64,
    pub initial_placement: Placement,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: &'a MiddlewareData,
    pub elements: Elements<'a, Element>,
    pub rects: &'a ElementRects,
    pub platform: &'a dyn Platform<Element, Window>,
}
```

### `x`

This is the x-axis coordinate to position the floating element to.

### `y`

This is the y-axis coordinate to position the floating element to.

### `elements`

This is a struct instance containing the reference and floating elements.

### `rects`

This is a struct instance containing the `Rect`s of the reference and floating elements, a struct of shape `{x, y, width, height}`.

### `middleware_data`

This is a struct instance containing all the data of any middleware at the current step in the lifecycle. The lifecycle loops over the `middleware` vector, so later middleware have access to data from any middleware run prior.

### `strategy`

The positioning strategy.

### `initial_placement`

The initial (or preferred) placement passed in to `compute_position()`.

### `placement`

The stateful resultant placement. Middleware like `Flip` change `initial_placement` to a new one.

### `platform`

A struct instance containing methods to make Floating UI work on the current platform, e.g. DOM.

## Ordering

The order in which middleware are placed in the vector matters, as middleware **use** the coordinates that were returned from previous ones. This means they perform their work based on the current positioning state.

Three `ShiftByOnePixel` in the middleware vector means the coordinates get shifted by 3 pixels in total:

```rust,ignore
let middleware = vec![
    Box::new(ShiftByOnePixel::new()),
    Box::new(ShiftByOnePixel::new()),
    Box::new(ShiftByOnePixel::new()),
];
```

If the later `ShiftByOnePixel` implementations had a condition based on the current value of `x` and `y`, the condition can change based on their placement in the vector.

Understanding this can help in knowing which order to place middleware in, as placing a middleware before or after another can produce a different result.

In general, `Offset` should always go at the beginning of the middleware vector, while `Arrow` and `Hide` at the end. The other core middleware can be shifted around depending on the desired behavior.

```rust,ignore
let middleware = vec![
    Box::new(Offset::new(OffsetOptions::default())),
    // ...
    Box::new(Arrow::new(ArrowOptions::new(arrow_element))),
    Box::new(Hide::new(HideOptions::default())),
];
```

## Resetting the Lifecycle

There are use cases for needing to reset the middleware lifecycle so that other middleware perform fresh logic.

-   When `Flip` and `AutoPlacement` change the placement, they reset the lifecycle so that other middleware that modify the coordinates based on the current `placement` do not perform stale logic.
-   `Size` resets the lifecycle with the newly applied dimensions, as many middleware read the dimensions to perform their logic.
-   `Inline` resets the lifecycle when it changes the reference rect to a custom implementation, similar to a [Virtual Element](../virtual-elements.md).

In order to do this, add a `reset` field to the struct instance returned from `compute`.

```rust,ignore
pub enum Reset {
    True,
    Value(ResetValue),
}

pub struct ResetValue {
    pub placement: Option<Placement>,
    pub rects: Option<ResetRects>,
}

// `True` will compute the new `rects` if the dimensions were mutated.
// Otherwise, you can return your own new rects.
pub enum ResetRects {
    True,
    Value(ElementRects),
}
```

```rust,ignore
const SOME_NAME: &str = "some";

#[derive(Clone, PartialEq)]
struct SomeMiddleware {}

impl SomeMiddleware {
    pub fn new() -> Self {
        SomeMiddleware {}
    }
}

impl<Element: Clone + PartialEq, Window: Clone + PartialEq> Middleware<Element, Window>
    for SomeMiddleware
{
    fn name(&self) -> &'static str {
        SOME_NAME
    }

    fn compute(&self, state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
        if some_condition {
            MiddlewareReturn {
                x: None,
                y: None,
                data: None,
                reset: Some(Reset::Value(ResetValue {
                    placements: Some(next_placement),
                    reset: None
                })),
            }
        } else {
            MiddlewareReturn {
                x: None,
                y: None,
                data: None,
                reset: None,
            }
        }
    }
}
```

Data supplied to `middleware_data` is preserved by doing this, so you can read it at any point after you've reset the lifecycle.

## See Also

-   [Floating UI documentation](https://floating-ui.com/docs/middleware)
