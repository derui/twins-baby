use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::{
    asset::Assets,
    color::{Color, palettes::tailwind::RED_500},
    ecs::{
        error::BevyError,
        system::{Commands, ResMut},
    },
    math::{Vec3, primitives::Cuboid},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
};

use crate::bevy_app::camera::CAMERA_3D_LAYER;

const COBE_SCALE: f32 = 3.0;

/// Setup the scene
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result<(), BevyError> {
    // Cube
    let cube = meshes.add(Cuboid::default());

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(materials.add(Color::from(RED_500))),
        Transform::from_xyz(0., 0., 0.0).with_scale(Vec3::splat(COBE_SCALE)),
        RenderLayers::layer(CAMERA_3D_LAYER),
    ));

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
