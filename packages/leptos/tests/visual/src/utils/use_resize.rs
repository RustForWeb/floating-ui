use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use leptos::{html::Div, prelude::*};
use send_wrapper::SendWrapper;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{ResizeObserver, ResizeObserverEntry};

pub fn use_resize(node_ref: NodeRef<Div>, update: SendWrapper<Rc<dyn Fn()>>) {
    type CleanupFn = dyn Fn();
    let cleanup: Arc<Mutex<Option<SendWrapper<Box<CleanupFn>>>>> = Arc::new(Mutex::new(None));

    Effect::new({
        let cleanup = cleanup.clone();

        move |_| {
            if let Some(cleanup) = cleanup.lock().expect("Lock should be acquired.").as_ref() {
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

                *cleanup.lock().expect("Lock should be acquired.") =
                    Some(SendWrapper::new(Box::new(move || {
                        observer.unobserve(&element);
                    })));
            }
        }
    });

    on_cleanup(move || {
        if let Some(cleanup) = cleanup.lock().expect("Lock should be acquired.").as_ref() {
            cleanup();
        }
    });
}
