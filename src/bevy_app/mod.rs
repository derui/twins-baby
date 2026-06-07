mod camera;
mod command;
mod component;
mod pan_orbit;
mod picking;
mod resize;
mod resource;
mod setup;
pub(crate) mod support;
mod ui;

// This initializes a normal Bevy app
use bevy::{asset::AssetMetaCheck, picking::input::PointerInputSettings, prelude::*};
use leptos_bevy_canvas::prelude::{BevyMessageReceiver, BevyMessageSender, LeptosBevyApp};
use ui_event::{
    Correlation, command::Commands, intent::Intents, notification::Notifications,
    server::ServerIntents,
};

use crate::bevy_app::{
    camera::{
        LastWindowSize, PanOrbitOperation, move_camera_with_request, reposition_ui_cameras,
        setup_camera,
    },
    command::CommandAppExt,
    pan_orbit::{pan_orbit_camera, setup_pan_orbit},
    picking::{PickingMessages, update_toggling_selection},
    resize::WindowResizePlugin,
    resource::{AppResourceExt, VisualConfiguration},
    setup::setup_scene,
    ui::{
        AppUiExt, AxesGizmoGroup, SketchBaseGizmoGroup, anchor::transform_ui_anchors, draw_gizmos,
        draw_sketch_gizmos,
    },
};

/// Settings for bevy application, to pass massive message recievers
#[derive(Debug)]
pub struct BevyAppSettings {
    pub intent: BevyMessageReceiver<Intents>,
    pub command: BevyMessageReceiver<Correlation<Commands>>,
    pub notification: BevyMessageSender<Correlation<Notifications>>,
    pub server_intent: BevyMessageSender<ServerIntents>,
}

pub fn init_bevy_app(setting: BevyAppSettings) -> App {
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
            })
            .disable::<GilrsPlugin>(),
        MeshPickingPlugin,
        WindowResizePlugin,
    ))
    .init_gizmo_group::<AxesGizmoGroup>()
    .init_gizmo_group::<SketchBaseGizmoGroup>()
    .init_resource::<LastWindowSize>()
    .insert_resource(PointerInputSettings {
        is_touch_enabled: true,
        is_mouse_enabled: true,
    })
    .init_resource::<Messages<PickingMessages>>()
    .insert_resource(ClearColor(Color::srgb(0.7, 0.7, 0.7)))
    .init_app_resources()
    .import_message_from_leptos(setting.intent)
    .import_message_from_leptos(setting.command)
    .export_message_to_leptos(setting.notification)
    .export_message_to_leptos(setting.server_intent)
    .register_commands()
    .init_ui()
    .add_systems(Startup, (setup_scene, setup_camera, setup_pan_orbit))
    .add_systems(
        Update,
        (
            reposition_ui_cameras,
            (
                pan_orbit_camera.run_if(any_with_component::<PanOrbitOperation>),
                move_camera_with_request,
                transform_ui_anchors,
                draw_gizmos,
                draw_sketch_gizmos,
            )
                .chain(),
        ),
    )
    .add_systems(Update, update_toggling_selection);

    app
}
