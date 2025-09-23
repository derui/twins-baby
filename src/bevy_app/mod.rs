mod camera;
mod pan_orbit;
mod setup;
mod ui;

// This initializes a normal Bevy app
use bevy::prelude::*;
use leptos_bevy_canvas::prelude::{BevyEventSender, LeptosBevyApp};

use crate::{
    bevy_app::{
        camera::setup_camera,
        pan_orbit::{PanOrbitState, pan_orbit_camera},
        setup::setup_scene,
        ui::setup_ui,
    },
    events::LoggingEvent,
};

pub fn init_bevy_app(logger: BevyEventSender<LoggingEvent>) -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // "#bevy_canvas" is the default and can be
                // changed in the <BevyCanvas> component
                canvas: Some("#bevy_canvas".into()),
                ..default()
            }),
            ..default()
        }),
        MeshPickingPlugin,
    ))
    .export_event_to_leptos(logger)
    .add_systems(Startup, (setup_scene, setup_camera, setup_ui))
    .add_systems(
        Update,
        pan_orbit_camera.run_if(any_with_component::<PanOrbitState>),
    );

    app
}
