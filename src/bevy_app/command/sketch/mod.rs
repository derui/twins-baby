use bevy::prelude::*;
use cad_base::{
    body::{BodyPerspective, PlaneRef},
    id::SketchId,
    plane::Plane,
    sketch::{AttachableTarget, SketchPerspective},
};
use ui_event::{
    Correlation, ObjectType, SketchCreationFailure,
    command::CreateSketchOnSelectedCommand,
    notification::{Notifications, SketchCreatedNotification, SketchCreationFailedNotification},
    server::{ObjectSelectionChangeServerIntent, ServerIntents},
};

use crate::bevy_app::{
    component::BodyPartType,
    resource::{AppActiveBody, AppSelections, EngineState},
};

#[cfg(test)]
mod tests;

/// Convert selected object to attachable target. Only plane and face can be attachable target.
fn to_attachable_target(engine: &AppActiveBody, selections: &AppSelections) -> Option<PlaneRef> {
    let Some(body_id) = engine.0 else {
        return None;
    };

    if selections.len() != 1 {
        return None;
    }

    match selections[0] {
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
    active_body: Res<AppActiveBody>,
    mut selections: ResMut<AppSelections>,
    mut writer: MessageWriter<Correlation<Notifications>>,
    mut intent: MessageWriter<ServerIntents>,
    _commands: Commands,
) {
    let command = trigger.event();

    let Some(target) = to_attachable_target(&active_body, &selections) else {
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
    selections.clear();

    // reset selection
    intent.write(
        ObjectSelectionChangeServerIntent {
            selections: Vec::new(),
        }
        .into(),
    );
}
