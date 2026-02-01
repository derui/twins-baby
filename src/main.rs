mod bevy_app;
mod events;
mod leptos_app;
mod resize_nob;
#[cfg(test)]
mod test_leptos;
mod use_resize;

use leptos::mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(leptos_app::App)
}
