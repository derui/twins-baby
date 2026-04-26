use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        error::BevyError,
        system::{Commands, ResMut},
    },
    pbr::StandardMaterial,
};

use crate::bevy_app::camera::CAMERA_3D_LAYER;

const COBE_SCALE: f32 = 3.0;

/// Setup the scene
pub fn setup_scene(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) -> Result<(), BevyError> {
    // Light
    commands.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 300.,
            ..default()
        },
        RenderLayers::from_layers(&[CAMERA_3D_LAYER]),
    ));

    Ok(())
}
