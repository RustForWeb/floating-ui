use floating_ui_utils::{Coords, Placement, Strategy};

use crate::compute_coords_from_placement::compute_coords_from_placement;
use crate::types::{
    ComputePositionConfig, ComputePositionReturn, Elements, GetElementRectsArgs, MiddlewareData,
    MiddlewareReturn, MiddlewareState, Reset, ResetRects,
};

pub fn compute_position<Element>(
    reference: &Element,
    floating: &Element,
    config: ComputePositionConfig<Element>,
) -> ComputePositionReturn {
    let placement = config.placement.unwrap_or(Placement::Bottom);
    let strategy = config.strategy.unwrap_or(Strategy::Absolute);
    let platform = config.platform;
    let middlewares = config.middleware.unwrap_or_default();

    let rtl = platform.is_rtl(floating);

    let mut rects = platform.get_element_rects(GetElementRectsArgs {
        reference,
        floating,
        strategy,
    });
    let Coords { mut x, mut y } = compute_coords_from_placement(&rects, placement, rtl);
    let mut stateful_placement = placement;
    let mut middleware_data = MiddlewareData::default();
    let mut reset_count = 0;

    let mut i = 0;
    while i < middlewares.len() {
        let middleware = &middlewares[i];

        let MiddlewareReturn {
            x: next_x,
            y: next_y,
            data,
            reset,
        } = middleware.compute(MiddlewareState {
            x,
            y,
            initial_placement: placement,
            placement: stateful_placement,
            strategy,
            middleware_data: &middleware_data,
            rects: &rects,
            platform,
            elements: &Elements {
                reference: &reference,
                floating: &floating,
            },
        });

        x = next_x.unwrap_or(x);
        y = next_y.unwrap_or(y);

        if let Some(data) = data {
            middleware_data.set(middleware.name(), data);
        }

        if let Some(reset) = reset {
            if reset_count <= 50 {
                reset_count += 1;

                match reset {
                    Reset::True => {}
                    Reset::Value(value) => {
                        if let Some(reset_placement) = value.placement {
                            stateful_placement = reset_placement;
                        }

                        if let Some(reset_rects) = value.rects {
                            rects = match reset_rects {
                                ResetRects::True => {
                                    platform.get_element_rects(GetElementRectsArgs {
                                        reference,
                                        floating,
                                        strategy,
                                    })
                                }
                                ResetRects::Value(element_rects) => element_rects,
                            }
                        }

                        let Coords {
                            x: next_x,
                            y: next_y,
                        } = compute_coords_from_placement(&rects, stateful_placement, rtl);
                        x = next_x;
                        y = next_y;
                    }
                }

                i = 0;
                continue;
            }
        }

        i += 1;
    }

    ComputePositionReturn {
        x,
        y,
        placement: stateful_placement,
        strategy,
        middleware_data,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_utils::{FLOATING, PLATFORM, REFERENCE};
    use crate::types::Middleware;

    use super::*;

    #[test]
    fn test_returned_data() {
        struct CustomMiddleware {}

        impl<Element> Middleware<Element> for CustomMiddleware {
            fn name(&self) -> &'static str {
                "custom"
            }

            fn compute(&self, _state: MiddlewareState<Element>) -> MiddlewareReturn {
                MiddlewareReturn {
                    x: None,
                    y: None,
                    data: Some(json!({"property": true})),
                    reset: None,
                }
            }
        }

        let ComputePositionReturn {
            x,
            y,
            placement,
            strategy,
            middleware_data,
        } = compute_position(
            &REFERENCE,
            &FLOATING,
            ComputePositionConfig {
                platform: &PLATFORM,
                placement: Some(Placement::Top),
                strategy: None,
                middleware: Some(vec![&CustomMiddleware {}]),
            },
        );

        assert_eq!(x, 25);
        assert_eq!(y, -50);
        assert_eq!(placement, Placement::Top);
        assert_eq!(strategy, Strategy::Absolute);
        assert_eq!(
            middleware_data.get("custom"),
            Some(&json!({"property": true}))
        );
    }

    #[test]
    fn test_middleware() {
        struct TestMiddleware {}

        impl<Element> Middleware<Element> for TestMiddleware {
            fn name(&self) -> &'static str {
                "test"
            }

            fn compute(
                &self,
                MiddlewareState { x, y, .. }: MiddlewareState<Element>,
            ) -> MiddlewareReturn {
                MiddlewareReturn {
                    x: Some(x + 1),
                    y: Some(y + 1),
                    data: None,
                    reset: None,
                }
            }
        }

        let ComputePositionReturn { x, y, .. } = compute_position(
            &REFERENCE,
            &FLOATING,
            ComputePositionConfig {
                platform: &PLATFORM,
                placement: None,
                strategy: None,
                middleware: None,
            },
        );

        let ComputePositionReturn { x: x2, y: y2, .. } = compute_position(
            &REFERENCE,
            &FLOATING,
            ComputePositionConfig {
                platform: &PLATFORM,
                placement: None,
                strategy: None,
                middleware: Some(vec![&TestMiddleware {}]),
            },
        );

        assert_eq!((x2, y2), (x + 1, y + 1));
    }

    #[test]
    fn test_middleware_data() {
        struct TestMiddleware {}

        impl<Element> Middleware<Element> for TestMiddleware {
            fn name(&self) -> &'static str {
                "test"
            }

            fn compute(&self, _state: MiddlewareState<Element>) -> MiddlewareReturn {
                MiddlewareReturn {
                    x: None,
                    y: None,
                    data: Some(json!({"hello": true})),
                    reset: None,
                }
            }
        }

        let ComputePositionReturn {
            middleware_data, ..
        } = compute_position(
            &REFERENCE,
            &FLOATING,
            ComputePositionConfig {
                platform: &PLATFORM,
                placement: None,
                strategy: None,
                middleware: Some(vec![&TestMiddleware {}]),
            },
        );

        assert_eq!(middleware_data.get("test"), Some(&json!({"hello": true})));
    }
}
