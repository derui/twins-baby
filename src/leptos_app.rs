use std::time::Duration;

use bevy::{app::DynEq, window::WindowResized};
use leptos::prelude::*;
use leptos::{
    ev::{self, UiEvent},
    html::ElementType,
    prelude::*,
    tachys::renderer::dom::Element,
};
use leptos_bevy_canvas::prelude::*;
use leptos_use::{use_debounce_fn, use_debounce_fn_with_arg, use_window};

use crate::{
    bevy_app::init_bevy_app,
    events::{LogLevel, LoggingEvent},
};

#[component]
pub fn App() -> impl IntoView {
    let (log_receiver, log_sender) = message_b2l::<LoggingEvent>();

    view! {
        <div class="flex flex-col gap-5 items-center p-5 mx-auto h-full w-full ">
            <BevyCanvas init=move || { init_bevy_app(log_sender) } {..} />
        </div>
    }
}

#[component]
pub fn LogConsole(log_receiver: LeptosMessageReceiver<LoggingEvent>) -> impl IntoView {
    let (events, set_events) = signal(Vec::<LoggingEvent>::new());

    Effect::new(move || {
        if let Some(log) = log_receiver.get() {
            set_events.update(|events| {
                events.push(log);

                if events.len() >= 100 {
                    events.remove(0);
                }
            });
        }
    });

    view! {
        <div class="flex flex-row gap-2 h-[320px]">
            <div class="flex-1 overflow-y-auto border rounded p-2 bg-black text-white">
                <For
                    each=move || { events.get().into_iter().enumerate().collect::<Vec<_>>() }
                    key=|(i, _)| *i
                    children=|(_, log)| {
                        let color = match log.log_level {
                            LogLevel::Debug => "text-gray-400",
                            LogLevel::Info => "text-green-400",
                            LogLevel::Warning => "text-yellow-400",
                            LogLevel::Error => "text-red-400",
                        };
                        view! {
                            <div class=color>
                                {format!("{:?} [{:?}] {}", log.timestamp, log.log_level, log.text)}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
pub fn Frame(class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("border-2 border-solid {class} rounded-lg p-5")>{children()}</div> }
}
