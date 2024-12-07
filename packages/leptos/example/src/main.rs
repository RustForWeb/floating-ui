mod app;

use leptos::prelude::*;

use crate::app::App;

pub fn main() {
    console_log::init_with_level(log::Level::Debug).expect("Console logger should be available");
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
