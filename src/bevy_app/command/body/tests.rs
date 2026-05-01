use bevy::asset::Assets;
use bevy::ecs::{message::Messages, system::RunSystemOnce, world::World};
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use cad_base::body::BodyPerspective;
use eyre::Result;
use pretty_assertions::assert_eq;
use ui_event::{
    CommandId,
    command::{CreateBodyCommand, SwitchActiveBodyCommand},
    notification::{
        BodyActivatedNotification, BodyCreatedNotification, Notification, Notifications,
    },
};

use crate::bevy_app::resource::{EngineAppState, EngineState};

use super::*;

fn make_world() -> World {
    let mut world = World::new();
    world.init_resource::<Messages<Notifications>>();
    world.init_resource::<EngineState>();
    world.init_resource::<EngineAppState>();
    world.init_resource::<Assets<Mesh>>();
    world.init_resource::<Assets<StandardMaterial>>();
    world.add_observer(on_create_body);
    world.add_observer(on_switch_active_body);
    world
}

#[test]
fn writes_notification_with_given_name_for_unique_name() {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(CreateBodyCommand {
        id: CommandId::new(1).into(),
        name: "body1".to_string().into(),
    });
    world.flush();

    // Assert
    let messages = world.resource::<Messages<Notifications>>();
    let mut cursor = messages.get_cursor();
    let notifications: Vec<_> = cursor.read(messages).collect();
    assert_eq!(notifications.len(), 1);
    let notif = notifications[0]
        .select_ref::<BodyCreatedNotification>()
        .unwrap();
    assert_eq!(*notif.name, "body1");
    let body_id = *notif.body_id;
    let app_state = world.resource::<EngineAppState>();
    assert_eq!(
        app_state.body_planes_map.get(&body_id).map(|v| v.len()),
        Some(6)
    );
}

#[test]
fn writes_notification_with_fallback_name_when_name_already_exists() -> Result<()> {
    // Arrange
    let mut world = make_world();
    {
        let mut engine = world.resource_mut::<EngineState>();
        let mut tx = engine.0.begin();
        let bodies = tx.modify::<BodyPerspective>().unwrap();
        let id = bodies.add_body();
        bodies.rename_body(&id, "body1").unwrap();
        tx.commit();
    }

    // Act
    world.trigger(CreateBodyCommand {
        id: CommandId::new(1).into(),
        name: "body1".to_string().into(),
    });
    world.flush();

    // Assert
    let messages = world.resource::<Messages<Notifications>>();
    let mut cursor = messages.get_cursor();
    let notifications: Vec<_> = cursor.read(messages).collect();
    let notif = notifications[0]
        .select_ref::<BodyCreatedNotification>()
        .unwrap();
    assert_eq!(*notif.name, "body1003");
    let body_id = *notif.body_id;
    let app_state = world.resource::<EngineAppState>();
    assert_eq!(
        app_state.body_planes_map.get(&body_id).map(|v| v.len()),
        Some(6)
    );
    Ok(())
}

#[test]
fn registered_planes_have_xy_yz_zx_axes_in_order() {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(CreateBodyCommand {
        id: CommandId::new(1).into(),
        name: "body1".to_string().into(),
    });
    world.flush();

    // Assert
    let body_id = {
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        *notifications[0]
            .select_ref::<BodyCreatedNotification>()
            .unwrap()
            .body_id
    };
    let entities = world
        .resource::<EngineAppState>()
        .body_planes_map
        .get(&body_id)
        .unwrap()
        .clone();
    let (ref_x, ref_y, ref_z) = {
        let mut engine = world.resource_mut::<EngineState>();
        let tx = engine.0.begin();
        let bodies = tx.read::<BodyPerspective>().unwrap();
        (
            bodies.as_x_plane_ref(&body_id).unwrap(),
            bodies.as_y_plane_ref(&body_id).unwrap(),
            bodies.as_z_plane_ref(&body_id).unwrap(),
        )
    };
    let axes: Vec<BodyBasePlane> = entities
        .iter()
        .map(|&e| *world.entity(e).get::<BodyBasePlane>().unwrap())
        .collect();
    assert_eq!(
        axes,
        vec![
            BodyBasePlane(ref_z),
            BodyBasePlane(ref_z),
            BodyBasePlane(ref_x),
            BodyBasePlane(ref_x),
            BodyBasePlane(ref_y),
            BodyBasePlane(ref_y),
        ]
    );
}

