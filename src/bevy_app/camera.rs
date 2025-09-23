use bevy::{
    prelude::*,
    render::{render_graph::RenderLabel, view::RenderLayers},
};

use crate::bevy_app::pan_orbit::PanOrbitCameraBundle;

pub const CAMERA_3D_LAYER: usize = 0;
pub const CAMERA_2D_LAYER: usize = 1;

/// This module provides 3D camera basic functionally in Bevy.

/// Setup camera with pan-orbit controller
pub fn setup_camera(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
) -> Result<(), BevyError> {
    let mut camera = PanOrbitCameraBundle::default();

    camera.state.center = Vec3::new(1.0, 2.0, 3.0);
    camera.state.radius = 50.0;
    camera.state.pitch = 15.0f32.to_radians();
    camera.state.yaw = 30.0f32.to_radians();

    commands.spawn((camera, RenderLayers::from_layers(&[CAMERA_3D_LAYER])));

    commands.spawn((
        Camera2d::default(),
        Camera {
            // clear color, use background
            clear_color: ClearColorConfig::None,
            order: 1,
            ..default()
        },
        RenderLayers::from_layers(&[CAMERA_2D_LAYER]),
    ));

    Ok(())
}
