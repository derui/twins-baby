use bevy::prelude::*;
use bevy::{
    asset::{AssetServer, Assets, transformer::TransformedAsset},
    color::{
        Color,
        palettes::tailwind::{RED_50, RED_500},
    },
    core_pipeline::core_3d::Camera3d,
    ecs::{
        error::{BevyError, default_error_handler},
        system::{Commands, Res, ResMut},
    },
    math::{Vec3, primitives::Cuboid},
    pbr::{
        MeshMaterial3d, PointLight, StandardMaterial, environment_map::EnvironmentMapLight,
        wireframe::Mesh3dWireframe,
    },
    render::{
        mesh::{Mesh, Mesh3d},
        render_resource::Face,
    },
    transform::components::Transform,
};

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
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));

    Ok(())
}
