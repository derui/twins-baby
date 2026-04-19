use cad_base::id::BodyId;
use leptos::{IntoView, component, prelude::*, view};
use reactive_stores::Store;
use ui_component::accordion::TreeAccordion;

use crate::leptos_app::{
    app_state::AppStore,
    ui_action::BodyActivatedAction,
    ui_state::{BodyUI, UiStore},
    use_action::{UseActionReturn, use_action},
};

/// A body row of featur
#[component]
fn BodyFeature(id: BodyId) -> impl IntoView {
    let app_store = use_context::<Store<AppStore>>().expect("AppStore should be provided");

    let UseActionReturn { dispatch, .. } = use_action();

    view! {
        <TreeAccordion
            node=move || {
                let body = BodyUI::from_store(app_store, id);
                let dispatch = dispatch.clone();
                let class = move || {
                    if body.active.get() {
                        "text-sm font-medium py-1 px-2 cursor-pointer truncate rounded border border-white/60 bg-white/90 text-gray-900 transition-colors"
                    } else {
                        "text-sm font-medium py-1 px-2 cursor-pointer truncate rounded border border-transparent text-white/90 hover:text-white transition-colors"
                    }
                };
                view! {
                    <span
                        class=class
                        on:click=move |e| {
                            e.stop_propagation();
                            e.prevent_default();
                            dispatch(
                                Box::new(BodyActivatedAction {
                                    body_id: body.id.get(),
                                }),
                            )
                        }
                    >
                        {body.name.get()}
                    </span>
                }
            }
            initial_open=true
        >
            <span class="text-xs text-white/50 italic px-1 py-0.5">"No sketches"</span>
        </TreeAccordion>
    }
}

/// A component for feature island that displays the feature tree.
#[component]
pub fn FeatureIsland() -> impl IntoView {
    let ui_store = use_context::<UiStore>().expect("UiStore should be provided");

    view! {
        <div class="flex flex-col h-full w-full rounded-lg bg-white/10 backdrop-blur-sm border border-white/20 p-2 overflow-y-auto">
            <h3 class="text-xs font-semibold text-white/70 uppercase tracking-wider mb-2 px-1">
                "Features"
            </h3>

            <For
                each=move || ui_store.ui.bodies.get()
                key=|id| *id
                children=move |id| view! { <BodyFeature id=id /> }
            ></For>
        </div>
    }
}
