use cad_base::id::BodyId;
use leptos::{IntoView, component, prelude::*, view};
use reactive_stores::Store;
use ui_component::{
    accordion::TreeAccordion,
    icon::{Icon, IconSize, IconType},
};

use crate::leptos_app::{
    app_state::{AppStore, AppStoreStoreFields as _},
    ui_action::BodyActivatedAction,
    ui_state::{BodiesUI, BodyUI, FeatureTreeUI, FeatureUIType, SketchUI},
    use_action::{UseActionReturn, use_action},
};

/// A sketch item row in the feature tree.
#[component]
fn SketchItem(sketch: SketchUI) -> impl IntoView {
    let name = (*sketch.name);

    view! {
        <div class="flex flex-row items-center gap-1 rounded-full px-2 py-0.5 min-w-0 overflow-hidden text-white/80 hover:text-white hover:bg-white/10 transition-colors">
            <Icon icon=IconType::Sketch(IconSize::ExtraSmall) />
            <span class="text-xs truncate flex content-center">{name}</span>
        </div>
    }
}

/// A body row of featur
#[component]
fn BodyFeature(id: BodyId) -> impl IntoView {
    let app_store = use_context::<Store<AppStore>>().expect("AppStore should be provided");

    let UseActionReturn { dispatch, .. } = use_action();
    let body = BodyUI::from_store(app_store, id);
    let feature_tree = FeatureTreeUI::from_store(app_store, id);

    view! {
        <TreeAccordion
            node={
                let body = body.clone();
                move || {
                    let body = body.clone();
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
            }
            initial_open=true
        >
            {move || {
                let features = feature_tree.features.get();
                if features.is_empty() {
                    view! {
                        <span class="text-xs text-white/50 italic px-1 py-0.5">"No sketches"</span>
                    }
                        .into_any()
                } else {
                    features
                        .into_iter()
                        .map(|feature| match feature {
                            FeatureUIType::Sketch(sketch_id) => {
                                let sketch = SketchUI::from_store(app_store, sketch_id);
                                view! { <SketchItem sketch=sketch /> }.into_any()
                            }
                        })
                        .collect_view()
                        .into_any()
                }
            }}
        </TreeAccordion>
    }
}

/// A component for feature island that displays the feature tree.
#[component]
pub fn FeatureIsland() -> impl IntoView {
    let store = use_context::<Store<AppStore>>().expect("UiStore should be provided");

    let bodies = BodiesUI::from_store(store);

    view! {
        <div class="flex flex-col h-full w-full rounded-lg bg-white/10 backdrop-blur-sm border border-white/20 p-2 overflow-y-auto">
            <h3 class="text-xs font-semibold text-white/70 uppercase tracking-wider mb-2 px-1">
                "Features"
            </h3>

            <For
                each=move || bodies.bodies.get()
                key=|id| *id
                children=move |id| view! { <BodyFeature id=id /> }
            ></For>
        </div>
    }
}
