mod app;
mod components;
mod utils;

#[cfg(feature = "flip")]
mod flip;
#[cfg(feature = "placement")]
mod placement;
#[cfg(feature = "shift")]
mod shift;
#[cfg(feature = "size")]
mod size;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
