use bevy::color::palettes::css::PINK;
use bevy::color::palettes::tailwind::PINK_500;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        error::BevyError,
        system::{Commands, ResMut},
    },
    math::{Vec3, primitives::Cuboid},
    pbr::{MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    transform::components::Transform,
};

use crate::bevy_app::camera::CAMERA_2D_LAYER;

#[derive(Component)]
pub struct NavigationCube;

/// Setup the twins-baby UI elements
pub fn setup_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result<(), BevyError> {
    // Navigation Cube
    let cube = meshes.add(Cuboid::default());

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(materials.add(Color::from(PINK_500))),
        RenderLayers::layer(CAMERA_2D_LAYER),
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(5.)),
        NavigationCube,
    ));

    // Light for UI
    commands.spawn((
        PointLight {
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..Default::default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
        RenderLayers::layer(CAMERA_2D_LAYER),
    ));

    Ok(())
}
