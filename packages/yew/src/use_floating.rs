use std::rc::Rc;

use floating_ui_dom::{
    compute_position, ComputePositionConfig, MiddlewareData, OwnedElementOrVirtual, Placement,
    Strategy, VirtualElement,
};
use web_sys::wasm_bindgen::JsCast;
use yew::{hook, use_callback, use_effect_with, use_memo, use_state, Callback, NodeRef};

use crate::{
    types::{FloatingStyles, UseFloatingOptions, UseFloatingReturn},
    utils::{get_dpr::get_dpr, round_by_dpr::round_by_dpr},
};

#[derive(Clone)]
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

impl PartialEq for VirtualElementOrNodeRef {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Virtual element does not implement PartialEq, so always return false.
            // TODO: implement PartialEq for virtual elements.
            (Self::VirtualElement(a), Self::VirtualElement(b)) => false,
            (Self::NodeRef(a), Self::NodeRef(b)) => a == b,
            _ => false,
        }
    }
}

struct ShallowRc<T: ?Sized>(Rc<T>);

impl<T: ?Sized> PartialEq for ShallowRc<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// Computes the `x` and `y` coordinates that will place the floating element next to a reference element.
#[hook]
pub fn use_floating(
    reference: VirtualElementOrNodeRef,
    floating: NodeRef,
    options: UseFloatingOptions,
) -> UseFloatingReturn {
    use std::rc::Rc;

    let while_elements_mounted_option = options.while_elements_mounted.map(ShallowRc);
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

    let x = use_state(|| 0.0);
    let y = use_state(|| 0.0);
    let strategy = use_state(|| *strategy_option);
    let placement = use_state(|| *placement_option);
    let middleware_data = use_state(MiddlewareData::default);
    let is_positioned = use_state(|| false);
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

                    let position = compute_position(
                        (&reference_element).into(),
                        floating_element
                            .dyn_ref()
                            .expect("Floating element should be an Element."),
                        Some(config),
                    );
                    x.set(position.x);
                    y.set(position.y);
                    strategy.set(position.strategy);
                    placement.set(position.placement);
                    middleware_data.set(position.middleware_data);
                    is_positioned.set(true);
                }
            }
        },
    );

    let cleanup = use_callback((), |_, ()| {});

    let attach = use_callback(
        (while_elements_mounted_option, update.clone(), cleanup),
        |_, (while_elements_mounted_option, update, cleanup)| {
            cleanup.emit(());

            if let Some(while_elements_mounted) = while_elements_mounted_option {
            } else {
                update.emit(());
            }
        },
    );

    let reset = use_callback((), |_, ()| {});

    use_effect_with(
        (
            placement_option,
            strategy_option,
            // middleware_option,
            update.clone(),
        ),
        |(_, _, update)| {
            update.emit(());
        },
    );

    use_effect_with((reference, floating, attach), |(_, _, attach)| {
        attach.emit(());
    });

    use_effect_with((open_option, reset), |(_, reset)| {
        reset.emit(());
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
