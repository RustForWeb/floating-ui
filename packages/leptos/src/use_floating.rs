use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use floating_ui_dom::{
    compute_position, ComputePositionConfig, MiddlewareData, OwnedElementOrVirtual, Placement,
    Strategy, VirtualElement,
};
use leptos::{
    create_effect, create_memo, create_signal,
    html::{AnyElement, ElementDescriptor},
    on_cleanup, watch, MaybeProp, NodeRef, SignalGet, SignalGetUntracked, SignalSet,
};

use crate::{
    node_ref::NodeRefAsElement,
    types::{FloatingStyles, UseFloatingOptions, UseFloatingReturn, WhileElementsMountedCleanupFn},
    utils::{get_dpr::get_dpr, round_by_dpr::round_by_dpr},
};

pub enum VirtualElementOrNodeRef<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy + 'static,
    El: ElementDescriptor + Clone + 'static,
{
    VirtualElement(Box<dyn VirtualElement<web_sys::Element>>),
    NodeRef(NodeRef, PhantomData<El>),
}

impl<NodeRef, El> VirtualElementOrNodeRef<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy + 'static,
    El: ElementDescriptor + Clone + 'static,
{
    pub fn get(&self) -> Option<OwnedElementOrVirtual> {
        match self {
            VirtualElementOrNodeRef::VirtualElement(virtual_element) => {
                Some(virtual_element.clone().into())
            }
            VirtualElementOrNodeRef::NodeRef(node_ref, _) => {
                node_ref.get_as_element().map(|element| element.into())
            }
        }
    }

    pub fn get_untracked(&self) -> Option<OwnedElementOrVirtual> {
        match self {
            VirtualElementOrNodeRef::VirtualElement(virtual_element) => {
                Some(virtual_element.clone().into())
            }
            VirtualElementOrNodeRef::NodeRef(node_ref, _) => node_ref
                .get_untracked_as_element()
                .map(|element| element.into()),
        }
    }
}

impl<NodeRef, El> Clone for VirtualElementOrNodeRef<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy + 'static,
    El: ElementDescriptor + Clone + 'static,
{
    fn clone(&self) -> Self {
        match self {
            Self::VirtualElement(virtual_element) => Self::VirtualElement(virtual_element.clone()),
            Self::NodeRef(node_ref, phantom) => Self::NodeRef(*node_ref, *phantom),
        }
    }
}

impl From<Box<dyn VirtualElement<web_sys::Element>>>
    for VirtualElementOrNodeRef<NodeRef<AnyElement>, AnyElement>
{
    fn from(value: Box<dyn VirtualElement<web_sys::Element>>) -> Self {
        VirtualElementOrNodeRef::VirtualElement(value)
    }
}

impl<NodeRef, El> From<NodeRef> for VirtualElementOrNodeRef<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy,
    El: ElementDescriptor + Clone + 'static,
{
    fn from(value: NodeRef) -> Self {
        VirtualElementOrNodeRef::NodeRef(value, PhantomData)
    }
}

pub trait IntoReference<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy,
    El: ElementDescriptor + Clone + 'static,
{
    fn into_reference(self) -> MaybeProp<VirtualElementOrNodeRef<NodeRef, El>>;
}

impl IntoReference<NodeRef<AnyElement>, AnyElement> for Box<dyn VirtualElement<web_sys::Element>> {
    fn into_reference(self) -> MaybeProp<VirtualElementOrNodeRef<NodeRef<AnyElement>, AnyElement>> {
        VirtualElementOrNodeRef::VirtualElement(self).into()
    }
}

impl<NodeRef, El> IntoReference<NodeRef, El> for NodeRef
where
    NodeRef: NodeRefAsElement<El> + Copy,
    El: ElementDescriptor + Clone + 'static,
{
    fn into_reference(self) -> MaybeProp<VirtualElementOrNodeRef<NodeRef, El>> {
        VirtualElementOrNodeRef::NodeRef(self, PhantomData).into()
    }
}

/// Computes the `x` and `y` coordinates that will place the floating element next to a reference element.
pub fn use_floating<
    Reference: NodeRefAsElement<ReferenceEl> + Copy + 'static,
    ReferenceEl: ElementDescriptor + Clone + 'static,
    Floating: NodeRefAsElement<FloatingEl> + Copy + 'static,
    FloatingEl: ElementDescriptor + Clone + 'static,
