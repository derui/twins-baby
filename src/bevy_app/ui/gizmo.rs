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
use cad_base::{
    body::BodyPerspective,
    sketch::{AttachableTarget, SketchPerspective},
};

use crate::bevy_app::{
    camera::{CAMERA_3D_LAYER, CAMERA_UI_LAYER},
    resource::{AppActiveSketch, EngineState},
    support::Vec3Ext,
    ui::components::{AxesGizmo, HudAnchor, SketchBaseGizmo},
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
    mut engine: ResMut<EngineState>,
    sketches: Query<(Entity, &Transform), With<SketchBaseGizmo>>,
) {
    let Some(sketch_id) = active_sketch.0 else {
        return;
    };

    let transaction = engine.0.begin();
    let Some(sketch_p) = transaction.read::<SketchPerspective>() else {
        return;
    };
    let Some(sketch) = sketch_p.get(&sketch_id) else {
        return;
    };

    let normal = match &*sketch.attach_target {
        AttachableTarget::Plane(plane_ref) => {
            let Some(body_p) = transaction.read::<BodyPerspective>() else {
                return;
            };
            let Some(body) = body_p.get(&plane_ref.body_id()) else {
                return;
            };
            let plane = plane_ref.to_plane_from(body);
            plane.normal.to_vec3()
        }
        AttachableTarget::Face(_) => {
            // TODO: derive normal from solid face
            Vec3::Z
        }
    };

    let (axis_u, axis_v) = normal.any_orthonormal_pair();

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
