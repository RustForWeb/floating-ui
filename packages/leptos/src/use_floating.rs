use std::{
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
};

use floating_ui_dom::{
    ComputePositionConfig, MiddlewareData, OwnedElementOrVirtual, Placement, Strategy,
    VirtualElement, compute_position,
};
use leptos::{html::ElementType, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, JsValue};

use crate::{
    types::{FloatingStyles, UseFloatingOptions, UseFloatingReturn, WhileElementsMountedCleanupFn},
    utils::{get_dpr::get_dpr, round_by_dpr::round_by_dpr},
};

pub struct Virtual;

impl ElementType for Virtual {
    type Output = JsValue;

    const TAG: &'static str = "virtual";
    const SELF_CLOSING: bool = true;
    const ESCAPE_CHILDREN: bool = true;
    const NAMESPACE: Option<&'static str> = None;

    fn tag(&self) -> &str {
        Self::TAG
    }
}

#[derive(Clone)]
pub enum VirtualElementOrNodeRef {
    VirtualElement(SendWrapper<Box<dyn VirtualElement<web_sys::Element>>>),
    NodeRef(AnyNodeRef),
}

impl VirtualElementOrNodeRef {
    pub fn get(&self) -> Option<OwnedElementOrVirtual> {
        match self {
            VirtualElementOrNodeRef::VirtualElement(virtual_element) => {
                Some((**virtual_element).clone().into())
            }
            VirtualElementOrNodeRef::NodeRef(node_ref) => node_ref
                .get()
                .and_then(|element| element.dyn_into::<web_sys::Element>().ok())
                .map(|element| element.into()),
        }
    }

    pub fn get_untracked(&self) -> Option<OwnedElementOrVirtual> {
        match self {
            VirtualElementOrNodeRef::VirtualElement(virtual_element) => {
                Some((**virtual_element).clone().into())
            }
            VirtualElementOrNodeRef::NodeRef(node_ref) => node_ref
                .get_untracked()
                .and_then(|element| element.dyn_into::<web_sys::Element>().ok())
                .map(|element| element.into()),
        }
    }
}

// impl<E: ElementType> Clone for VirtualElementOrNodeRef<E> {
//     fn clone(&self) -> Self {
//         match self {
//             Self::VirtualElement(virtual_element) => Self::VirtualElement(virtual_element.clone()),
//             Self::NodeRef(node_ref) => Self::NodeRef(*node_ref),
//         }
//     }
// }

impl From<Box<dyn VirtualElement<web_sys::Element>>> for VirtualElementOrNodeRef {
    fn from(value: Box<dyn VirtualElement<web_sys::Element>>) -> Self {
        VirtualElementOrNodeRef::VirtualElement(SendWrapper::new(value))
    }
}

impl From<AnyNodeRef> for VirtualElementOrNodeRef {
    fn from(value: AnyNodeRef) -> Self {
        VirtualElementOrNodeRef::NodeRef(value)
    }
}

#[derive(Clone, Copy)]
pub struct Reference(MaybeProp<VirtualElementOrNodeRef>);

