use leptos::prelude::*;
use leptos::{IntoView, component, view};
use ui_component::select::SelectBox;

use crate::{
    events::PerspectiveKind,
    leptos_app::{
        component::SketchIsland,
        use_perspective::{UsePerspective, use_perspective},
    },
};

fn perspective_item_view(kind: PerspectiveKind) -> AnyView {
    view! { <span class="px-2 py-1">{kind.to_string()}</span> }.into_any()
}

fn perspective_selected_view(kind: Option<PerspectiveKind>) -> AnyView {
    let label = kind.map(|k| k.to_string()).unwrap_or_default();
    view! { <span class="px-2 py-1">{label}</span> }.into_any()
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
        <div>
            <SelectBox
                items=items
                initial_selected=initial
                item_view=perspective_item_view
                selected_view=perspective_selected_view
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
            <Show when=move || perspective.get() == PerspectiveKind::Sketch>
                <SketchIsland />
            </Show>
        </div>
    }
}
