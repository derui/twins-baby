use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::{bevy_app::init_bevy_app, events::LoggingEvent};

#[component]
pub fn App() -> impl IntoView {
    let (_log_receiver, log_sender) = message_b2l::<LoggingEvent>();
    let grid_cols_template = format!("grid-cols-[240px_minmax(600px,1fr)_240px]");
    let grid_rows_template = format!("grid-rows-[120px_minmax(480px,1fr)_120px]");
    let whole_class = format!(
        "grid gap-5 items-center p-5 mx-auto h-full w-full {} {}",
        grid_rows_template, grid_cols_template
    );

    view! {
        <div class=whole_class>
            <PerspectiveIsland />
            <FeatureIsland />
            <BevyCanvas init=move || { init_bevy_app(log_sender) } {..} />
            <SupportIsland />
            <InfoIsland />
        </div>
    }
}

/// A component for perspective island.
#[component]
pub fn PerspectiveIsland() -> impl IntoView {
    view! { <div class="flex flex-row h-full w-full col-span-3"></div> }
}

/// A component for feature island.
#[component]
pub fn FeatureIsland() -> impl IntoView {
    view! { <div class="flex flex-col h-full w-full"></div> }
}

/// A component for support island.
#[component]
pub fn SupportIsland() -> impl IntoView {
    view! { <div class="flex flex-col h-full w-full"></div> }
}

/// A component for info island.
#[component]
pub fn InfoIsland() -> impl IntoView {
    view! { <div class="flex flex-row h-full w-full col-span-3"></div> }
}
