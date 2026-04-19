use leptos::{IntoView, component, prelude::*, view};
use ui_component::accordion::TreeAccordion;

use crate::leptos_app::{
    ui_action::BodyActivatedAction,
    ui_state::UiStore,
    use_action::{UseActionReturn, use_action},
};

/// A component for feature island that displays the feature tree.
#[component]
pub fn FeatureIsland() -> impl IntoView {
    let ui_store = use_context::<UiStore>().expect("UiStore should be provided");
    let bodies = ui_store.ui.bodies;
    let UseActionReturn { dispatch, .. } = use_action();

    view! {
        <div class="flex flex-col h-full w-full rounded-lg bg-white/10 backdrop-blur-sm border border-white/20 p-2 overflow-y-auto">
            <h3 class="text-xs font-semibold text-white/70 uppercase tracking-wider mb-2 px-1">
                "Features"
            </h3>
            <For each=move || bodies.get() key=|body| *body.id let:body>
                <TreeAccordion
                    node={
                        let dispatch = dispatch.clone();
                        move || {
                            let name = body.name.clone();
                            let body_id = *body.id;
                            let active = *body.active;
                            let dispatch = dispatch.clone();
                            let class = if active {
                                "text-sm font-medium py-1 px-2 cursor-pointer truncate rounded border border-white/60 bg-white/90 text-gray-900 transition-colors"
                            } else {
                                "text-sm font-medium py-1 px-2 cursor-pointer truncate rounded border border-transparent text-white/90 hover:text-white transition-colors"
                            };
                            view! {
                                <span
                                    class=class
                                    on:dblclick=move |_| {
                                        dispatch(Box::new(BodyActivatedAction { body_id }))
                                    }
                                >
                                    {(*name).clone()}
                                </span>
                            }
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
