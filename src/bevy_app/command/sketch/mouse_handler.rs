// Mouse handler for sketch commands.
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use crate::bevy_app::{
    component::{RequestedGeometryOperation, sketch::GeometryOperation},
    support::Vec3Ext,
};

/// handle mouse events while geometry creation operation.
///
/// this handles:
/// - convert click point in the window to the point on the attachable target
/// - Step forward the operation.
/// - finalize operation if it completed.
pub fn handle_geometry_operation(
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut processing: Query<(
        Entity,
        &mut RequestedGeometryOperation,
        &mut GeometryOperation,
    )>,
) {
    let just_activated = mouse.just_pressed(MouseButton::Left);

    // Upon starting a new orbit maneuver
    if just_activated {
        // check if the cursor is inside the window and get its position
        let Some(cursor_position) = q_window.single().expect("Should get").cursor_position() else {
            return;
        };

        let Ok((e, mut op, mut geo)) = processing.single_mut() else {
            return;
        };
    }
}
