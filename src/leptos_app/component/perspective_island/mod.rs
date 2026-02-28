mod feature_toolbar;
mod sketch_toolbar;

use leptos::prelude::*;
use leptos::{IntoView, component, view};
use ui_component::select::SelectBox;
use ui_event::PerspectiveKind;

use crate::leptos_app::component::perspective_island::feature_toolbar::FeatureToolbar;
use crate::leptos_app::component::perspective_island::sketch_toolbar::SketchToolbar;
use crate::leptos_app::use_perspective::{UsePerspective, use_perspective};

fn perspective_item_view(kind: PerspectiveKind) -> AnyView {
    view! {
        <span class="px-3 py-1 text-sm text-gray-200 hover:bg-gray-600 w-full block">
            {kind.to_string()}
        </span>
    }
    .into_any()
}

fn perspective_selected_view(kind: Option<PerspectiveKind>) -> AnyView {
    let label = kind.map(|k| k.to_string()).unwrap_or_default();
    view! { <span class="px-3 py-1 text-sm text-gray-200">{label}</span> }.into_any()
}

/// Switcher of perspective
#[component]
fn PerspectiveSwitcher() -> impl IntoView {
    let UsePerspective {
        perspective,
        set_perspective,
    } = use_perspective();

    let items = vec![PerspectiveKind::Feature, PerspectiveKind::Sketch];
    let initial = perspective.get_untracked();

    view! {
        <div class="flex items-center px-2 border-r border-gray-600">
            <SelectBox
                items=items
                initial_selected=initial
                item_view=perspective_item_view
                selected_view=perspective_selected_view
                button_class="p-2 rounded-xl border border-white/10 bg-black/50 shadow-lg backdrop-blur-md hover:bg-black/70 transition-colors text-gray-200 text-sm"
                on_change=Callback::new(move |kind: Option<PerspectiveKind>| {
                    if let Some(k) = kind {
                        set_perspective.run(k);
                    }
                })
            />
        </div>
    }
}

/// A component for perspective island.
#[component]
pub fn PerspectiveIsland() -> impl IntoView {
    let UsePerspective { perspective, .. } = use_perspective();

    view! {
        <div class="flex flex-row h-full w-full col-span-5 rounded-lg bg-gray-700/90">
            <PerspectiveSwitcher />
            <Show when=move || perspective.get() == PerspectiveKind::Feature>
                <FeatureToolbar />
            </Show>
            <Show when=move || perspective.get() == PerspectiveKind::Sketch>
                <SketchToolbar />
            </Show>
        </div>
    }
}
