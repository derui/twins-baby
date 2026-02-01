
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::{
    bevy_app::init_bevy_app,
    events::LoggingEvent,
};

#[component]
pub fn App() -> impl IntoView {
    let (_log_receiver, log_sender) = message_b2l::<LoggingEvent>();

    view! {
        <div class="flex flex-col gap-5 items-center p-5 mx-auto h-full w-full ">
            <BevyCanvas init=move || { init_bevy_app(log_sender) } {..} />
        </div>
    }
}
