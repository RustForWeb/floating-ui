use leptos::{html::AnyElement, *};
use tailwind_fuse::tw_merge;

#[component]
pub fn Reference(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
    view! {
        <button
            class={move || {
                let class = class.get();
                tw_merge!(
                    "z-50 h-24 w-24 cursor-default border-2 border-dashed border-gray-900 bg-gray-50 p-2 text-sm font-bold text-gray-900",
                    class
                )
            }}
            aria-label="Reference element"
        >
            Reference
        </button>
    }
    .into_any()
    .node_ref(node_ref)
}
