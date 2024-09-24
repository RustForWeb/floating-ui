use std::{cell::RefCell, rc::Rc};

use floating_ui_dom::{
    compute_position, ComputePositionConfig, MiddlewareData, OwnedElementOrVirtual, Placement,
    Strategy, VirtualElement,
};
use web_sys::wasm_bindgen::JsCast;
use yew::{hook, use_callback, use_effect_with, use_memo, use_mut_ref, use_state_eq, NodeRef};

use crate::{
    types::{
        FloatingStyles, ShallowRc, UseFloatingOptions, UseFloatingReturn,
        WhileElementsMountedCleanupFn,
    },
    utils::{get_dpr::get_dpr, round_by_dpr::round_by_dpr},
};

#[derive(Clone, PartialEq)]
pub enum VirtualElementOrNodeRef {
    VirtualElement(Box<dyn VirtualElement<web_sys::Element>>),
    NodeRef(NodeRef),
}

impl VirtualElementOrNodeRef {
    pub fn get(&self) -> Option<OwnedElementOrVirtual> {
        match self {
            VirtualElementOrNodeRef::VirtualElement(virtual_element) => {
                Some(virtual_element.clone().into())
            }
            VirtualElementOrNodeRef::NodeRef(node_ref) => node_ref.get().map(|node| {
                OwnedElementOrVirtual::Element(
                    node.dyn_into::<web_sys::Element>()
                        .expect("Reference element should be an Element."),
                )
            }),
        }
    }
}

impl From<Box<dyn VirtualElement<web_sys::Element>>> for VirtualElementOrNodeRef {
    fn from(value: Box<dyn VirtualElement<web_sys::Element>>) -> Self {
        VirtualElementOrNodeRef::VirtualElement(value)
    }
}

impl From<NodeRef> for VirtualElementOrNodeRef {
    fn from(value: NodeRef) -> Self {
        VirtualElementOrNodeRef::NodeRef(value)
    }
}

