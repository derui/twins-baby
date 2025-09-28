use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::{
    asset::{AssetServer, Assets},
    color::{Color, palettes::tailwind::RED_500},
    ecs::{
        error::BevyError,
        system::{Commands, Res, ResMut},
    },
    math::{Vec3, primitives::Cuboid},
    pbr::{MeshMaterial3d, PointLight, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    transform::components::Transform,
};

use crate::bevy_app::camera::CAMERA_3D_LAYER;

const COBE_SCALE: f32 = 3.0;

/// Setup the scene
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..Default::default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
        RenderLayers::from_layers(&[CAMERA_3D_LAYER]),
    ));

    Ok(())
}
