use bevy::prelude::*;
use cad_base::{
    body::{BodyPerspective, PlaneRef},
    id::SketchId,
    plane::Plane,
    sketch::{AttachableTarget, SketchPerspective},
};
use ui_event::{
    Correlation, ObjectType, SketchCreationFailure,
    command::{
        ActivateSketchCommand, CreateSketchOnSelectedCommand, RequestGeometryCreationCommand,
    },
    notification::{
        Notifications, SketchActivatedNotification, SketchCreatedNotification,
        SketchCreationFailedNotification,
    },
};

use crate::bevy_app::{
    component::{BodyPartType, RequestedGeometryOperation, sketch::GeometryOperation},
    picking::PickingMessages,
    resource::{
        AppActiveBody, AppActiveSketch, AppCursorIcon, AppSelections, EngineState, IconType,
    },
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
    selections: ResMut<AppSelections>,
    mut writer: MessageWriter<Correlation<Notifications>>,
    mut picking: MessageWriter<PickingMessages>,
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

    // reset selection
    picking.write(PickingMessages::Clear);
}

/// A command to create sketch on the plane.
pub(super) fn on_activate_sketch(
    trigger: On<Correlation<ActivateSketchCommand>>,
    mut engine: ResMut<EngineState>,
    mut active_sketch: ResMut<AppActiveSketch>,
    mut writer: MessageWriter<Correlation<Notifications>>,
) {
    let command = trigger.event();

    let transaction = engine.0.begin();

    let sketch_id = *command.sketch_id;

    match transaction
        .read::<SketchPerspective>()
        .and_then(|p| p.get(&sketch_id))
    {
        Some(_) => {
            active_sketch.0 = Some(sketch_id);

            writer.write(
                trigger.event().correlate(
                    SketchActivatedNotification {
                        sketch_id: sketch_id.into(),
                    }
                    .into(),
                ),
            );
        }
        None => {
            tracing::warn!("Can not get sketch perspective");
        }
    }
}

/// A command to start creation process of a geometry requested
pub(super) fn on_request_geometry_creation_command(
    trigger: On<Correlation<RequestGeometryCreationCommand>>,
    mut commands: Commands,
    mut processing: Query<(
        Entity,
        &mut RequestedGeometryOperation,
        &mut GeometryOperation,
    )>,
    mut cursor: ResMut<AppCursorIcon>,
) {
    let command = trigger.event();

    // update old if exists
    if let Ok((_, mut typ, mut ope)) = processing.single_mut() {
        typ.0 = (*command.geometry).clone();
        *ope = GeometryOperation::from_geometry((*command.geometry).clone());
    } else {
        commands.spawn((
            RequestedGeometryOperation((*command.geometry).clone()),
            GeometryOperation::from_geometry((*command.geometry).clone()),
        ));
    }

    let icon = match *command.geometry {
        ui_event::SketchGeometryOperation::LineSegment => IconType::SketchLine,
        ui_event::SketchGeometryOperation::Rectangle => IconType::SketchRect,
    };

    cursor.0 = Some(icon);
}
