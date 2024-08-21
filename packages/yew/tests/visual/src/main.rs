mod app;
mod spec;
mod utils;

use web_sys::window;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    yew::Renderer::<App>::with_root(
        window()
            .expect("Window should exist.")
            .document()
            .expect("Document should exist.")
            .get_element_by_id("root")
            .expect("Element should exist."),
    )
    .render();
}
