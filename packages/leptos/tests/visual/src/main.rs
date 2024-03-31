mod app;
mod spec;
mod utils;

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to(
        leptos::document()
            .get_element_by_id("root")
            .unwrap()
            .unchecked_into::<HtmlElement>(),
        App,
    );
}
