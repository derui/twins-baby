mod components;
mod gizmo;
mod navigation_cube;

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::scene::SceneInstance;
use bevy::{
    ecs::{error::BevyError, system::Commands},
    math::Vec3,
    transform::components::Transform,
};

use crate::bevy_app::camera::CAMERA_CUBE_LAYER;
use crate::bevy_app::ui::components::{NavigationCube, NeedsRenderLayers, NeedsTextureSetup};

const NAVIGATION_CUBE_SCALE: f32 = 1.0 * 4.; // 100 = 1mm to 1m, 4 to 4unit = 40px on UI

pub use gizmo::AxesGizmoGroup;
pub use gizmo::draw_gizmos;
pub use gizmo::setup_gizmos;
pub use navigation_cube::setup_navigation_texture;

/// Setup the twins-baby UI elements
pub fn setup_ui(mut commands: Commands, asset: Res<AssetServer>) -> Result<(), BevyError> {
    // Navigation Cube
    let cube = asset.load(GltfAssetLabel::Scene(0).from_asset("navigation-cube.gltf"));

    commands.spawn((
        SceneRoot(cube),
        // current navigation cube model is located XY plane. so translate it a bit down to avoid z-fighting with grid.
        Transform::from_scale(Vec3::splat(NAVIGATION_CUBE_SCALE))
            .with_translation(Vec3::new(0., 0., 0.)),
        NavigationCube,
        NeedsRenderLayers(RenderLayers::layer(CAMERA_CUBE_LAYER)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 600.,
        ..default()
    });

    Ok(())
}

/// Setup navigation cube as UI element.
///
/// glTF scene with render layer can not reflect children to the same render layer, so
/// we should do it manually.
pub fn insert_render_layer(
    mut commands: Commands,
    scenes: Query<(Entity, &SceneInstance, &NeedsRenderLayers)>,
    scene_spawmer: Res<SceneSpawner>,
) -> Result<(), BevyError> {
    for (entity, instance, needs_render_layers) in &scenes {
        if !scene_spawmer.instance_is_ready(**instance) {
            continue;
        }

        scene_spawmer
            .iter_instance_entities(**instance)
            .for_each(|e| {
                commands.entity(e).insert(needs_render_layers.0.clone());
                commands.entity(e).insert(NavigationCube);
                commands.entity(e).insert(NeedsTextureSetup);
            });

        commands.entity(entity).remove::<NeedsRenderLayers>();
    }

    Ok(())
}