/// Computes the `x` and `y` coordinates that will place the floating element next to a reference element.
#[hook]
pub fn use_floating(
    reference: VirtualElementOrNodeRef,
    floating: NodeRef,
    options: UseFloatingOptions,
) -> UseFloatingReturn {
    let while_elements_mounted_option = options.while_elements_mounted.map(ShallowRc::from);
    let open_option = use_memo(options.open, |open| open.unwrap_or(true));
    let middleware_option = use_memo(options.middleware, |middleware| {
        middleware.clone().unwrap_or_default()
    });
    let placement_option = use_memo(options.placement, |placement| {
        placement.unwrap_or(Placement::Bottom)
    });
    let strategy_option = use_memo(options.strategy, |strategy| {
        strategy.unwrap_or(Strategy::Absolute)
    });
    let transform_option = use_memo(options.transform, |transform| transform.unwrap_or(true));

    let x = use_state_eq(|| 0.0);
    let y = use_state_eq(|| 0.0);
    let strategy = use_state_eq(|| *strategy_option);
    let placement = use_state_eq(|| *placement_option);
    let middleware_data = use_state_eq(MiddlewareData::default);
    let is_positioned = use_state_eq(|| false);
    let floating_styles = use_memo(
        (
            floating.clone(),
            transform_option,
            x.clone(),
            y.clone(),
            strategy.clone(),
        ),
        |(floating, transform_option, x, y, strategy)| {
            let initial_styles = FloatingStyles {
                position: **strategy,
                top: "0".into(),
                left: "0".into(),
                transform: None,
                will_change: None,
            };

            if let Some(floating_element) = floating.get() {
                let x_val = round_by_dpr(&floating_element, **x);
                let y_val = round_by_dpr(&floating_element, **y);

                if **transform_option {
                    FloatingStyles {
                        transform: Some(format!("translate({x_val}px, {y_val}px)")),
                        will_change: match get_dpr(&floating_element) >= 1.5 {
                            true => Some("transform".into()),
                            false => None,
                        },
                        ..initial_styles
                    }
                } else {
                    FloatingStyles {
                        left: format!("{x_val}px"),
                        top: format!("{y_val}px"),
                        ..initial_styles
                    }
                }
            } else {
                initial_styles
            }
        },
    );

    let update = use_callback(
        (
            reference.clone(),
            floating.clone(),
            placement_option.clone(),
            strategy_option.clone(),
            middleware_option.clone(),
            x.clone(),
            y.clone(),
            strategy.clone(),
            placement.clone(),
            middleware_data.clone(),
            is_positioned.clone(),
        ),
        {
            let open_option = open_option.clone();

            move |_,
                  (
                reference,
                floating,
                placement_option,
                strategy_option,
                middleware_option,
                x,
                y,
                strategy,
                placement,
                middleware_data,
                is_positioned,
            )| {
                if let Some(reference_element) = reference.get() {
                    if let Some(floating_element) = floating.get() {
                        let config = ComputePositionConfig {
                            placement: Some(**placement_option),
                            strategy: Some(**strategy_option),
                            middleware: Some((**middleware_option).clone()),
                        };

                        let open = *open_option;

                        let position = compute_position(
                            (&reference_element).into(),
                            floating_element
                                .dyn_ref()
                                .expect("Floating element should be an Element."),
                            config,
                        );
                        x.set(position.x);
                        y.set(position.y);
                        strategy.set(position.strategy);
                        placement.set(position.placement);
                        middleware_data.set(position.middleware_data);
                        // The floating element's position may be recomputed while it's closed
                        // but still mounted (such as when transitioning out). To ensure
                        // `is_positioned` will be `false` initially on the next open,
                        // avoid setting it to `true` when `open === false` (must be specified).
                        is_positioned.set(open);
                    }
                }
            }
        },
    );

    let while_elements_mounted_cleanup: Rc<
        RefCell<Option<ShallowRc<WhileElementsMountedCleanupFn>>>,
    > = use_mut_ref(|| None);

    let cleanup = use_callback(
        while_elements_mounted_cleanup.clone(),
        |_, while_elements_mounted_cleanup| {
            if let Some(while_elements_mounted_cleanup) = while_elements_mounted_cleanup.take() {
                while_elements_mounted_cleanup();
            }
        },
    );

    let attach = use_callback(
        (
            reference.clone(),
            floating.clone(),
            while_elements_mounted_option,
            while_elements_mounted_cleanup,
        ),
        {
            let update = update.clone();
            let cleanup = cleanup.clone();

            move |_: (),
                  (
                reference,
                floating,
                while_elements_mounted_option,
                while_elements_mounted_cleanup,
            )| {
                cleanup.emit(());

                if let Some(while_elements_mounted) = while_elements_mounted_option {
                    if let Some(reference_element) = reference.get() {
                        if let Some(floating_element) = floating.get() {
                            while_elements_mounted_cleanup.replace(Some(ShallowRc::from(
                                (**while_elements_mounted)(
                                    (&reference_element).into(),
                                    floating_element
                                        .dyn_ref()
                                        .expect("Floating element should be an Element."),
                                    Rc::new({
                                        let update = update.clone();

                                        move || {
                                            update.emit(());
                                        }
                                    }),
                                ),
                            )));
                        }
                    }
                } else {
                    update.emit(());
                }
            }
        },
    );

    let reset = use_callback(
        (open_option.clone(), is_positioned.clone()),
        |_, (open_option, is_positioned)| {
            if **open_option {
                is_positioned.set(false);
            }
        },
    );

    use_effect_with(
        (
            open_option.clone(),
            placement_option,
            strategy_option,
            middleware_option,
            update.clone(),
        ),
        |(_, _, _, _, update)| {
            update.emit(());
        },
    );

    use_effect_with((reference, floating, attach), |(_, _, attach)| {
        attach.emit(());
    });

    use_effect_with((open_option, reset), |(_, reset)| {
        reset.emit(());
    });

    use_effect_with((), move |_| {
        move || {
            cleanup.emit(());
        }
    });

    UseFloatingReturn {
        x,
        y,
        placement,
        strategy,
        middleware_data,
        is_positioned,
        floating_styles,
        update,
    }
}