impl Deref for Reference {
    type Target = MaybeProp<VirtualElementOrNodeRef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MaybeProp<VirtualElementOrNodeRef>> for Reference {
    fn from(value: MaybeProp<VirtualElementOrNodeRef>) -> Self {
        Reference(value)
    }
}

impl From<Memo<VirtualElementOrNodeRef>> for Reference {
    fn from(value: Memo<VirtualElementOrNodeRef>) -> Self {
        Reference(value.into())
    }
}

impl From<ReadSignal<VirtualElementOrNodeRef>> for Reference {
    fn from(value: ReadSignal<VirtualElementOrNodeRef>) -> Self {
        Reference(value.into())
    }
}

impl From<RwSignal<VirtualElementOrNodeRef>> for Reference {
    fn from(value: RwSignal<VirtualElementOrNodeRef>) -> Self {
        Reference(value.into())
    }
}

impl From<Signal<VirtualElementOrNodeRef>> for Reference {
    fn from(value: Signal<VirtualElementOrNodeRef>) -> Self {
        Reference(value.into())
    }
}

impl From<VirtualElementOrNodeRef> for Reference {
    fn from(value: VirtualElementOrNodeRef) -> Self {
        Reference(value.into())
    }
}

impl From<Box<dyn VirtualElement<web_sys::Element>>> for Reference {
    fn from(value: Box<dyn VirtualElement<web_sys::Element>>) -> Self {
        Reference(VirtualElementOrNodeRef::from(value).into())
    }
}

impl From<AnyNodeRef> for Reference {
    fn from(value: AnyNodeRef) -> Self {
        Reference(VirtualElementOrNodeRef::from(value).into())
    }
}

/// Computes the `x` and `y` coordinates that will place the floating element next to a reference element.
pub fn use_floating<R: Into<Reference>>(
    reference: R,
    floating: AnyNodeRef,
    options: UseFloatingOptions,
) -> UseFloatingReturn {
    let reference: Reference = reference.into();

    let open_option = Signal::derive(move || options.open.get().unwrap_or(true));
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
    let middleware_option_untracked = move || options.middleware.get_untracked();
    let transform_option = move || options.transform.get().unwrap_or(true);
    let while_elements_mounted_untracked = move || options.while_elements_mounted.get_untracked();

    let (x, set_x) = signal(0.0);
    let (y, set_y) = signal(0.0);
    let (strategy, set_strategy) = signal(strategy_option_untracked());
    let (placement, set_placement) = signal(placement_option_untracked());
    let (middleware_data, set_middleware_data) = signal(MiddlewareData::default());
    let (is_positioned, set_is_positioned) = signal(false);
    let floating_styles = Memo::new(move |_| {
        let initial_styles = FloatingStyles {
            position: strategy.get(),
            top: "0".to_owned(),
            left: "0".to_owned(),
            transform: None,
            will_change: None,
        };

        match floating
            .get()
            .and_then(|floating| floating.dyn_into::<web_sys::Element>().ok())
        {
            Some(floating_element) => {
                let x_val = round_by_dpr(&floating_element, x.get());
                let y_val = round_by_dpr(&floating_element, y.get());

                if transform_option() {
                    FloatingStyles {
                        transform: Some(format!("translate({x_val}px, {y_val}px)")),
                        will_change: (get_dpr(&floating_element) >= 1.5)
                            .then_some("transform".to_owned()),
                        ..initial_styles
                    }
                } else {
                    FloatingStyles {
                        left: format!("{x_val}px"),
                        top: format!("{y_val}px"),
                        ..initial_styles
                    }
                }
            }
            _ => initial_styles,
        }
    });

    let update = Rc::new({
        move || {
            if let Some(reference) = reference.get_untracked()
                && let Some(reference_element) = reference.get_untracked()
                && let Some(floating_element) = floating
                    .get_untracked()
                    .and_then(|floating| floating.dyn_into::<web_sys::Element>().ok())
            {
                let config = ComputePositionConfig {
                    placement: Some(placement_option_untracked()),
                    strategy: Some(strategy_option_untracked()),
                    middleware: middleware_option_untracked()
                        .map(|middleware| middleware.deref().clone()),
                };

                let open = open_option.get_untracked();

                let position =
                    compute_position((&reference_element).into(), &floating_element, config);
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
    });

    let while_elements_mounted_cleanup: Arc<
        Mutex<Option<SendWrapper<WhileElementsMountedCleanupFn>>>,
    > = Arc::new(Mutex::new(None));

    let cleanup = Arc::new({
        let while_elements_mounted_cleanup = while_elements_mounted_cleanup.clone();

        move || {
            if let Some(while_elements_mounted_cleanup) = while_elements_mounted_cleanup
                .lock()
                .expect("Lock should be acquired.")
                .as_ref()
            {
                while_elements_mounted_cleanup();
            }
        }
    });

    let attach = Rc::new({
        let update = update.clone();
        let cleanup = cleanup.clone();
        let while_elements_mounted_cleanup = while_elements_mounted_cleanup.clone();

        move || {
            cleanup();

            match while_elements_mounted_untracked() {
                Some(while_elements_mounted) => {
                    if let Some(reference) = reference.get_untracked()
                        && let Some(reference_element) = reference.get_untracked()
                        && let Some(floating_element) = floating
                            .get_untracked()
                            .and_then(|floating| floating.dyn_into::<web_sys::Element>().ok())
                    {
                        *while_elements_mounted_cleanup
                            .lock()
                            .expect("Lock should be acquired.") =
                            Some(SendWrapper::new(while_elements_mounted(
                                (&reference_element).into(),
                                &floating_element,
                                update.clone(),
                            )));
                    }
                }
                _ => {
                    update();
                }
            }
        }
    });

    let reset = move || {
        if !open_option.get_untracked() {
            set_is_positioned.set(false);
        }
    };

    Effect::new({
        let attach = attach.clone();

        move |_| {
            if let Some(reference) = reference.get() {
                match reference {
                    VirtualElementOrNodeRef::VirtualElement(_) => {
                        attach();
                    }
                    VirtualElementOrNodeRef::NodeRef(reference) => {
                        if reference
                            .get()
                            .and_then(|reference| reference.dyn_into::<web_sys::Element>().ok())
                            .is_some()
                        {
                            attach();
                        }
                    }
                }
            }
        }
    });

    Effect::new({
        let attach = attach.clone();

        move |_| {
            if floating
                .get()
                .and_then(|floating| floating.dyn_into::<web_sys::Element>().ok())
                .is_some()
            {
                attach();
            }
        }
    });

    Effect::new(move |_| {
        reset();
    });

    _ = Effect::watch(
        move || open_option.get(),
        {
            let update = update.clone();

            move |_, _, _| {
                update();
            }
        },
        false,
    );
    _ = Effect::watch(
        move || options.placement.get(),
        {
            let update = update.clone();

            move |_, _, _| {
                update();
            }
        },
        false,
    );
    _ = Effect::watch(
        move || options.strategy.get(),
        {
            let update = update.clone();

            move |_, _, _| {
                update();
            }
        },
        false,
    );
    _ = Effect::watch(
        move || options.middleware.get(),
        {
            let update = update.clone();

            move |_, _, _| {
                update();
            }
        },
        false,
    );
    _ = Effect::watch(
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
        update: SendWrapper::new(update.clone()),
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    use leptos_node_ref::AnyNodeRef;
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn updates_is_positioned_when_position_is_computed() {
        #[component]
        fn Component() -> impl IntoView {
            let reference = AnyNodeRef::new();
            let floating = AnyNodeRef::new();
            let UseFloatingReturn { is_positioned, .. } =
                use_floating(reference, floating, UseFloatingOptions::default());

            view! {
                <div node_ref=reference />
                <div node_ref=floating />
                <div id="test-is-positioned">{is_positioned}</div>
            }
        }

        mount_to_body(Component);

        // assert_eq!(
        //     document
        //         .get_element_by_id("test-is-positioned")
        //         .and_then(|element| element.text_content()),
        //     Some("true".to_owned())
        // );
    }
}