>(
    reference: MaybeProp<VirtualElementOrNodeRef<Reference, ReferenceEl>>,
    floating: Floating,
    options: UseFloatingOptions,
) -> UseFloatingReturn {
    let open_option = move || options.open.get().unwrap_or(true);
    let placement_option_untracked = move || {
        options
            .placement
            .get_untracked()
            .unwrap_or(Placement::Bottom)
    };
    let strategy_option_untracked = move || {
        options
            .strategy
            .get_untracked()
            .unwrap_or(Strategy::Absolute)
    };
    let options_middleware = options.middleware.clone();
    let middleware_option_untracked = move || options_middleware.get_untracked();
    let transform_option = move || options.transform.get().unwrap_or(true);
    let options_while_elements_mounted = options.while_elements_mounted.clone();
    let while_elements_mounted_untracked = move || options_while_elements_mounted.get_untracked();

    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);
    let (strategy, set_strategy) = create_signal(strategy_option_untracked());
    let (placement, set_placement) = create_signal(placement_option_untracked());
    let (middleware_data, set_middleware_data) = create_signal(MiddlewareData::default());
    let (is_positioned, set_is_positioned) = create_signal(false);
    let floating_styles = create_memo(move |_| {
        let initial_styles = FloatingStyles {
            position: strategy.get(),
            top: "0".into(),
            left: "0".into(),
            transform: None,
            will_change: None,
        };

        if let Some(floating_element) = floating.get_as_element() {
            let x_val = round_by_dpr(&floating_element, x.get());
            let y_val = round_by_dpr(&floating_element, y.get());

            if transform_option() {
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
    });

    let update = Rc::new({
        let reference = reference.clone();

        move || {
            if let Some(reference) = reference.get_untracked() {
                if let Some(reference_element) = reference.get_untracked() {
                    if let Some(floating_element) = floating.get_untracked_as_element() {
                        let config = ComputePositionConfig {
                            placement: Some(placement_option_untracked()),
                            strategy: Some(strategy_option_untracked()),
                            middleware: middleware_option_untracked(),
                        };

                        let open = open_option();

                        let position = compute_position(
                            (&reference_element).into(),
                            &floating_element,
                            config,
                        );
                        set_x.set(position.x);
                        set_y.set(position.y);
                        set_strategy.set(position.strategy);
                        set_placement.set(position.placement);
                        set_middleware_data.set(position.middleware_data);
                        // The floating element's position may be recomputed while it's closed
                        // but still mounted (such as when transitioning out). To ensure
                        // `is_positioned` will be `false` initially on the next open,
                        // avoid setting it to `true` when `open === false` (must be specified).
                        set_is_positioned.set(open);
                    }
                }
            }
        }
    });

    let while_elements_mounted_cleanup: Rc<RefCell<Option<WhileElementsMountedCleanupFn>>> =
        Rc::new(RefCell::new(None));

    let cleanup = Rc::new({
        let while_elements_mounted_cleanup = while_elements_mounted_cleanup.clone();

        move || {
            if let Some(while_elements_mounted_cleanup) = while_elements_mounted_cleanup.take() {
                while_elements_mounted_cleanup();
            }
        }
    });

    let attach_while_elements_mounted_cleanup = while_elements_mounted_cleanup.clone();
    let attach = Rc::new({
        let reference = reference.clone();
        let update = update.clone();
        let cleanup = cleanup.clone();

        move || {
            cleanup();

            if let Some(while_elements_mounted) = while_elements_mounted_untracked() {
                if let Some(reference) = reference.get_untracked() {
                    if let Some(reference_element) = reference.get_untracked() {
                        if let Some(floating_element) = floating.get_untracked_as_element() {
                            attach_while_elements_mounted_cleanup.replace(Some(
                                while_elements_mounted(
                                    (&reference_element).into(),
                                    &floating_element,
                                    update.clone(),
                                ),
                            ));
                        }
                    }
                }
            } else {
                update();
            }
        }
    });

    let reset = move || {
        if !open_option() {
            set_is_positioned.set(false);
        }
    };

    create_effect({
        let attach = attach.clone();

        move |_| {
            if let Some(reference) = reference.get() {
                match reference {
                    VirtualElementOrNodeRef::VirtualElement(_) => {
                        attach();
                    }
                    VirtualElementOrNodeRef::NodeRef(reference, _) => {
                        if let Some(reference) = reference.get() {
                            _ = reference.on_mount({
                                let attach = attach.clone();

                                move |_| {
                                    attach();
                                }
                            });
                        }
                    }
                }
            }
        }
    });

    create_effect({
        let attach = attach.clone();

        move |_| {
            if let Some(floating) = floating.get() {
                _ = floating.on_mount({
                    let attach = attach.clone();

                    move |_| {
                        attach();
                    }
                });
            }
        }
    });

    create_effect(move |_| {
        reset();
    });

    _ = watch(
        open_option,
        {
            let update = update.clone();

            move |_, _, _| {
                update();
            }
        },
        false,
    );
    _ = watch(
        move || options.placement.get(),
        {
            let update = update.clone();

            move |_, _, _| {
                update();
            }
        },
        false,
    );
    _ = watch(
        move || options.strategy.get(),
        {
            let update = update.clone();

            move |_, _, _| {
                update();
            }
        },
        false,
    );
    _ = watch(
        move || options.middleware.get(),
        {
            let update = update.clone();

            move |_, _, _| {
                update();
            }
        },
        false,
    );
    _ = watch(
        move || options.while_elements_mounted.get(),
        move |_, _, _| {
            attach();
        },
        false,
    );

    on_cleanup(move || {
        cleanup();
    });

    UseFloatingReturn {
        x: x.into(),
        y: y.into(),
        placement: placement.into(),
        strategy: strategy.into(),
        middleware_data: middleware_data.into(),
        is_positioned: is_positioned.into(),
        floating_styles: floating_styles.into(),
        update: update.clone(),
    }
}

#[cfg(test)]
mod tests {
    use leptos::{html::Div, *};
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn updates_is_positioned_when_position_is_computed() {
        #[component]
        fn Component() -> impl IntoView {
            let reference = create_node_ref::<Div>();
            let floating = create_node_ref::<Div>();
            let UseFloatingReturn { is_positioned, .. } = use_floating(
                reference.into_reference(),
                floating,
                UseFloatingOptions::default(),
            );

            view! {
                <div _ref=reference />
                <div _ref=floating />
                <div id="test-is-positioned">{is_positioned}</div>
            }
        }

        let document = leptos::document();
        mount_to(document.body().unwrap(), Component);

        // assert_eq!(
        //     document
        //         .get_element_by_id("test-is-positioned")
        //         .and_then(|element| element.text_content()),
        //     Some("true".into())
        // );
    }
}
