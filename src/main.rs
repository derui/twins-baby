mod leptos_app;

use leptos::{
    leptos_dom::logging::{console_debug_error, console_error},
    mount::{self, mount_to, mount_to_body},
};

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(leptos_app::App)
}
