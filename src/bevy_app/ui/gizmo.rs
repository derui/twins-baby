use bevy::{
    camera::visibility::RenderLayers,
    ecs::{
        entity::Entity,
        error::BevyError,
        query::With,
        system::{Commands, Query, ResMut},
    },
    gizmos::{
        config::{GizmoConfigGroup, GizmoConfigStore},
        gizmos::Gizmos,
    },
    math::Vec3,
    reflect::Reflect,
    transform::components::Transform,
};

use crate::bevy_app::{camera::CAMERA_GIZMO_LAYER, ui::components::AxesGizmo};

// 2.5unit = 25px per line
const GIZMO_LENGTH: f32 = 2.5;

/// Gizmo configuration group for Axes
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct AxesGizmoGroup;

/// Setup Gizmos on the scene
///
/// Our gizmos are these:
/// - An axis of red, green, and blue lines representing the X, Y, and Z axes respectively.
///   - It is rendered on the 2D camera layer so that it is always visible.
pub fn setup_gizmos(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
) -> Result<(), BevyError> {
    // it needs only transformation
    commands.spawn((
        Transform::from_scale(Vec3::splat(1.)),
        AxesGizmo,
        RenderLayers::from_layers(&[CAMERA_GIZMO_LAYER]),
    ));

    // Configure the gizmo group to render on the gizmo layer
    let (config, _) = config_store.config_mut::<AxesGizmoGroup>();
    config.render_layers = RenderLayers::from_layers(&[CAMERA_GIZMO_LAYER]);
    config.line.width = 2.0;

    Ok(())
}

/// draw gizmos
pub fn draw_gizmos(
    mut gizmos: Gizmos<AxesGizmoGroup>,
    arrow_gizmo: Query<(Entity, &Transform), With<AxesGizmo>>,
) -> Result<(), BevyError> {
    for (_, transform) in &arrow_gizmo {
        gizmos.axes(*transform, GIZMO_LENGTH)
    }

    Ok(())
}
