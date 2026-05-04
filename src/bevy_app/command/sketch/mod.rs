use bevy::prelude::*;
use cad_base::{
    body::{BodyPerspective, PlaneRef},
    id::SketchId,
    plane::Plane,
    point::Point,
    sketch::{AttachableTarget, SketchPerspective},
};
use ui_event::{
    Correlation, ObjectType, SketchCreationFailure,
    command::CreateSketchOnSelectedCommand,
    notification::{Notifications, SketchCreatedNotification, SketchCreationFailedNotification},
    server::{ObjectSelectionChangeServerIntent, ServerIntent, ServerIntents},
};

use crate::bevy_app::{
    camera::{CameraMoveDuration, CameraMoveOperation, CameraMoveRequest},
    component::BodyPartType,
    resource::{EngineAppState, EngineState},
    support::Vec3Ext as _,
};

#[cfg(test)]
mod tests;

/// Convert selected object to attachable target. Only plane and face can be attachable target.
fn to_attachable_target(engine: &EngineAppState) -> Option<PlaneRef> {
    let Some(body_id) = engine.active_body else {
        return None;
    };

    if engine.selections.len() != 1 {
        return None;
    }

    match engine.selections[0] {
        (_, BodyPartType(ObjectType::Plane(plane_ref))) => {
            if plane_ref.body_id() == body_id {
                Some(plane_ref)
            } else {
                None
            }
        }
        (_, BodyPartType(ObjectType::Face(_))) => None,
        (_, BodyPartType(ObjectType::Edge(_))) => None,
        (_, BodyPartType(ObjectType::Point)) => None,
    }
}

/// A command to create sketch on the plane.
pub(super) fn on_create_sketch_on_plane(
    trigger: On<Correlation<CreateSketchOnSelectedCommand>>,
    mut engine: ResMut<EngineState>,
    app_state: Res<EngineAppState>,
    mut writer: MessageWriter<Correlation<Notifications>>,
    mut intent: MessageWriter<ServerIntents>,
    mut commands: Commands,
) {
    let command = trigger.event();

    let Some(target) = to_attachable_target(&app_state) else {
        writer.write(
            command.correlate(
                SketchCreationFailedNotification {
                    reason: SketchCreationFailure::TargetIsNotValid.into(),
                }
                .into(),
            ),
        );
        return;
    };

    let mut transaction = engine.0.begin();

    let created_sketch: SketchId;
    let sketch_name: String;

    {
        let Some(sketch_p) = transaction.modify::<SketchPerspective>() else {
            tracing::warn!("Can not get sketch perspective");
            return;
        };

        created_sketch = sketch_p.add_sketch(&AttachableTarget::Plane(target));
        sketch_name = sketch_p
            .get(&created_sketch)
            .map(|v| (*v.name).clone())
            .expect("Should be found");
    }

    let camera_target: Plane;
    if let Some(body_p) = transaction.modify::<BodyPerspective>()
        && let Some(body) = body_p.get_mut(&target.body_id())
    {
        body.add_sketch(&created_sketch);
        camera_target = target.to_plane_from(body);
    } else {
        tracing::warn!("Can not get body");
        return;
    };

    transaction.commit();

    writer.write(
        trigger.event().correlate(
            SketchCreatedNotification {
                sketch_id: created_sketch.into(),
                name: sketch_name.into(),
                body_id: target.body_id().into(),
            }
            .into(),
        ),
    );

    // reset selection
    intent.write(
        ObjectSelectionChangeServerIntent {
            selections: Vec::new(),
        }
        .into(),
    );

    {
        let target = camera_target.r0.to_vec3();
        let position = camera_target.normal.to_vec3() * 2.;
        commands.spawn(CameraMoveRequest::new(
            CameraMoveOperation::BySystem {
                target: target,
                position: target + position,
                pitch: None,
                yaw: None,
            },
            CameraMoveDuration::Duration(1.),
        ));
    }
}
