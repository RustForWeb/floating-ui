use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{js_sys::Reflect, window, Event};
use yew::{hook, use_state, UseStateHandle};

#[hook]
pub fn use_size(initial_size: Option<i32>, key: Option<&'static str>) -> UseStateHandle<i32> {
    let initial_size = initial_size.unwrap_or(80);
    let key = key.unwrap_or("floating");

    let size = use_state(|| initial_size);

    let closure: Closure<dyn Fn(Event)> = Closure::new({
        let size = size.clone();

        move |event: Event| {
            size.set(
                event
                    .target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlInputElement>()
                    .value()
                    .parse()
                    .unwrap(),
            );
        }
    });

    Reflect::set(
        &window().expect("Window should exist."),
        &format!("__handleSizeChange_{}", key).into(),
        &closure.into_js_value(),
    )
    .expect("Reflect set should be successful.");

    size
}
