use bevy::asset::Assets;
use bevy::ecs::{entity::Entity, message::Messages, system::RunSystemOnce, world::World};
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use cad_base::body::BodyPerspective;
use eyre::Result;
use pretty_assertions::assert_eq;
use ui_event::{
    CommandId, Correlation,
    command::{CreateBodyCommand, SwitchActiveBodyCommand},
    notification::{
        BodyActivatedNotification, BodyCreatedNotification, Notification, Notifications,
    },
};

use crate::bevy_app::resource::{AppActiveBody, EngineState};

use super::*;

fn make_world() -> World {
    let mut world = World::new();
    world.init_resource::<Messages<Correlation<Notifications>>>();
    world.init_resource::<EngineState>();
    world.init_resource::<AppActiveBody>();
    world.init_resource::<Assets<Mesh>>();
    world.init_resource::<Assets<StandardMaterial>>();
    world.add_observer(on_create_body);
    world.add_observer(on_switch_active_body);
    world
}

fn get_plane_entities_for_body(world: &mut World, body_id: cad_base::id::BodyId) -> Vec<Entity> {
    let mut query = world.query::<(Entity, &BodyBasePlane)>();
    query
        .iter(world)
        .filter(|(_, plane)| (*plane).body_id() == body_id)
        .map(|(e, _)| e)
        .collect()
}

#[test]
fn writes_notification_with_given_name_for_unique_name() {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(Correlation::new(CommandId::new(1), CreateBodyCommand {}));
    world.flush();

    // Assert
    let body_id = {
        let messages = world.resource::<Messages<Correlation<Notifications>>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        assert_eq!(notifications.len(), 1);
        let notif = notifications[0]
            .data
            .select_ref::<BodyCreatedNotification>()
            .unwrap();
        assert!(!notif.name.is_empty());
        *notif.body_id
    };
    let plane_count = get_plane_entities_for_body(&mut world, body_id).len();
    assert_eq!(plane_count, 6);
}

#[test]
fn writes_notification_with_body_created() -> Result<()> {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(Correlation::new(CommandId::new(1), CreateBodyCommand {}));
    world.flush();

    // Assert
    let body_id = {
        let messages = world.resource::<Messages<Correlation<Notifications>>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        let notif = notifications[0]
            .data
            .select_ref::<BodyCreatedNotification>()
            .unwrap();
        assert!(!notif.name.is_empty());
        *notif.body_id
    };
    let plane_count = get_plane_entities_for_body(&mut world, body_id).len();
    assert_eq!(plane_count, 6);
    Ok(())
}

#[test]
fn registered_planes_have_xy_yz_zx_axes_in_order() {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(Correlation::new(CommandId::new(1), CreateBodyCommand {}));
    world.flush();

    // Assert
    let body_id = {
        let messages = world.resource::<Messages<Correlation<Notifications>>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        *notifications[0]
            .data
            .select_ref::<BodyCreatedNotification>()
            .unwrap()
            .body_id
    };
    let entities = get_plane_entities_for_body(&mut world, body_id);
    let (ref_x, ref_y, ref_z) = {
        let mut engine = world.resource_mut::<EngineState>();
        let tx = engine.0.begin();
        let bodies = tx.read::<BodyPerspective>().unwrap();
        (
            bodies.to_x_plane_ref(&body_id).unwrap(),
            bodies.to_y_plane_ref(&body_id).unwrap(),
            bodies.to_z_plane_ref(&body_id).unwrap(),
        )
    };
    let axes: Vec<BodyBasePlane> = entities
        .iter()
        .map(|&e| *world.entity(e).get::<BodyBasePlane>().unwrap())
        .collect();
    assert_eq!(
        axes,
        vec![
            Into::<BodyBasePlane>::into(ref_z),
            Into::<BodyBasePlane>::into(ref_z),
            Into::<BodyBasePlane>::into(ref_x),
            Into::<BodyBasePlane>::into(ref_x),
            Into::<BodyBasePlane>::into(ref_y),
            Into::<BodyBasePlane>::into(ref_y),
        ]
    );
}

#[test]
fn registered_planes_are_placed_at_origin() {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(Correlation::new(CommandId::new(1), CreateBodyCommand {}));
    world.flush();

    // Assert
    let body_id = {
        let messages = world.resource::<Messages<Correlation<Notifications>>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        *notifications[0]
            .data
            .select_ref::<BodyCreatedNotification>()
            .unwrap()
            .body_id
    };
    let entities = get_plane_entities_for_body(&mut world, body_id);
    for &e in &entities {
        let transform = *world.entity(e).get::<Transform>().unwrap();
        assert_eq!(transform.translation, bevy::math::Vec3::ZERO);
    }
}

