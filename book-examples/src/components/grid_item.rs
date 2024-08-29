use leptos::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn GridItem<F, IV>(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] description: MaybeSignal<String>,
    chrome: F,
    // #[prop(into)] demo_link: MaybeSignal<String>,
    #[prop(default = false.into(), into)] hidden: MaybeSignal<bool>,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView + 'static,
{
    view! {
        <div
            class={move || tw_merge!(
                "relative flex-col justify-between overflow-x-hidden bg-gray-50 px-4 py-8 shadow dark:bg-gray-700 sm:p-8 md:rounded-lg lg:flex",
                hidden.get().then_some("hidden")
            )}
        >
            <div class="overflow-hidden">
                <h3 class="mb-2 text-3xl font-bold">{title}</h3>
                <p class="mb-6 text-xl">{description}</p>
            </div>
            <div class="relative items-center rounded-lg bg-gray-800 shadow-md lg:h-auto">
                {chrome()}
            </div>
            // <a
            //     class="absolute right-6 top-6 inline-flex items-center gap-1 border-none font-bold text-rose-600 underline decoration-rose-500/80 decoration-2 underline-offset-4 transition-colors hover:text-gray-1000 hover:decoration-gray-1000 dark:text-rose-300 dark:decoration-rose-300/80 dark:hover:text-gray-50 dark:hover:decoration-gray-50"
            //     href=demo_link
            //     target="_blank"
            //     rel="noopener noreferrer"
            // >
            //     CodeSandbox
            // </a>
        </div>
    }
}
