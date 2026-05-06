pub mod anchor;
pub mod components;
mod gizmo;
mod navigation_cube;

use bevy::ecs::{error::BevyError, system::Commands};
use bevy::prelude::*;
use bevy::scene::SceneInstance;

use crate::bevy_app::ui::components::{
    HudRotation, NavigationCube, NeedsRenderLayers, NeedsTextureSetup,
};
use crate::bevy_app::ui::gizmo::setup_gizmos;
use crate::bevy_app::ui::navigation_cube::setup_navigation_cube;

pub use gizmo::AxesGizmoGroup;
pub use gizmo::SketchBaseGizmoGroup;
pub use gizmo::draw_gizmos;
pub use navigation_cube::setup_navigation_texture;

pub trait AppUiExt {
    /// Init UI resources
    fn init_ui(&mut self) -> &mut Self;
}

impl AppUiExt for App {
    fn init_ui(&mut self) -> &mut Self {
        self.add_systems(Startup, (setup_ui, setup_navigation_cube, setup_gizmos))
            .add_systems(Update, (setup_navigation_texture, insert_render_layer))
    }
}

/// Setup the twins-baby UI elements
fn setup_ui(mut commands: Commands) -> Result<(), BevyError> {
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 600.,
        ..default()
    });

    commands.spawn(HudRotation::default());

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
        commands.entity(entity).insert(Visibility::Inherited);
    }

    Ok(())
}
