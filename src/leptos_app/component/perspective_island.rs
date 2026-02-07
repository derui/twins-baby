use leptos::prelude::*;
use leptos::{IntoView, component, view};
use leptos_bevy_canvas::prelude::LeptosMessageSender;

use crate::events::PerspectiveChangeEvent;
use crate::leptos_app::use_perspective::{UsePerspective, use_perspective};

/// A component for perspective island.
#[component]
pub fn PerspectiveIsland(sender: LeptosMessageSender<PerspectiveChangeEvent>) -> impl IntoView {
    let UsePerspective {
        perspective: _,
        set_perspective: _,
    } = use_perspective(sender);

    view! { <div class="flex flex-col h-full w-full col-span-5 rounded-lg bg-white"></div> }
}
