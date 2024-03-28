use std::{cell::RefCell, rc::Rc};

use floating_ui_utils::{
    dom::{get_overflow_ancestors, OverflowAncestor},
    OwnedElementOrVirtual,
};
use log::info;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    window, AddEventListenerOptions, Element, EventTarget, ResizeObserver, ResizeObserverEntry,
};

use crate::{types::ElementOrVirtual, utils::get_bounding_client_rect::get_bounding_client_rect};

fn request_animation_frame(callback: &Closure<dyn FnMut()>) -> i32 {
    window()
        .expect("Window should exist.")
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .expect("Request animation frame should be successful.")
}

fn cancel_animation_frame(handle: i32) {
    window()
        .expect("Window should exist.")
        .cancel_animation_frame(handle)
        .expect("Cancel animation frame should be successful.")
}

fn observe_move(_element: &Element, _on_move: Rc<dyn Fn()>) -> Box<dyn Fn()> {
    // TODO

    Box::new(|| {})
}

/// Options for [`auto_update``].
#[derive(Clone, Debug, Default)]
pub struct AutoUpdateOptions {
    /// Whether to update the position when an overflow ancestor is scrolled.
    ///
    /// Defaults to `true`.
    pub ancestor_scroll: Option<bool>,

    /// Whether to update the position when an overflow ancestor is resized. This uses the native `resize` event.
    ///
    /// Defaults to `true`.
    pub ancestor_resize: Option<bool>,

    /// Whether to update the position when either the reference or floating elements resized. This uses a `ResizeObserver`.
    ///
    /// Defaults to `true`.
    pub element_resize: Option<bool>,

    /// Whether to update the position when the reference relocated on the screen due to layout shift.
    ///
    /// Defaults to `true`.
    pub layout_shift: Option<bool>,

    /// Whether to update on every animation frame if necessary.
    /// Only use if you need to update the position in response to an animation using transforms.
    ///
    /// Defaults to `false`.
    pub animation_frame: Option<bool>,
}

impl AutoUpdateOptions {
    /// Set [`Self::ancestor_scroll`] option.
    pub fn ancestor_scroll(mut self, value: bool) -> Self {
        self.ancestor_scroll = Some(value);
        self
    }

    /// Set [`Self::ancestor_resize`] option.
    pub fn ancestor_resize(mut self, value: bool) -> Self {
        self.ancestor_resize = Some(value);
        self
    }

    /// Set [`Self::element_resize`] option.
    pub fn element_resize(mut self, value: bool) -> Self {
        self.element_resize = Some(value);
        self
    }

    /// Set [`Self::layout_shift`] option.
    pub fn layout_shift(mut self, value: bool) -> Self {
        self.layout_shift = Some(value);
        self
    }

    /// Set [`Self::animation_frame`] option.
    pub fn animation_frame(mut self, value: bool) -> Self {
        self.animation_frame = Some(value);
        self
    }
}

