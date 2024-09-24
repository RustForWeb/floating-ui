use std::{cell::RefCell, rc::Rc};

use floating_ui_utils::{
    dom::{get_document_element, get_overflow_ancestors, get_window, OverflowAncestor},
    ClientRectObject,
};
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast, JsValue},
    window, AddEventListenerOptions, Element, EventTarget, IntersectionObserver,
    IntersectionObserverEntry, IntersectionObserverInit, ResizeObserver, ResizeObserverEntry,
};

use crate::{
    types::{ElementOrVirtual, OwnedElementOrVirtual},
    utils::get_bounding_client_rect::get_bounding_client_rect,
};

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

fn observe_move(element: Element, on_move: Rc<dyn Fn()>) -> Box<dyn Fn()> {
    let io: Rc<RefCell<Option<IntersectionObserver>>> = Rc::new(RefCell::new(None));
    let timeout_id: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));

    let window = get_window(Some(&element));
    let root = get_document_element(Some((&element).into()));

    type ObserveClosure = Closure<dyn Fn(Vec<IntersectionObserverEntry>)>;
    let observe_closure: Rc<RefCell<Option<ObserveClosure>>> = Rc::new(RefCell::new(None));

    let cleanup_io = io.clone();
    let cleanup_timeout_id = timeout_id.clone();
    let cleanup_window = window.clone();
    let cleanup_observe_closure = observe_closure.clone();
    let cleanup = move || {
        if let Some(timeout_id) = cleanup_timeout_id.take() {
            cleanup_window.clear_timeout_with_handle(timeout_id);
        }

        if let Some(io) = cleanup_io.take() {
            io.disconnect();
        }

        _ = cleanup_observe_closure.take();
    };
    let cleanup_rc = Rc::new(cleanup);
    type RefreshFn = Box<dyn Fn(bool, f64)>;
    let refresh_closure: Rc<RefCell<Option<RefreshFn>>> = Rc::new(RefCell::new(None));
    let refresh_closure_clone = refresh_closure.clone();

    let refresh_cleanup = cleanup_rc.clone();
    *refresh_closure_clone.borrow_mut() = Some(Box::new(move |skip: bool, threshold: f64| {
        refresh_cleanup();

        let rect = element.get_bounding_client_rect();

        if !skip {
            on_move();
        }

        if rect.width() == 0.0 || rect.height() == 0.0 {
            return;
        }

        let inset_top = rect.top().floor();
        let inset_right = (root.client_width() as f64 - (rect.left() + rect.width())).floor();
        let inset_bottom = (root.client_height() as f64 - (rect.top() + rect.height())).floor();
        let inset_left = rect.left().floor();
        let root_margin = format!(
            "{}px {}px {}px {}px",
            -inset_top, -inset_right, -inset_bottom, -inset_left
        );

        let is_first_update: Rc<RefCell<bool>> = Rc::new(RefCell::new(true));

        let timeout_refresh = refresh_closure.clone();
        let timeout_closure: Rc<Closure<dyn Fn()>> = Rc::new(Closure::new(move || {
            timeout_refresh
                .borrow()
                .as_ref()
                .expect("Refresh closure should exist.")(false, 1e-7)
        }));

        let observe_timeout_id = timeout_id.clone();
        let observe_window = window.clone();
        let observe_refresh = refresh_closure.clone();
        let local_observe_closure = Closure::new(move |entries: Vec<IntersectionObserverEntry>| {
            let ratio = entries[0].intersection_ratio();

            if ratio != threshold {
                if !*is_first_update.borrow() {
                    observe_refresh
                        .borrow()
                        .as_ref()
                        .expect("Refresh closure should exist.")(false, 1.0);
                    return;
                }

                if ratio == 0.0 {
                    // If the reference is clipped, the ratio is 0. Throttle the refresh to prevent an infinite loop of updates.
                    observe_timeout_id.replace(Some(
                        observe_window
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                (*timeout_closure).as_ref().unchecked_ref(),
                                1000,
                            )
                            .expect("Set timeout should be successful."),
                    ));
                } else {
                    observe_refresh
                        .borrow()
                        .as_ref()
                        .expect("Refresh closure should exist.")(false, ratio);
                }

                is_first_update.replace(false);
            }
        });

        let options = IntersectionObserverInit::new();
        options.set_root_margin(&root_margin);
        options.set_threshold(&JsValue::from_f64(threshold.clamp(0.0, 1.0)));

        let local_io = IntersectionObserver::new_with_options(
            local_observe_closure.as_ref().unchecked_ref(),
            &options,
        )
        .expect("Intersection observer should be created.");

        observe_closure.replace(Some(local_observe_closure));

        local_io.observe(&element);
        io.replace(Some(local_io));
    }));

    refresh_closure_clone
        .borrow()
        .as_ref()
        .expect("Refresh closure should exist.")(true, 1.0);

    Box::new(move || {
        cleanup_rc();
    })
}

