use bevy::{
    camera::visibility::RenderLayers,
    color::{
        Color,
        palettes::css::{GREEN, RED},
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
        primitives::dim3::GizmoPrimitive3d,
    },
    math::{Dir3, Vec3, primitives::Line3d},
    reflect::Reflect,
    transform::components::Transform,
};
use cad_base::sketch::SketchPerspective;

use crate::bevy_app::{
    camera::{CAMERA_3D_LAYER, CAMERA_UI_LAYER},
    component::ui::{AxesGizmo, HudAnchor, SketchBaseGizmo},
    resource::{AppActiveSketch, EngineState},
    support::Vec3Ext,
};

// 2.5unit = 25px per line
const GIZMO_LENGTH: f32 = 2.5;

/// Gizmo configuration group for Axes
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct AxesGizmoGroup;

/// Gizmo configuration group for Sketch perspective
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SketchBaseGizmoGroup;

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
        HudAnchor::Axes,
    ));

    {
        // Configure the gizmo group to render on the gizmo layer
        let (config, _) = config_store.config_mut::<AxesGizmoGroup>();
        config.render_layers = RenderLayers::from_layers(&[CAMERA_UI_LAYER]);
        config.line.width = 2.0;
    }

    {
        // It needs only transformation
        commands.spawn((SketchBaseGizmo, Transform::from_scale(Vec3::splat(100.))));

        let (config, _) = config_store.config_mut::<SketchBaseGizmoGroup>();
        config.render_layers = RenderLayers::from_layers(&[CAMERA_3D_LAYER]);
        config.line.width = 2.0;
    }

    Ok(())
}

/// draw axes gizmos
pub fn draw_gizmos(
    mut gizmos: Gizmos<AxesGizmoGroup>,
    arrow_gizmo: Query<(Entity, &Transform), With<AxesGizmo>>,
) {
    for (_, transform) in &arrow_gizmo {
        gizmos.axes(*transform, GIZMO_LENGTH)
    }
}

/// draw sketch gizmos
pub fn draw_sketch_gizmos(
    mut gizmos_sketch: Gizmos<SketchBaseGizmoGroup>,
    active_sketch: Res<AppActiveSketch>,
    engine: Res<EngineState>,
    sketches: Query<(Entity, &Transform), With<SketchBaseGizmo>>,
) {
    let Some(sketch_id) = active_sketch.0 else {
        return;
    };

    let baseline = engine.0.baseline();

    let Some(sketch) = baseline
        .read::<SketchPerspective>()
        .and_then(|p| p.get(&sketch_id))
    else {
        return;
    };

    let Some(normal) = sketch
        .attach_target
        .to_plane(&baseline)
        .map(|plane| plane.normal.to_vec3())
    else {
        return;
    };

    let (axis_u, axis_v) = normal.any_orthonormal_pair();

    // show lines through x/y axis on the attachable target
    for (_, transform) in &sketches {
        gizmos_sketch.primitive_3d(
            &Line3d {
                direction: Dir3::from_xyz(axis_u.x, axis_u.y, axis_u.z).unwrap(),
            },
            transform.translation,
            Color::from(RED),
        );

        gizmos_sketch.primitive_3d(
            &Line3d {
                direction: Dir3::from_xyz(axis_v.x, axis_v.y, axis_v.z).unwrap(),
            },
            transform.translation,
            Color::from(GREEN),
        );
    }
}
