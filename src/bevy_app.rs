// This initializes a normal Bevy app
use bevy::prelude::*;

pub fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // "#bevy_canvas" is the default and can be
            // changed in the <BevyCanvas> component
            canvas: Some("#bevy_canvas".into()),
            ..default()
        }),
        ..default()
    }));

    app
}