/// Options for [`auto_update`].
#[derive(Clone, Debug, Default, PartialEq)]
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
    /// Set `ancestor_scroll` option.
    pub fn ancestor_scroll(mut self, value: bool) -> Self {
        self.ancestor_scroll = Some(value);
        self
    }

    /// Set `ancestor_resize` option.
    pub fn ancestor_resize(mut self, value: bool) -> Self {
        self.ancestor_resize = Some(value);
        self
    }

    /// Set `element_resize` option.
    pub fn element_resize(mut self, value: bool) -> Self {
        self.element_resize = Some(value);
        self
    }

    /// Set `layout_shift` option.
    pub fn layout_shift(mut self, value: bool) -> Self {
        self.layout_shift = Some(value);
        self
    }

    /// Set `animation_frame` option.
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

    let reference_element = reference.clone().resolve();

    let owned_reference = match reference.clone() {
        ElementOrVirtual::Element(e) => OwnedElementOrVirtual::Element(e.clone()),
        ElementOrVirtual::VirtualElement(ve) => OwnedElementOrVirtual::VirtualElement(ve.clone()),
    };

    let ancestors = match ancestor_scoll || ancestor_resize {
        true => {
            let mut ancestors = vec![];

            if let Some(reference) = reference_element.as_ref() {
                ancestors = get_overflow_ancestors(reference, ancestors, true);
            }

            ancestors.append(&mut get_overflow_ancestors(floating, vec![], true));

            ancestors
        }
        false => vec![],
    };

    let update_closure: Closure<dyn Fn()> = Closure::new({
        let update = update.clone();

        move || {
            update();
        }
    });

    for ancestor in &ancestors {
        let event_target: &EventTarget = match ancestor {
            OverflowAncestor::Element(element) => element,
            OverflowAncestor::Window(window) => window,
        };

        if ancestor_scoll {
            let options = AddEventListenerOptions::new();
            options.set_passive(true);

            event_target
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "scroll",
                    update_closure.as_ref().unchecked_ref(),
                    &options,
                )
                .expect("Scroll event listener should be added.");
        }

        if ancestor_resize {
            event_target
                .add_event_listener_with_callback("resize", update_closure.as_ref().unchecked_ref())
                .expect("Resize event listener should be added.");
        }
    }

    let cleanup_observe_move =
        reference_element
            .as_ref()
            .and_then(|reference_element| match layout_shift {
                true => Some(observe_move(reference_element.clone(), update.clone())),
                false => None,
            });

    let reobserve_frame: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
    let resize_observer: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));

    if element_resize {
        let reobserve_floating = floating.clone();
        let reobserve_closure: Rc<Closure<dyn FnMut()>> = Rc::new(Closure::new({
            let resize_observer = resize_observer.clone();

            move || {
                resize_observer
                    .borrow()
                    .as_ref()
                    .expect("Resize observer should exist.")
                    .observe(&reobserve_floating);
            }
        }));

        let resize_reference_element = reference_element.clone();
        let resize_closure: Closure<dyn Fn(Vec<ResizeObserverEntry>)> = Closure::new({
            let reobserve_frame = reobserve_frame.clone();
            let update = update.clone();

            move |entries: Vec<ResizeObserverEntry>| {
                if let Some(first_entry) = entries.first() {
                    if resize_reference_element
                        .as_ref()
                        .is_some_and(|reference_element| first_entry.target() == *reference_element)
                    {
                        if let Some(reobserve_frame) = reobserve_frame.take() {
                            cancel_animation_frame(reobserve_frame);
                        }

                        reobserve_frame
                            .replace(Some(request_animation_frame(reobserve_closure.as_ref())));
                    }
                }

                update();
            }
        });

        resize_observer.replace(Some(
            ResizeObserver::new(resize_closure.into_js_value().unchecked_ref())
                .expect("Resize observer should be created."),
        ));

        if let Some(reference) = reference_element.as_ref() {
            if !animation_frame {
                resize_observer
                    .borrow()
                    .as_ref()
                    .expect("Resize observer should exist.")
                    .observe(reference);
            }
        }

        resize_observer
            .borrow()
            .as_ref()
            .expect("Resize observer should exist.")
            .observe(floating);
    }

    let frame_id: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
    let prev_ref_rect: Rc<RefCell<Option<ClientRectObject>>> =
        Rc::new(RefCell::new(match animation_frame {
            true => Some(get_bounding_client_rect(reference, false, false, None)),
            false => None,
        }));

    let frame_loop_frame_id = frame_id.clone();
    let frame_loop_closure = Rc::new(RefCell::new(None));
    let frame_loop_closure_clone = frame_loop_closure.clone();

    *frame_loop_closure_clone.borrow_mut() = Some(Closure::new({
        let owned_reference = owned_reference.clone();
        let update = update.clone();
        let prev_ref_rect = prev_ref_rect.clone();
        let frame_loop_frame_id = frame_loop_frame_id.clone();

        move || {
            let next_ref_rect =
                get_bounding_client_rect((&owned_reference).into(), false, false, None);

            if let Some(prev_ref_rect) = prev_ref_rect.borrow().as_ref() {
                if next_ref_rect.x != prev_ref_rect.x
                    || next_ref_rect.y != prev_ref_rect.y
                    || next_ref_rect.width != prev_ref_rect.width
                    || next_ref_rect.height != prev_ref_rect.height
                {
                    update();
                }
            }

            prev_ref_rect.replace(Some(next_ref_rect));
            frame_loop_frame_id.replace(Some(request_animation_frame(
                frame_loop_closure
                    .borrow()
                    .as_ref()
                    .expect("Frame loop closure should exist."),
            )));
        }
    }));

    if animation_frame {
        // Frame loop closure can't be called here, so the code below is copied.

        let next_ref_rect = get_bounding_client_rect((&owned_reference).into(), false, false, None);

        if let Some(prev_ref_rect) = prev_ref_rect.borrow().as_ref() {
            if next_ref_rect.x != prev_ref_rect.x
                || next_ref_rect.y != prev_ref_rect.y
                || next_ref_rect.width != prev_ref_rect.width
                || next_ref_rect.height != prev_ref_rect.height
            {
                update();
            }
        }

        prev_ref_rect.replace(Some(next_ref_rect));
        frame_loop_frame_id.replace(Some(request_animation_frame(
            frame_loop_closure_clone
                .borrow()
                .as_ref()
                .expect("Frame loop closure should exist."),
        )));
    }

    update();

    Box::new(move || {
        for ancestor in &ancestors {
            let event_target: &EventTarget = match ancestor {
                OverflowAncestor::Element(element) => element,
                OverflowAncestor::Window(window) => window,
            };

            if ancestor_scoll {
                event_target
                    .remove_event_listener_with_callback(
                        "scroll",
                        update_closure.as_ref().unchecked_ref(),
                    )
                    .expect("Scroll event listener should be removed.");
            }

            if ancestor_resize {
                event_target
                    .remove_event_listener_with_callback(
                        "resize",
                        update_closure.as_ref().unchecked_ref(),
                    )
                    .expect("Resize event listener should be removed.");
            }
        }

        if let Some(cleanup_observe_move) = &cleanup_observe_move {
            cleanup_observe_move();
        }

        if let Some(reobserve_frame) = reobserve_frame.take() {
            cancel_animation_frame(reobserve_frame);
        }

        if let Some(resize_observer) = resize_observer.take() {
            resize_observer.disconnect();
        }

        if let Some(frame_id) = frame_id.take() {
            cancel_animation_frame(frame_id);
        }
    })
}
