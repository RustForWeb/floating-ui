use std::rc::Rc;

use floating_ui_dom::{auto_update, AutoUpdateOptions};
use yew::prelude::*;

use crate::types::WhileElementsMountedFn;

#[hook]
pub fn use_auto_update() -> Rc<Rc<WhileElementsMountedFn>> {
    use_memo((), |_| {
        let rc: Rc<WhileElementsMountedFn> = Rc::new(|reference, floating, update| {
            auto_update(reference, floating, update, AutoUpdateOptions::default()).into()
        });

        rc
    })
}

#[hook]
pub fn use_auto_update_with_options(options: AutoUpdateOptions) -> Rc<Rc<WhileElementsMountedFn>> {
    use_memo(options, |options| {
        let options = options.clone();

        let rc: Rc<WhileElementsMountedFn> = Rc::new(move |reference, floating, update| {
            auto_update(reference, floating, update, options.clone()).into()
        });

        rc
    })
}
