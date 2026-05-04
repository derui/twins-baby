use bevy::prelude::*;
use cad_base::{
    body::{BodyPerspective, PlaneRef},
    id::SketchId,
    sketch::{AttachableTarget, SketchPerspective},
};
use ui_event::{
    Correlation, ObjectType, SketchCreationFailure,
    command::CreateSketchOnSelectedCommand,
    notification::{Notifications, SketchCreatedNotification, SketchCreationFailedNotification},
};

use crate::bevy_app::{
    component::BodyPartType,
    resource::{EngineAppState, EngineState},
};

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

    if let Some(body_p) = transaction.modify::<BodyPerspective>()
        && let Some(body) = body_p.get_mut(&target.body_id())
    {
        body.add_sketch(&created_sketch);
    } else {
        tracing::warn!("Can not get body");
        return;
    };

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

    transaction.commit();
}

#[cfg(test)]
mod tests {
    use bevy::ecs::{message::Messages, world::World};
    use cad_base::body::BodyPerspective;
    use eyre::Result;
    use pretty_assertions::assert_eq;
    use ui_event::{
        CommandId, Correlation, ObjectType, SketchCreationFailure,
        command::CreateSketchOnSelectedCommand,
        notification::{
            Notification, Notifications, SketchCreatedNotification,
            SketchCreationFailedNotification,
        },
    };

    use crate::bevy_app::{
        component::BodyPartType,
        resource::{EngineAppState, EngineState},
    };

    use super::*;

    fn make_world() -> World {
        let mut world = World::new();
        world.init_resource::<Messages<Correlation<Notifications>>>();
        world.init_resource::<EngineState>();
        world.init_resource::<EngineAppState>();
        world.add_observer(on_create_sketch_on_plane);
        world
    }

    fn create_body_with_plane(world: &mut World) -> cad_base::body::PlaneRef {
        let mut engine = world.resource_mut::<EngineState>();
        let mut tx = engine.0.begin();
        let bodies = tx.modify::<BodyPerspective>().unwrap();
        let body_id = bodies.add_body();
        let plane_ref = bodies.to_x_plane_ref(&body_id).unwrap();
        tx.commit();
        plane_ref
    }

    #[test]
    fn writes_sketch_created_notification_when_plane_selected() -> Result<()> {
        // Arrange
        let mut world = make_world();
        let plane_ref = create_body_with_plane(&mut world);
        let entity = world.spawn(BodyPartType(ObjectType::Plane(plane_ref))).id();
        {
            let mut app_state = world.resource_mut::<EngineAppState>();
            app_state.active_body = Some(plane_ref.body_id());
            app_state.selections = vec![(entity, BodyPartType(ObjectType::Plane(plane_ref)))];
        }

        // Act
        world.trigger(Correlation::new(
            CommandId::new(1),
            CreateSketchOnSelectedCommand {},
        ));
        world.flush();

        // Assert
        let messages = world.resource::<Messages<Correlation<Notifications>>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        assert_eq!(notifications.len(), 1);
        let notif = notifications[0]
            .data
            .select_ref::<SketchCreatedNotification>()
            .unwrap();
        assert_eq!(*notifications[0].id, CommandId::new(1));
        assert!(!notif.name.is_empty());
        assert_eq!(*notif.body_id, plane_ref.body_id());
        Ok(())
    }

    #[test]
    fn writes_failure_notification_when_non_plane_selected() -> Result<()> {
        // Arrange
        let mut world = make_world();

        // Act
        world.trigger(Correlation::new(
            CommandId::new(1),
            CreateSketchOnSelectedCommand {},
        ));
        world.flush();

        // Assert
        let messages = world.resource::<Messages<Correlation<Notifications>>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        assert_eq!(notifications.len(), 1);
        let notif = notifications[0]
            .data
            .select_ref::<SketchCreationFailedNotification>()
            .unwrap();
        assert_eq!(*notif.reason, SketchCreationFailure::TargetIsNotValid);
        Ok(())
    }
}
