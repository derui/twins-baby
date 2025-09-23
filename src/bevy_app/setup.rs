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
    render::{
        mesh::{Mesh, Mesh3d},
        render_resource::Face,
    },
    transform::components::Transform,
};

use crate::bevy_app::camera::{CAMERA_2D_LAYER, CAMERA_3D_LAYER};

const CUBE_X: f32 = 4.0;
const CUBE_Y: f32 = 0.0;
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

    let highlight_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        cull_mode: Some(Face::Front),
        unlit: true,
        ..Default::default()
    });

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(materials.add(Color::from(RED_500))),
        Transform::from_xyz(-CUBE_X, CUBE_X, 0.0).with_scale(Vec3::splat(COBE_SCALE)),
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
        RenderLayers::from_layers(&[CAMERA_2D_LAYER, CAMERA_3D_LAYER]),
    ));

    Ok(())
}
