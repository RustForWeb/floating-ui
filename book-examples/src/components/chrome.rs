// TODO: remove
#![allow(unused)]

use leptos::{html::Div, *};
use tailwind_fuse::tw_merge;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scrollable {
    None,
    X,
    Y,
    Both,
}

#[derive(Clone)]
pub struct ChromeContext(pub NodeRef<Div>);

#[component]
pub fn Chrome(
    #[prop(default = false.into(), into)] center: MaybeSignal<bool>,
    #[prop(default = Scrollable::None.into(), into)] scrollable: MaybeSignal<Scrollable>,
    #[prop(default = true.into(), into)] relative: MaybeSignal<bool>,
    #[prop(into, optional)] label: MaybeProp<String>,
    #[prop(default = 305.into(), into)] scroll_height: MaybeSignal<isize>,
    #[prop(default = true.into(), into)] shadow: MaybeSignal<bool>,
    #[prop(default = false.into(), into)] tall: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    let scrollable_ref: NodeRef<Div> = NodeRef::new();

    let scrollable_x =
        Signal::derive(move || matches!(scrollable.get(), Scrollable::X | Scrollable::Both));
    let scrollable_y =
        Signal::derive(move || matches!(scrollable.get(), Scrollable::Y | Scrollable::Both));
    let is_scrollable = Signal::derive(move || scrollable_x.get() || scrollable_y.get());

    Effect::new(move |_| {
        if let Some(scrollable) = scrollable_ref.get() {
            if scrollable_y.get() {
                scrollable.set_scroll_top(
                    scrollable.scroll_height() / 2 - scrollable.offset_height() / 2,
                );
            }

            if scrollable_x.get() {
                scrollable
                    .set_scroll_left(scrollable.scroll_width() / 2 - scrollable.offset_width() / 2);
            }
        }
    });

    view! {
        <div
            class={move || tw_merge!(
                "overflow-hidden rounded-lg text-gray-900 [color-scheme:light] dark:border-none bg-clip-padding",
                shadow.get().then_some("shadow border border-black/10 dark:border-gray-700")
            )}
        >
            <div class="bg-gray-75 dark:bg-gray-600/60 dark:text-white">
            <div class={{
                let label = label.clone();

                move || tw_merge!("absolute mx-4 flex h-12 items-center gap-2", label.get().map(|_| "sm:flex"))}
            }>
                <div
                    class="h-3 w-3 rounded-full"
                    style:background="#ec695e"
                />
                <div
                    class="h-3 w-3 rounded-full"
                    style:background="#f4bf4f"
                />
                <div
                    class="h-3 w-3 rounded-full"
                    style:background="#61c653"
                />
                </div>
                    <div class="flex h-12 items-center justify-center font-semibold">
                    {move || label.get()}
                </div>
            </div>
            <div class="will-change-transform">
                <div
                    node_ref={scrollable_ref}
                    class={move || tw_merge!(
                        "h-[20rem] overflow-hidden bg-gray-50 p-2",
                        center.get().then_some("grid place-items-center"),
                        scrollable_y.get().then_some("overflow-y-auto"),
                        scrollable_x.get().then_some("overflow-x-auto"),
                        tall.get().then_some("h-[50rem] md:h-[30rem]"),
                        relative.get().then_some("relative")
                    )}
                >
                    <Show when=move || is_scrollable.get()>
                        <div
                            class={scrollable_x.get().then_some("w-[180vw] md:w-[75rem] lg:w-[90rem]")}
                            style:height={match scrollable_y.get() {
                                true => format!("{}px", scroll_height.get()),
                                false => "1px".into(),
                            }}
                        />
                    </Show>
                    <Provider value={ChromeContext(scrollable_ref)}>
                        {children()}
                    </Provider>
                    <Show when=move || is_scrollable.get()>
                        <div
                            class={scrollable_x.get().then_some("w-[180vw] md:w-[75rem] lg:w-[90rem]")}
                            style:height={match scrollable_y.get() {
                                true => format!("{}px", scroll_height.get()),
                                false => "1px".into(),
                            }}
                        />
                    </Show>
                </div>
            </div>
        </div>
    }
}
