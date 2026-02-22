use leptos::prelude::*;
use leptos::{IntoView, component, view};

use crate::leptos_app::use_perspective::{UsePerspective, use_perspective};

/// A component for perspective island.
#[component]
pub fn PerspectiveIsland() -> impl IntoView {
    let UsePerspective {
        perspective: _,
        set_perspective: _,
    } = use_perspective();

    view! { <div class="flex flex-col h-full w-full col-span-5 rounded-lg bg-gray-900/90"></div> }
}
