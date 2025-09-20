use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::{
    bevy_app::init_bevy_app,
    events::{LogLevel, LoggingEvent},
};

#[component]
pub fn App() -> impl IntoView {
    let (log_receiver, log_sender) = event_b2l::<LoggingEvent>();

    view! {
        <div class="flex flex-col gap-5 items-center p-5 mx-auto h-full w-full max-w-[1400px]">
            <Frame class="border-red-500 flex-4 bg-red-500/5" {..}>
                <div class="float-right">Click on a cube to select</div>

                <h2 class="relative text-xl font-bold text-red-500 top-[-10px]">Bevy</h2>
                <div
                    class="overflow-hidden rounded-lg aspect-[8/5]"
                    style:max-width="100%"
                    style:max-height="100%"
                >

                    <BevyCanvas
                        init=move || { init_bevy_app(log_sender) }
                        {..}
                        width="300"
                        height="500"
                    />
                </div>
            </Frame>
            <LogConsole log_receiver></LogConsole>
        </div>
    }
}

#[component]
pub fn LogConsole(log_receiver: LeptosEventReceiver<LoggingEvent>) -> impl IntoView {
    let (events, set_events) = signal(Vec::<LoggingEvent>::new());

    Effect::new(move || {
        if let Some(log) = log_receiver.get() {
            set_events.update(|events| {
                events.push(log);
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
