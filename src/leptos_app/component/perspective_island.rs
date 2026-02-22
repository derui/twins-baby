use leptos::prelude::*;
use leptos::{IntoView, component, view};

use crate::{
    events::PerspectiveKind,
    leptos_app::{
        component::SketchIsland,
        use_perspective::{UsePerspective, use_perspective},
    },
};

/// Switcher of perspective
fn PerspectiveSwitcher() -> impl IntoView {
    let UsePerspective {
        perspective,
        set_perspective,
    } = use_perspective();

    let feature_selected = move || perspective.get() == PerspectiveKind::Feature;
    let sketch_selected = move || perspective.get() == PerspectiveKind::Sketch;

    view! {
        <div>
            <select on:change:target=move |ev| {
                let Ok(kind) = PerspectiveKind::from_string(&ev.target().value()) else {
                    return;
                };
                set_perspective.run(kind);
            }>
                <option value=PerspectiveKind::Feature.to_string() selected=feature_selected>
                    Feature
                </option>
                <option value=PerspectiveKind::Sketch.to_string() selected=sketch_selected>
                    Sketch
                </option>
            </select>
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
