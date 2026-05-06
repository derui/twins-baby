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
    server::{ObjectSelectionChangeServerIntent, ServerIntent as _, ServerIntents},
};

use crate::bevy_app::{
    component::BodyPartType,
    resource::{AppActiveBody, AppSelections, EngineState},
};

use super::*;

fn make_world() -> World {
    let mut world = World::new();
    world.init_resource::<Messages<Correlation<Notifications>>>();
    world.init_resource::<Messages<ServerIntents>>();
    world.init_resource::<EngineState>();
    world.init_resource::<AppActiveBody>();
    world.init_resource::<AppSelections>();
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
        world.resource_mut::<AppActiveBody>().0 = Some(plane_ref.body_id());
        *world.resource_mut::<AppSelections>() =
            vec![(entity, BodyPartType(ObjectType::Plane(plane_ref)))].into();
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
fn clears_selection_via_server_intent_after_sketch_creation() -> Result<()> {
    // Arrange
    let mut world = make_world();
    let plane_ref = create_body_with_plane(&mut world);
    let entity = world.spawn(BodyPartType(ObjectType::Plane(plane_ref))).id();
    {
        world.resource_mut::<AppActiveBody>().0 = Some(plane_ref.body_id());
        *world.resource_mut::<AppSelections>() =
            vec![(entity, BodyPartType(ObjectType::Plane(plane_ref)))].into();
    }

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        CreateSketchOnSelectedCommand {},
    ));
    world.flush();

    // Assert
    let intents = world.resource::<Messages<ServerIntents>>();
    let mut cursor = intents.get_cursor();
    let received: Vec<_> = cursor.read(intents).collect();
    assert_eq!(received.len(), 1);
    let intent = received[0]
        .select_ref::<ObjectSelectionChangeServerIntent>()
        .unwrap();
    assert_eq!(intent.selections, Vec::new());
    Ok(())
}

#[test]
fn does_not_send_server_intent_when_creation_fails() -> Result<()> {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        CreateSketchOnSelectedCommand {},
    ));
    world.flush();

    // Assert
    let intents = world.resource::<Messages<ServerIntents>>();
    let mut cursor = intents.get_cursor();
    let received: Vec<_> = cursor.read(intents).collect();
    assert_eq!(received.len(), 0);
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
