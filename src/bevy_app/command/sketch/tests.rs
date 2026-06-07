use bevy::ecs::{message::Messages, world::World};
use cad_base::{
    body::BodyPerspective,
    id::SketchId,
    sketch::{AttachableTarget, SketchPerspective},
};
use eyre::Result;
use pretty_assertions::assert_eq;
use ui_event::{
    CommandId, Correlation, ObjectType, SketchCreationFailure, SketchGeometryOperation,
    command::{
        ActivateSketchCommand, CreateSketchOnSelectedCommand, RequestGeometryCreationCommand,
    },
    notification::{
        Notification, Notifications, SketchActivatedNotification, SketchCreatedNotification,
        SketchCreationFailedNotification,
    },
    server::ServerIntents,
};

use crate::bevy_app::{
    component::{BodyPartType, sketch::GeometryOperationStep},
    picking::PickingMessages,
    resource::{AppActiveBody, AppActiveSketch, AppSelections, EngineState},
};

use super::*;

fn make_world() -> World {
    let mut world = World::new();
    world.init_resource::<Messages<Correlation<Notifications>>>();
    world.init_resource::<Messages<ServerIntents>>();
    world.init_resource::<Messages<PickingMessages>>();
    world.init_resource::<EngineState>();
    world.init_resource::<AppActiveBody>();
    world.init_resource::<AppActiveSketch>();
    world.init_resource::<AppSelections>();
    world.add_observer(on_create_sketch_on_plane);
    world.add_observer(on_activate_sketch);
    world.add_observer(on_request_geometry_creation_command);
    world
}

fn create_sketch(world: &mut World, plane_ref: cad_base::body::PlaneRef) -> SketchId {
    let mut engine = world.resource_mut::<EngineState>();
    let mut tx = engine.0.begin();
    let sketch_id;
    {
        let sketch_p = tx.modify::<SketchPerspective>().unwrap();
        sketch_id = sketch_p.add_sketch(&AttachableTarget::Plane(plane_ref));
    }
    {
        let body_p = tx.modify::<BodyPerspective>().unwrap();
        body_p
            .get_mut(&plane_ref.body_id())
            .unwrap()
            .add_sketch(&sketch_id);
    }
    tx.commit();
    sketch_id
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
fn sends_picking_clear_message_after_sketch_creation() -> Result<()> {
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
    let messages = world.resource::<Messages<PickingMessages>>();
    let mut cursor = messages.get_cursor();
    let received: Vec<_> = cursor.read(messages).collect();
    assert_eq!(received.len(), 1);
    assert_eq!(*received[0], PickingMessages::Clear);
    Ok(())
}

#[test]
fn does_not_send_picking_clear_when_creation_fails() -> Result<()> {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        CreateSketchOnSelectedCommand {},
    ));
    world.flush();

    // Assert
    let messages = world.resource::<Messages<PickingMessages>>();
    let mut cursor = messages.get_cursor();
    let received: Vec<_> = cursor.read(messages).collect();
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

#[test]
fn writes_sketch_activated_notification_when_sketch_exists() -> Result<()> {
    // Arrange
    let mut world = make_world();
    let plane_ref = create_body_with_plane(&mut world);
    let sketch_id = create_sketch(&mut world, plane_ref);

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        ActivateSketchCommand {
            sketch_id: sketch_id.into(),
        },
    ));
    world.flush();

    // Assert
    let messages = world.resource::<Messages<Correlation<Notifications>>>();
    let mut cursor = messages.get_cursor();
    let notifications: Vec<_> = cursor.read(messages).collect();
    assert_eq!(notifications.len(), 1);
    let notif = notifications[0]
        .data
        .select_ref::<SketchActivatedNotification>()
        .unwrap();
    assert_eq!(*notif.sketch_id, sketch_id);
    Ok(())
}

#[test]
fn sets_active_sketch_when_sketch_exists() -> Result<()> {
    // Arrange
    let mut world = make_world();
    let plane_ref = create_body_with_plane(&mut world);
    let sketch_id = create_sketch(&mut world, plane_ref);

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        ActivateSketchCommand {
            sketch_id: sketch_id.into(),
        },
    ));
    world.flush();

    // Assert
    let active = world.resource::<AppActiveSketch>();
    assert_eq!(active.0, Some(sketch_id));
    Ok(())
}

#[test]
fn does_not_write_notification_when_sketch_not_found() -> Result<()> {
    // Arrange
    let mut world = make_world();
    let nonexistent_sketch_id = SketchId::from(99999);

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        ActivateSketchCommand {
            sketch_id: nonexistent_sketch_id.into(),
        },
    ));
    world.flush();

    // Assert
    let messages = world.resource::<Messages<Correlation<Notifications>>>();
    let mut cursor = messages.get_cursor();
    let notifications: Vec<_> = cursor.read(messages).collect();
    assert_eq!(notifications.len(), 0);
    Ok(())
}

#[test]
fn spawns_geometry_operation_entity_when_none_exists() -> Result<()> {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        RequestGeometryCreationCommand {
            geometry: SketchGeometryOperation::LineSegment.into(),
        },
    ));
    world.flush();

    // Assert
    let mut query = world.query::<(&RequestedGeometryOperation, &GeometryOperation)>();
    let entities: Vec<_> = query.iter(&world).collect();
    assert_eq!(entities.len(), 1);
    assert_eq!(entities[0].0.0, SketchGeometryOperation::LineSegment);
    assert_eq!(
        entities[0].1.steps.as_slice(),
        &[GeometryOperationStep::Point, GeometryOperationStep::Point]
    );
    Ok(())
}

#[test]
fn updates_existing_entity_when_geometry_operation_already_exists() -> Result<()> {
    // Arrange
    let mut world = make_world();
    world.trigger(Correlation::new(
        CommandId::new(1),
        RequestGeometryCreationCommand {
            geometry: SketchGeometryOperation::LineSegment.into(),
        },
    ));
    world.flush();

    // Act
    world.trigger(Correlation::new(
        CommandId::new(2),
        RequestGeometryCreationCommand {
            geometry: SketchGeometryOperation::Rectangle.into(),
        },
    ));
    world.flush();

    // Assert
    let mut query = world.query::<(&RequestedGeometryOperation, &GeometryOperation)>();
    let entities: Vec<_> = query.iter(&world).collect();
    assert_eq!(entities.len(), 1);
    assert_eq!(entities[0].0.0, SketchGeometryOperation::Rectangle);
    Ok(())
}

#[test]
fn spawns_rectangle_operation_with_correct_steps() -> Result<()> {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        RequestGeometryCreationCommand {
            geometry: SketchGeometryOperation::Rectangle.into(),
        },
    ));
    world.flush();

    // Assert
    let mut query = world.query::<(&RequestedGeometryOperation, &GeometryOperation)>();
    let entities: Vec<_> = query.iter(&world).collect();
    assert_eq!(entities.len(), 1);
    assert_eq!(entities[0].0.0, SketchGeometryOperation::Rectangle);
    assert_eq!(
        entities[0].1.steps.as_slice(),
        &[GeometryOperationStep::Point, GeometryOperationStep::Point]
    );
    Ok(())
}
