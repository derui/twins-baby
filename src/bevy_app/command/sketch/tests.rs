use bevy::ecs::{message::Messages, world::World};
use cad_base::body::BodyPerspective;
use eyre::Result;
use pretty_assertions::assert_eq;
use ui_event::{
    CommandId, Correlation, ObjectType, SketchCreationFailure,
    command::CreateSketchOnSelectedCommand,
    notification::{
        Notification, Notifications, SketchCreatedNotification, SketchCreationFailedNotification,
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
