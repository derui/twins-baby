use leptos::{IntoView, component, prelude::*, view};
use ui_component::accordion::TreeAccordion;

use crate::leptos_app::ui_state::UiStore;

/// A component for feature island that displays the feature tree.
#[component]
pub fn FeatureIsland() -> impl IntoView {
    let ui_store = use_context::<UiStore>().expect("UiStore should be provided");
    let bodies = ui_store.ui.bodies;

    view! {
        <div class="flex flex-col h-full w-full rounded-lg bg-white/10 backdrop-blur-sm border border-white/20 p-2 overflow-y-auto">
            <h3 class="text-xs font-semibold text-white/70 uppercase tracking-wider mb-2 px-1">
                "Features"
            </h3>
            <For each=move || bodies.get() key=|body| *body.id  let:body>
                <TreeAccordion
                    trigger=move || {
                        let name = body.name.clone();
                        view! {
                            <span class="text-sm text-white/90 font-medium py-1 px-1 hover:text-white transition-colors cursor-pointer truncate">
                                {(*name).clone()}
                            </span>
                        }
                    }
                    initial_open=true
                >
                    <span class="text-xs text-white/50 italic px-1 py-0.5">"No sketches"</span>
                </TreeAccordion>
            </For>
        </div>
    }
}
