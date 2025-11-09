mod camera;
mod pan_orbit;
mod setup;
mod ui;

// This initializes a normal Bevy app
use bevy::{asset::AssetMetaCheck, prelude::*};
use leptos_bevy_canvas::prelude::{BevyEventSender, LeptosBevyApp};

use crate::{
    bevy_app::{
        camera::{PanOrbitOperation, move_camera_with_request, setup_camera},
        pan_orbit::{pan_orbit_camera, setup_pan_orbit},
        setup::setup_scene,
        ui::{insert_render_layer, setup_navigation_texture, setup_ui},
    },
    events::LoggingEvent,
};

pub fn init_bevy_app(logger: BevyEventSender<LoggingEvent>) -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // "#bevy_canvas" is the default and can be
                    // changed in the <BevyCanvas> component
                    canvas: Some("#bevy_canvas".into()),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
        MeshPickingPlugin,
    ))
    .export_event_to_leptos(logger)
    .add_systems(
        Startup,
        (setup_scene, setup_camera, setup_ui, setup_pan_orbit),
    )
    .add_systems(Update, setup_navigation_texture)
    .add_systems(
        Update,
        (
            insert_render_layer,
            (
                pan_orbit_camera.run_if(any_with_component::<PanOrbitOperation>),
                move_camera_with_request,
            )
                .chain(),
        ),
    );

    app
}