#[test]
fn registered_planes_are_placed_at_origin() {
    // Arrange
    let mut world = make_world();

    // Act
    world.trigger(CreateBodyCommand {
        id: CommandId::new(1).into(),
        name: "body1".to_string().into(),
    });
    world.flush();

    // Assert
    let body_id = {
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        *notifications[0]
            .select_ref::<BodyCreatedNotification>()
            .unwrap()
            .body_id
    };
    let entities = world
        .resource::<EngineAppState>()
        .body_planes_map
        .get(&body_id)
        .unwrap()
        .clone();
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
    world.trigger(CreateBodyCommand {
        id: CommandId::new(1).into(),
        name: "body1".to_string().into(),
    });
    world.flush();

    // Assert
    let body_id = {
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        *notifications[0]
            .select_ref::<BodyCreatedNotification>()
            .unwrap()
            .body_id
    };
    let entities = world
        .resource::<EngineAppState>()
        .body_planes_map
        .get(&body_id)
        .unwrap()
        .clone();
    for &e in &entities {
        let visibility = *world.entity(e).get::<Visibility>().unwrap();
        assert_eq!(visibility, Visibility::Hidden);
    }
}

fn create_body_and_get_plane_entities(
    world: &mut World,
    name: &str,
) -> (cad_base::id::BodyId, Vec<Entity>) {
    world.trigger(CreateBodyCommand {
        id: CommandId::new(1).into(),
        name: name.to_string().into(),
    });
    world.flush();
    let body_id = {
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        *notifications
            .iter()
            .filter_map(|n| n.select_ref::<BodyCreatedNotification>())
            .last()
            .unwrap()
            .body_id
    };
    let entities = world
        .resource::<EngineAppState>()
        .body_planes_map
        .get(&body_id)
        .unwrap()
        .clone();
    (body_id, entities)
}

#[test]
fn update_plane_visibilities_keeps_all_planes_hidden_when_no_active_body() {
    // Arrange
    let mut world = make_world();
    let (_, entities) = create_body_and_get_plane_entities(&mut world, "body1");

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
    let (body1_id, body1_entities) = create_body_and_get_plane_entities(&mut world, "body1");
    let (_, body2_entities) = create_body_and_get_plane_entities(&mut world, "body2");
    world.resource_mut::<EngineAppState>().active_body = Some(body1_id);

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
    let (body1_id, body1_entities) = create_body_and_get_plane_entities(&mut world, "body1");
    let (body2_id, body2_entities) = create_body_and_get_plane_entities(&mut world, "body2");
    world.resource_mut::<EngineAppState>().active_body = Some(body1_id);
    world.run_system_once(update_plane_visibilities).unwrap();

    // Act
    world.resource_mut::<EngineAppState>().active_body = Some(body2_id);
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
    world.trigger(SwitchActiveBodyCommand {
        id: CommandId::new(1).into(),
        body_id: body_id.into(),
    });
    world.flush();

    // Assert
    let messages = world.resource::<Messages<Notifications>>();
    let mut cursor = messages.get_cursor();
    let notifications: Vec<_> = cursor.read(messages).collect();
    assert_eq!(notifications.len(), 1);
    let notif = notifications[0]
        .select_ref::<BodyActivatedNotification>()
        .unwrap();
    assert_eq!(*notif.body_id, body_id);
    let app_state = world.resource::<EngineAppState>();
    assert_eq!(app_state.active_body, Some(body_id));
    Ok(())
}

#[test]
fn switch_active_body_returns_error_when_body_not_found() {
    // Arrange
    let mut world = make_world();
    let nonexistent_body_id = cad_base::id::BodyId::from(9999);

    // Act
    world.trigger(SwitchActiveBodyCommand {
        id: CommandId::new(1).into(),
        body_id: nonexistent_body_id.into(),
    });
    world.flush();

    // Assert
    let messages = world.resource::<Messages<Notifications>>();
    let mut cursor = messages.get_cursor();
    let notifications: Vec<_> = cursor.read(messages).collect();
    assert_eq!(notifications.len(), 0);
    let app_state = world.resource::<EngineAppState>();
    assert_eq!(app_state.active_body, None);
}
