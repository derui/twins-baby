use bevy::prelude::*;
use ui_event::CanvasResizeNotification;

/// From https://github.com/Leinnan/bevy_wasm_window_resize/blob/master/src/lib.rs
pub struct WindowResizePlugin;

impl Plugin for WindowResizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_browser_resize);
    }
}

/// handle resizing window each frames with message.
fn handle_browser_resize(
    mut message_reader: MessageReader<CanvasResizeNotification>,
    mut primary_query: bevy::ecs::system::Query<
        &mut bevy::window::Window,
        bevy::ecs::query::With<bevy::window::PrimaryWindow>,
    >,
) {
    for mut window in &mut primary_query {
        for message in message_reader.read() {
            if window.resolution.width() != (*message.width as f32)
                || window.resolution.height() != (*message.height as f32)
            {
                window
                    .resolution
                    .set(*message.width as f32, *message.height as f32);
            }
        }
    }
}
