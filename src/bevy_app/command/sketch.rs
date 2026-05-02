use bevy::prelude::*;
use cad_base::{
    body::{BodyPerspective, PlaneRef},
    id::SketchId,
    sketch::{AttachableTarget, SketchPerspective},
};
use immutable::Im;
use ui_event::{
    ObjectType, SketchCreationFailure,
    command::CreateSketchOnSelectedCommand,
    notification::{Notifications, SketchCreatedNotification, SketchCreationFailedNotification},
};

use crate::bevy_app::resource::EngineState;

/// Convert selected object to attachable target. Only plane and face can be attachable target.
fn to_attachable_target(selected: &Im<ObjectType>) -> Option<PlaneRef> {
    match **selected {
        ObjectType::Plane(plane_ref) => Some(plane_ref),
        ObjectType::Face(_) => None,
        ObjectType::Edge(_) => None,
        ObjectType::Point => None,
    }
}

/// A command to create sketch on the plane.
pub(super) fn on_create_sketch_on_plane(
    trigger: On<CreateSketchOnSelectedCommand>,
    mut engine: ResMut<EngineState>,
    mut writer: MessageWriter<Notifications>,
) {
    let command = trigger.event();

    let Some(target) = to_attachable_target(&command.selected) else {
        writer.write(
            SketchCreationFailedNotification {
                correlation_id: command.id.clone(),
                reason: SketchCreationFailure::TargetIsNotValid.into(),
            }
            .into(),
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
        SketchCreatedNotification {
            correlation_id: command.id.clone(),
            sketch_id: created_sketch.into(),
            name: sketch_name.into(),
        }
        .into(),
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
        CommandId, ObjectType, SketchCreationFailure,
        command::CreateSketchOnSelectedCommand,
        notification::{
            Notification, Notifications, SketchCreatedNotification,
            SketchCreationFailedNotification,
        },
    };

    use crate::bevy_app::resource::EngineState;

    use super::*;

    fn make_world() -> World {
        let mut world = World::new();
        world.init_resource::<Messages<Notifications>>();
        world.init_resource::<EngineState>();
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

        // Act
        world.trigger(CreateSketchOnSelectedCommand {
            id: CommandId::new(1).into(),
            selected: ObjectType::Plane(plane_ref).into(),
        });
        world.flush();

        // Assert
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        assert_eq!(notifications.len(), 1);
        let notif = notifications[0]
            .select_ref::<SketchCreatedNotification>()
            .unwrap();
        assert_eq!(*notif.correlation_id, CommandId::new(1));
        assert!(!notif.name.is_empty());
        Ok(())
    }

    #[test]
    fn writes_failure_notification_when_non_plane_selected() -> Result<()> {
        // Arrange
        let mut world = make_world();

        // Act
        world.trigger(CreateSketchOnSelectedCommand {
            id: CommandId::new(1).into(),
            selected: ObjectType::Point.into(),
        });
        world.flush();

        // Assert
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        assert_eq!(notifications.len(), 1);
        let notif = notifications[0]
            .select_ref::<SketchCreationFailedNotification>()
            .unwrap();
        assert_eq!(*notif.reason, SketchCreationFailure::TargetIsNotValid);
        Ok(())
    }
}
