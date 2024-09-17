use std::{cell::RefCell, rc::Rc};

use leptos::{html::Div, *};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{ResizeObserver, ResizeObserverEntry};

pub fn use_resize(node_ref: NodeRef<Div>, update: Rc<dyn Fn()>) {
    type CleanupFn = dyn Fn();
    let cleanup: Rc<RefCell<Option<Box<CleanupFn>>>> = Rc::new(RefCell::new(None));

    Effect::new({
        let cleanup = cleanup.clone();

        move |_| {
            if let Some(cleanup) = cleanup.take() {
                cleanup();
            }

            if let Some(element) = node_ref.get() {
                let resize_closure: Closure<dyn Fn(Vec<ResizeObserverEntry>)> = Closure::new({
                    let update = update.clone();

                    move |_entries: Vec<ResizeObserverEntry>| {
                        update();
                    }
                });

                let observer = ResizeObserver::new(resize_closure.into_js_value().unchecked_ref())
                    .expect("Resize observer should be created.");

                observer.observe(&element);

                cleanup.replace(Some(Box::new(move || {
                    observer.unobserve(&element);
                })));
            }
        }
    });

    on_cleanup(move || {
        if let Some(cleanup) = cleanup.take() {
            cleanup();
        }
    });
}
