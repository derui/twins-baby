use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

use crate::bevy_app::init_bevy_app;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex gap-5 items-center p-5 mx-auto w-full max-w-[1400px]">
            <Frame class="border-red-500 flex-4 bg-red-500/5" {..}>
                <div class="float-right">Click on a cube to select</div>

                <h2 class="relative text-xl font-bold text-red-500 top-[-10px]">Bevy</h2>
                <div
                    class="overflow-hidden rounded-lg aspect-[8/5]"
                    style:max-width="100%"
                    style:max-height="100%"
                >

                    <BevyCanvas init=move || { init_bevy_app() } {..} width="300" height="500" />
                </div>
            </Frame>
        </div>
    }
}

#[component]
pub fn Frame(class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("border-2 border-solid {class} rounded-lg p-5")>{children()}</div> }
}
