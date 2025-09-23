use bevy::{
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
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
        Camera2d,
        Camera {
            // clear color, use background
            clear_color: ClearColorConfig::None,
            order: 1,
            // set the viewport to a 256x256 square in the top left corner
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 1),
                physical_size: Vec2::new(96., 96.).as_uvec2(),
                ..default()
            }),
            ..default()
        },
        RenderLayers::from_layers(&[CAMERA_2D_LAYER]),
    ));

    Ok(())
}
