use std::rc::Rc;

use dioxus::prelude::*;
use floating_ui_dom::{AutoUpdateOptions, auto_update};

use crate::{ShallowRc, types::WhileElementsMountedFn};

/// Use [`auto_update`] with [`AutoUpdateOptions::default`].
///
/// Can be passed to [`UseFloatingOptions::while_elements_mounted`][crate::types::UseFloatingOptions::while_elements_mounted].
pub fn use_auto_update() -> Memo<ShallowRc<WhileElementsMountedFn>> {
    use_memo(|| {
        let rc: Rc<WhileElementsMountedFn> = Rc::new(|reference, floating, update| {
            auto_update(reference, floating, update, AutoUpdateOptions::default())
        });

        rc.into()
    })
}

/// Use [`auto_update`] with `options`.
///
/// Can be passed to [`UseFloatingOptions::while_elements_mounted`][crate::types::UseFloatingOptions::while_elements_mounted].
pub fn use_auto_update_with_options(
    options: ReadOnlySignal<AutoUpdateOptions>,
) -> Memo<ShallowRc<WhileElementsMountedFn>> {
    use_memo(move || {
        let options = options();

        let rc: Rc<WhileElementsMountedFn> = Rc::new(move |reference, floating, update| {
            auto_update(reference, floating, update, options.clone())
        });

        rc.into()
    })
}