#[test]
fn registered_planes_are_hidden_on_creation() {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(Correlation::new(CommandId::new(1), CreateBodyCommand {}));
    world.flush();

    // Assert
    let body_id = {
        let messages = world.resource::<Messages<Correlation<Notifications>>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        *notifications[0]
            .data
            .select_ref::<BodyCreatedNotification>()
            .unwrap()
            .body_id
    };
    let entities = get_plane_entities_for_body(&mut world, body_id);
    for &e in &entities {
        let visibility = *world.entity(e).get::<Visibility>().unwrap();
        assert_eq!(visibility, Visibility::Hidden);
    }
}

fn create_body_and_get_plane_entities(world: &mut World) -> (cad_base::id::BodyId, Vec<Entity>) {
    world.trigger(Correlation::new(CommandId::new(1), CreateBodyCommand {}));
    world.flush();
    let body_id = {
        let messages = world.resource::<Messages<Correlation<Notifications>>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        *notifications
            .iter()
            .filter_map(|n| n.data.select_ref::<BodyCreatedNotification>())
            .last()
            .unwrap()
            .body_id
    };
    let entities = get_plane_entities_for_body(world, body_id);
    (body_id, entities)
}

#[test]
fn update_plane_visibilities_keeps_all_planes_hidden_when_no_active_body() {
    // Arrange
    let mut world = make_world();
    let (_, entities) = create_body_and_get_plane_entities(&mut world);

    // Act
    world.run_system_once(update_plane_visibilities).unwrap();

    // Assert
    for &e in &entities {
        assert_eq!(
            world.entity(e).get::<Visibility>().copied(),
            Some(Visibility::Hidden)
        );
    }
}

#[test]
fn update_plane_visibilities_shows_active_body_planes_and_hides_others() {
    // Arrange
    let mut world = make_world();
    let (body1_id, body1_entities) = create_body_and_get_plane_entities(&mut world);
    let (_, body2_entities) = create_body_and_get_plane_entities(&mut world);
    world.resource_mut::<AppActiveBody>().0 = Some(body1_id);

    // Act
    world.run_system_once(update_plane_visibilities).unwrap();

    // Assert
    for &e in &body1_entities {
        assert_eq!(
            world.entity(e).get::<Visibility>().copied(),
            Some(Visibility::Visible)
        );
    }
    for &e in &body2_entities {
        assert_eq!(
            world.entity(e).get::<Visibility>().copied(),
            Some(Visibility::Hidden)
        );
    }
}

#[test]
fn update_plane_visibilities_switches_visibility_when_active_body_changes() {
    // Arrange
    let mut world = make_world();
    let (body1_id, body1_entities) = create_body_and_get_plane_entities(&mut world);
    let (body2_id, body2_entities) = create_body_and_get_plane_entities(&mut world);
    world.resource_mut::<AppActiveBody>().0 = Some(body1_id);
    world.run_system_once(update_plane_visibilities).unwrap();

    // Act
    world.resource_mut::<AppActiveBody>().0 = Some(body2_id);
    world.run_system_once(update_plane_visibilities).unwrap();

    // Assert
    for &e in &body1_entities {
        assert_eq!(
            world.entity(e).get::<Visibility>().copied(),
            Some(Visibility::Hidden)
        );
    }
    for &e in &body2_entities {
        assert_eq!(
            world.entity(e).get::<Visibility>().copied(),
            Some(Visibility::Visible)
        );
    }
}

#[test]
fn switch_active_body_writes_notification_and_updates_app_state() -> Result<()> {
    // Arrange
    let mut world = make_world();
    let body_id = {
        let mut engine = world.resource_mut::<EngineState>();
        let mut tx = engine.0.begin();
        let bodies = tx.modify::<BodyPerspective>().unwrap();
        let id = bodies.add_body();
        bodies.rename_body(&id, "body1").unwrap();
        tx.commit();
        id
    };

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        SwitchActiveBodyCommand {
            body_id: body_id.into(),
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
        .select_ref::<BodyActivatedNotification>()
        .unwrap();
    assert_eq!(*notif.body_id, body_id);
    let app_active_body = world.resource::<AppActiveBody>();
    assert_eq!(app_active_body.0, Some(body_id));
    Ok(())
}

#[test]
fn switch_active_body_returns_error_when_body_not_found() {
    // Arrange
    let mut world = make_world();
    let nonexistent_body_id = cad_base::id::BodyId::from(9999);

    // Act
    world.trigger(Correlation::new(
        CommandId::new(1),
        SwitchActiveBodyCommand {
            body_id: nonexistent_body_id.into(),
        },
    ));
    world.flush();

    // Assert
    let messages = world.resource::<Messages<Correlation<Notifications>>>();
    let mut cursor = messages.get_cursor();
    let notifications: Vec<_> = cursor.read(messages).collect();
    assert_eq!(notifications.len(), 0);
    let app_active_body = world.resource::<AppActiveBody>();
    assert_eq!(app_active_body.0, None);
}
