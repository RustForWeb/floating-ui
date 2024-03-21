use floating_ui_utils::{Coords, Placement, Strategy};

use crate::compute_coords_from_placement::compute_coords_from_placement;
use crate::types::{
    ComputePositionConfig, Elements, FloatingElement, GetElementRectsArgs, MiddlewareData,
    MiddlewareReturn, MiddlewareState, ReferenceElement, Reset,
};
use crate::{ComputePositionReturn, ResetRects};

pub fn compute_position(
    reference: ReferenceElement,
    floating: FloatingElement,
    config: ComputePositionConfig,
) -> ComputePositionReturn {
    let placement = config.placement.unwrap_or(Placement::Bottom);
    let strategy = config.strategy.unwrap_or(Strategy::Absolute);
    let platform = config.platform;
    let middlewares = config.middleware.unwrap_or(vec![]);

    let rtl = platform.is_rtl(floating);

    let mut rects = platform.get_element_rects(GetElementRectsArgs {
        reference,
        floating,
        strategy,
    });
    let Coords { mut x, mut y } = compute_coords_from_placement(&rects, placement, rtl);
    let mut stateful_placement = placement;
    let mut middleware_data = MiddlewareData {};
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
            platform: &platform,
            elements: &Elements {
                reference,
                floating,
            },
        });

        x = next_x.unwrap_or(x);
        y = next_y.unwrap_or(y);

        // TODO: modify middleware data

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

                i -= 1;
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
    use floating_ui_utils::{Dimensions, ElementRects, Rect};

    use crate::{
        types::{Element, Platform},
        Middleware,
    };

    use super::*;

    // TODO
    const REFERENCE: Element = false;
    const FLOATING: Element = false;
    const REFERENCE_RECT: Rect = Rect {
        x: 0,
        y: 0,
        width: 100,
        height: 100,
    };
    const FLOATING_RECT: Rect = Rect {
        x: 0,
        y: 0,
        width: 50,
        height: 50,
    };

    struct TestPlatform {}

    impl Platform for TestPlatform {
        fn get_element_rects(&self, _args: GetElementRectsArgs) -> ElementRects {
            ElementRects {
                reference: REFERENCE_RECT,
                floating: FLOATING_RECT,
            }
        }

        fn get_clipping_rect(&self, _args: crate::GetClippingRectArgs) -> Rect {
            todo!()
        }

        fn get_dimensions(&self, _element: Element) -> Dimensions {
            Dimensions {
                width: 10,
                height: 10,
            }
        }
    }

    const PLATFORM: TestPlatform = TestPlatform {};

    #[test]
    fn test_returned_data() {
        let ComputePositionReturn {
            x,
            y,
            placement,
            strategy,
            middleware_data,
        } = compute_position(
            REFERENCE,
            FLOATING,
            ComputePositionConfig {
                platform: Box::new(PLATFORM),
                placement: Some(Placement::Top),
                strategy: None,
                middleware: Some(vec![]),
            },
        );

        assert_eq!(x, 25);
        assert_eq!(y, -50);
        assert_eq!(placement, Placement::Top);
        assert_eq!(strategy, Strategy::Absolute);
        // assert_eq!(middleware_data, MiddlewareData {});
    }

    #[test]
    fn test_middleware() {
        struct TestMiddleware {}

        impl Middleware for TestMiddleware {
            fn name(&self) -> String {
                "test".into()
            }

            fn options(&self) -> bool {
                false
            }

            fn compute(&self, MiddlewareState { x, y, .. }: MiddlewareState) -> MiddlewareReturn {
                MiddlewareReturn {
                    x: Some(x + 1),
                    y: Some(y + 1),
                    data: None,
                    reset: None,
                }
            }
        }

        let ComputePositionReturn { x, y, .. } = compute_position(
            REFERENCE,
            FLOATING,
            ComputePositionConfig {
                platform: Box::new(PLATFORM),
                placement: None,
                strategy: None,
                middleware: None,
            },
        );

        let ComputePositionReturn { x: x2, y: y2, .. } = compute_position(
            REFERENCE,
            FLOATING,
            ComputePositionConfig {
                platform: Box::new(PLATFORM),
                placement: None,
                strategy: None,
                middleware: Some(vec![Box::new(TestMiddleware {})]),
            },
        );

        assert_eq!((x2, y2), (x + 1, y + 1));
    }

    #[test]
    fn test_middleware_data() {}
}
