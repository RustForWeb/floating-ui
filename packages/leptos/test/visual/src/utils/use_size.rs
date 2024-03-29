use leptos::{create_signal, event_target_value, ReadSignal, WriteSignal};
use wasm_bindgen::closure::Closure;
use web_sys::{js_sys::Reflect, window, Event};

pub fn use_size(
    initial_size: Option<i32>,
    key: Option<&'static str>,
) -> (ReadSignal<i32>, WriteSignal<i32>) {
    let initial_size = initial_size.unwrap_or(80);
    let key = key.unwrap_or("floating");

    let (size, set_size) = create_signal(initial_size);

    let closure: Closure<dyn Fn(Event)> = Closure::new(move |event: Event| {
        set_size(event_target_value(&event).parse().unwrap());
    });

    Reflect::set(
        &window().expect("Window should exist."),
        &format!("__handleSizeChange_{}", key).into(),
        &closure.into_js_value(),
    )
    .expect("Reflect set should be successful.");

    (size, set_size)
}
