use floating_ui_leptos::{
    MiddlewareVec, Placement, Strategy, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use tailwind_fuse::tw_merge;

#[component]
pub fn Floating<CF, CIV, RF, RIV>(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] strategy: MaybeProp<Strategy>,
    #[prop(into, optional)] placement: MaybeProp<Placement>,
    #[prop(into, optional)] middleware: MaybeProp<SendWrapper<MiddlewareVec>>,
    #[prop(default = false.into(), into)] arrow: Signal<bool>,
    content: CF,
    reference: RF,
) -> impl IntoView
where
    CF: Fn() -> CIV + 'static,
    CIV: IntoView + 'static,
    RF: Fn(AnyNodeRef) -> RIV + 'static,
    RIV: IntoView + 'static,
{
    let floating_ref = AnyNodeRef::new();
    let reference_ref = AnyNodeRef::new();
    let arrow_ref = AnyNodeRef::new();

    let UseFloatingReturn {
        floating_styles, ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default()
            .while_elements_mounted_auto_update()
            .placement(placement)
            .strategy(strategy)
            .middleware(middleware),
    );

    view! {
        {reference(reference_ref)}

        <div
            node_ref=floating_ref
            class=move || {
                let class = class.get();

                tw_merge!(
                    "z-10 grid place-items-center bg-rose-500 text-base font-semibold text-gray-50",
                    class
                )
            }
            // TODO: style
            style:position=move || floating_styles.get().style_position()
            style:top=move || floating_styles.get().style_top()
            style:left=move || floating_styles.get().style_left()
            style:transform=move || floating_styles.get().style_transform().unwrap_or_default()
            style:will-change=move || floating_styles.get().style_will_change().unwrap_or_default()
        >
            <div class="px-2 py-2">{content()}</div>
            <Show when=move || arrow.get()>
                <div
                    node_ref=arrow_ref
                    class="h-4 w-4 bg-gray-800 [left:-0.5rem]"
                    // TODO: style
                />
            </Show>
        </div>
    }
}
