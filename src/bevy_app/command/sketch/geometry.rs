// Mouse handler for sketch commands.
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use ui_event::SketchGeometryOperation;

use crate::bevy_app::{
    camera::MainCamera,
    component::{RequestedGeometryOperation, sketch::GeometryOperation},
    support::Vec3Ext,
};

/// The event to notify that a geometry operation is completed.
#[derive(Debug, Clone, Event)]
pub struct GeometryOperationCompletedEvent {
    /// A opelation that completed
    pub operation: SketchGeometryOperation,
    /// All points to create geometry
    pub points: Vec<Vec3>,
}

/// The systemt that handle mouse events while geometry creation operation.
///
/// this handles:
/// - convert click point in the window to the point on the attachable target
/// - Step forward the operation.
/// - finalize operation if it completed.
pub fn handle_geometry_operation(
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut processing: Query<(
        Entity,
        &mut RequestedGeometryOperation,
        &mut GeometryOperation,
    )>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let just_activated = mouse.just_pressed(MouseButton::Left);

    // handle only the button just pressed
    if !just_activated {
        return;
    }

    let Ok((camera, global_transform)) = q_camera.single() else {
        return;
    };
    // get global intersection position between sketch target and cursor
    let Some(cursor_position) = q_window.single().expect("Should get").cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(global_transform, cursor_position) else {
        return;
    };
    let Ok((e, ope, mut geo)) = processing.single_mut() else {
        return;
    };

    // the normal should be direction from local to global
    let normal = global_transform.transform_point(geo.plane.normal.to_vec3());

    if let Some(point) = ray.plane_intersection_point(ray.origin, InfinitePlane3d::new(normal)) {
        if let Err(_) = geo.forward_step(point) {
            // after operation finished, send event.
            commands.entity(e).despawn();

            commands.trigger(GeometryOperationCompletedEvent {
                operation: ope.0.clone(),
                points: geo.step_result().clone(),
            })
        }
    }
}
