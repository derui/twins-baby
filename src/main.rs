mod bevy_app;
mod events;
mod leptos_app;

use leptos::mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(leptos_app::App)
}