/// Automatically updates the position of the floating element when necessary.
/// Should only be called when the floating element is mounted on the DOM or visible on the screen.
pub fn auto_update(
    reference: ElementOrVirtual,
    floating: &Element,
    update: Rc<dyn Fn()>,
    options: AutoUpdateOptions,
) -> Box<dyn Fn()> {
    let ancestor_scoll = options.ancestor_scroll.unwrap_or(true);
    let ancestor_resize = options.ancestor_resize.unwrap_or(true);
    let element_resize = options.element_resize.unwrap_or(true);
    let layout_shift = options.layout_shift.unwrap_or(true);
    let animation_frame = options.animation_frame.unwrap_or(false);

    let reference_element = reference.resolve();

    let ancestors = match ancestor_scoll || ancestor_resize {
        true => {
            let mut ancestors = vec![];

            if let Some(reference) = reference_element {
                ancestors = get_overflow_ancestors(reference, ancestors, true);
            }

            ancestors.append(&mut get_overflow_ancestors(floating, vec![], true));

            ancestors
        }
        false => vec![],
    };

    let update_closure_update = update.clone();
    let update_closure: Closure<dyn Fn()> = Closure::new(move || {
        update_closure_update();
    });

    for ancestor in &ancestors {
        let event_target: &EventTarget = match ancestor {
            OverflowAncestor::Element(element) => element,
            OverflowAncestor::Window(window) => window,
        };

        if ancestor_scoll {
            _ = event_target.add_event_listener_with_callback_and_add_event_listener_options(
                "scroll",
                update_closure.as_ref().unchecked_ref(),
                AddEventListenerOptions::new().passive(true),
            );
        }

        if ancestor_resize {
            _ = event_target.add_event_listener_with_callback(
                "resize",
                update_closure.as_ref().unchecked_ref(),
            );
        }
    }

    let cleanup_observe_move = reference_element.and_then(|reference_element| match layout_shift {
        true => Some(observe_move(reference_element, update.clone())),
        false => None,
    });

    let reobserve_frame = -1;
    let mut resize_observer: Option<ResizeObserver> = None;

    if element_resize {
        let resize_update = update.clone();
        let resize_closure: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
            Closure::new(move |_entries: Vec<ResizeObserverEntry>| {
                // if let Some(first_entry) = entries.first() {
                //     // if reference_element.as_ref().is_some_and(|reference_element| {
                //     //     &&first_entry.target() == reference_element
                //     // }) {
                //         // let _resize_observer =
                //         //     &resize_observer.expect("Resize observer should exist.");

                //         // resize_observer.unobserve(floating);
                //         // TODO: cancel animation frame
                //     }
                // }

                resize_update();
            });

        let local_resize_observer =
            ResizeObserver::new(resize_closure.into_js_value().unchecked_ref())
                .expect("Resize observer should be created.");

        if let Some(reference) = reference_element {
            if !animation_frame {
                local_resize_observer.observe(reference);
            }
        }

        local_resize_observer.observe(floating);

        resize_observer = Some(local_resize_observer);
    }

    let mut frame_id: Option<i32> = None;
    let mut prev_ref_rect = match animation_frame {
        true => Some(get_bounding_client_rect(
            reference.clone(),
            false,
            false,
            None,
        )),
        false => None,
    };

    let frame_loop_closure = Rc::new(RefCell::new(None));
    let frame_loop_closure_clone = frame_loop_closure.clone();

    let owned = match reference {
        ElementOrVirtual::Element(e) => OwnedElementOrVirtual::Element(e.clone()),
        ElementOrVirtual::VirtualElement(ve) => panic!("virtual element"),
        // ElementOrVirtual::VirtualElement(ve) => OwnedElementOrVirtual::VirtualElement(ve.clone()),
    };

    *frame_loop_closure_clone.borrow_mut() = Some(Closure::new(move || {
        info!("animation frame");

        let next_ref_rect = get_bounding_client_rect(
            match &owned {
                OwnedElementOrVirtual::Element(e) => ElementOrVirtual::Element(e),
                OwnedElementOrVirtual::VirtualElement(ve) => {
                    ElementOrVirtual::VirtualElement(&**ve)
                }
            },
            false,
            false,
            None,
        );

        if let Some(prev_ref_rect) = &prev_ref_rect {
            if next_ref_rect.x != prev_ref_rect.x
                || next_ref_rect.y != prev_ref_rect.y
                || next_ref_rect.width != prev_ref_rect.width
                || next_ref_rect.height != prev_ref_rect.height
            {
                update();
            }
        }

        prev_ref_rect = Some(next_ref_rect);
        frame_id = Some(request_animation_frame(
            frame_loop_closure.borrow().as_ref().unwrap(),
        ));
    }));

    if animation_frame {
        request_animation_frame(frame_loop_closure_clone.borrow().as_ref().unwrap());
    }

    Box::new(move || {
        for ancestor in &ancestors {
            let event_target: &EventTarget = match ancestor {
                OverflowAncestor::Element(element) => element,
                OverflowAncestor::Window(window) => window,
            };

            if ancestor_scoll {
                _ = event_target.remove_event_listener_with_callback(
                    "scroll",
                    update_closure.as_ref().unchecked_ref(),
                );
            }

            if ancestor_resize {
                _ = event_target.remove_event_listener_with_callback(
                    "resize",
                    update_closure.as_ref().unchecked_ref(),
                );
            }
        }

        if let Some(cleanup_observe_move) = &cleanup_observe_move {
            cleanup_observe_move();
        }

        if let Some(resize_observer) = &resize_observer {
            resize_observer.disconnect();
        }

        if let Some(frame_id) = frame_id {
            cancel_animation_frame(frame_id);
        }
    })
}
