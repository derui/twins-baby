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
    // Cube
    let cube = meshes.add(Cuboid::default());

    commands.spawn((
        NavigationCube,
        Mesh3d(cube.clone()),
        MeshMaterial3d(materials.add(Color::WHITE)),
        RenderLayers::layer(CAMERA_2D_LAYER),
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(5.)),
    ));

    Ok(())
}
