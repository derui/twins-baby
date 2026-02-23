mod camera;
mod pan_orbit;
mod resize;
mod setup;
mod ui;

// This initializes a normal Bevy app
use bevy::{asset::AssetMetaCheck, input::InputPlugin, prelude::*};
use leptos_bevy_canvas::prelude::{BevyMessageReceiver, LeptosBevyApp};
use ui_event::{
    CanvasResizeNotification, MouseDownNotification, MouseMovementNotification,
    MouseUpNotification, SketchToolChangeNotification,
};

use crate::bevy_app::{
    camera::{
        LastWindowSize, PanOrbitOperation, move_camera_with_request, reposition_ui_cameras,
        setup_camera,
    },
    pan_orbit::{pan_orbit_camera, setup_pan_orbit},
    resize::WindowResizePlugin,
    setup::setup_scene,
    ui::{
        AxesGizmoGroup, draw_gizmos, insert_render_layer, setup_gizmos, setup_navigation_texture,
        setup_ui,
    },
};

/// Settings for bevy application, to pass massive message recievers
#[derive(Debug)]
pub struct BevyAppSettings {
    pub canvas_resize: BevyMessageReceiver<CanvasResizeNotification>,
    pub sketch_tool_change: BevyMessageReceiver<SketchToolChangeNotification>,
    pub mouse_movement: BevyMessageReceiver<MouseMovementNotification>,
    pub mouse_down: BevyMessageReceiver<MouseDownNotification>,
    pub mouse_up: BevyMessageReceiver<MouseUpNotification>,
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
            // disable input to avoid configuration
            .disable::<InputPlugin>(),
        MeshPickingPlugin,
        WindowResizePlugin,
    ))
    .init_gizmo_group::<AxesGizmoGroup>()
    .init_resource::<LastWindowSize>()
    .insert_resource(ClearColor(Color::srgb(0.7, 0.7, 0.7)))
    .import_message_from_leptos(setting.canvas_resize)
    .import_message_from_leptos(setting.sketch_tool_change)
    .import_message_from_leptos(setting.mouse_movement)
    .import_message_from_leptos(setting.mouse_down)
    .import_message_from_leptos(setting.mouse_up)
    .add_systems(
        Startup,
        (
            setup_scene,
            setup_camera,
            setup_ui,
            setup_pan_orbit,
            setup_gizmos,
        ),
    )
    .add_systems(Update, setup_navigation_texture)
    .add_systems(
        Update,
        (
            insert_render_layer,
            reposition_ui_cameras,
            (
                pan_orbit_camera.run_if(any_with_component::<PanOrbitOperation>),
                move_camera_with_request,
                draw_gizmos,
            )
                .chain(),
        ),
    );

    app
}
