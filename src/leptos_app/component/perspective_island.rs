use leptos::prelude::*;
use leptos::{IntoView, component, view};

use crate::{
    events::PerspectiveKind,
    leptos_app::{
        component::SketchIsland,
        use_perspective::{UsePerspective, use_perspective},
    },
};

/// A component for perspective island.
#[component]
pub fn PerspectiveIsland() -> impl IntoView {
    let UsePerspective {
        perspective,
        set_perspective: _,
    } = use_perspective();

    view! {
        <div class="flex flex-col h-full w-full col-span-5 rounded-lg bg-gray-900/90">
            <Show when=move || perspective.get() == PerspectiveKind::Sketch>
                <SketchIsland />
            </Show>
        </div>
    }
}
