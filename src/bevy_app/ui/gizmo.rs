use bevy::{
    camera::visibility::RenderLayers,
    color::{
        Color,
        palettes::css::{BLUE, GREEN, RED},
    },
    ecs::{
        entity::Entity,
        error::BevyError,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    gizmos::{
        config::{GizmoConfigGroup, GizmoConfigStore},
        gizmos::Gizmos,
    },
    math::Vec3,
    reflect::Reflect,
    transform::components::Transform,
};

use crate::bevy_app::{
    camera::{CAMERA_3D_LAYER, CAMERA_GIZMO_LAYER},
    ui::components::{AxesGizmo, SketchBaseGizmo},
};

// 2.5unit = 25px per line
const GIZMO_LENGTH: f32 = 2.5;

/// Gizmo configuration group for Axes
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct AxesGizmoGroup;

/// Gizmo configuration group for Sketch perspective
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SketchBaseGizmoGroup {
    /// show gizmos
    pub show_sketch: bool,
}

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

    {
        // Configure the gizmo group to render on the gizmo layer
        let (config, _) = config_store.config_mut::<AxesGizmoGroup>();
        config.render_layers = RenderLayers::from_layers(&[CAMERA_GIZMO_LAYER]);
        config.line.width = 2.0;
    }

    {
        // It needs only transformation
        commands.spawn((
            SketchBaseGizmo,
            Transform::from_scale(Vec3::splat(100.)),
            RenderLayers::from_layers(&[CAMERA_3D_LAYER]),
        ));

        let (config, t) = config_store.config_mut::<SketchBaseGizmoGroup>();
        config.line.width = 2.0;
        // All axes are hidden in initial
        t.show_sketch = false;
    }

    Ok(())
}

/// draw axes gizmos
pub fn draw_gizmos(
    mut gizmos: Gizmos<AxesGizmoGroup>,
    mut gizmos_sketch: Gizmos<SketchBaseGizmoGroup>,
    config_store: Res<GizmoConfigStore>,
    arrow_gizmo: Query<(Entity, &Transform), With<AxesGizmo>>,
    sketches: Query<(Entity, &Transform), With<SketchBaseGizmo>>,
) {
    for (_, transform) in &arrow_gizmo {
        gizmos.axes(*transform, GIZMO_LENGTH)
    }

    let (_, config) = config_store.config::<SketchBaseGizmoGroup>();

    if !config.show_sketch {
        return;
    }

    // TODO place it on plane/face normal based
    for (_, transform) in &sketches {
        gizmos_sketch.line(
            *transform * Vec3::X,
            *transform * Vec3::NEG_X,
            Color::from(RED),
        );

        gizmos_sketch.line(
            *transform * Vec3::Y,
            *transform * Vec3::NEG_Y,
            Color::from(GREEN),
        );
    }
}
