use bevy::prelude::*;
use leptos::prelude::window;

/// From https://github.com/Leinnan/bevy_wasm_window_resize/blob/master/src/lib.rs
pub struct WindowResizePlugin;

impl Plugin for WindowResizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_browser_resize);
    }
}

fn handle_browser_resize(
    mut primary_query: bevy::ecs::system::Query<
        &mut bevy::window::Window,
        bevy::ecs::query::With<bevy::window::PrimaryWindow>,
    >,
) {
    // Our app depends on leptus...
    let wasm_window = window();

    let Ok(inner_width) = wasm_window.inner_width() else {
        return;
    };
    let Ok(inner_height) = wasm_window.inner_height() else {
        return;
    };
    let Some(target_width) = inner_width.as_f64() else {
        return;
    };
    let Some(target_height) = inner_height.as_f64() else {
        return;
    };
    for mut window in &mut primary_query {
        if window.resolution.width() != (target_width as f32)
            || window.resolution.height() != (target_height as f32)
        {
            window
                .resolution
                .set(target_width as f32, target_height as f32);
        }
    }
}
