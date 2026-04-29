use bevy::asset::{Assets, Handle};
use bevy::camera::visibility::{RenderLayers, Visibility};
use bevy::color::palettes::tailwind::CYAN_500;
use bevy::color::{Alpha, Color};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EntityEvent as _;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::ecs::{error::BevyError, message::MessageWriter, observer::On};
use bevy::math::Dir3;
use bevy::math::primitives::Plane3d;
use bevy::mesh::{Mesh, Mesh3d, Meshable};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::picking::events::{Out, Over, Pointer};
use bevy::prelude::ResMut;
use bevy::transform::components::Transform;
use cad_base::body::{BodyPerspective, PlaneRef};
use cad_base::id::BodyId;
use ui_event::command::SwitchActiveBodyCommand;
use ui_event::{
    command::CreateBodyCommand,
    notification::{BodyActivatedNotification, BodyCreatedNotification, Notifications},
};

use crate::bevy_app::camera::CAMERA_3D_LAYER;
use crate::bevy_app::resource::{EngineAppState, EngineState};

// components

/// A marker compoment
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct BodyBasePlane(pub PlaneRef);

/// Return a obverver for [Pointer<Over>] event to update plane material to `material_over`
fn update_plane_over(
    material_over: Handle<StandardMaterial>,
) -> impl Fn(On<Pointer<Over>>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = material_over.clone()
        }
    }
}

/// Return a obverver for [Pointer<Out>] event to update plane material to `material_normal`
fn update_plane_out(
    material_normal: Handle<StandardMaterial>,
) -> impl Fn(On<Pointer<Out>>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = material_normal.clone()
        }
    }
}

/// Register 3 planes for the body.
///
/// # Return
/// The entities of planes. The order is XY, YZ, ZX plane.
fn register_body_base_planes(
    bodies: &BodyPerspective,
    body_id: &BodyId,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> eyre::Result<Vec<Entity>> {
    // all sizes are 1 = 1m
    let mut entities = Vec::new();
    let mat_normal = materials.add(Color::from(CYAN_500).with_alpha(0.3));
    let mat_over = materials.add(Color::from(CYAN_500).with_alpha(0.5));
    let ref_x = bodies
        .as_x_plane_ref(body_id)
        .ok_or(color_eyre::eyre::eyre!("Should get X ref"))?;
    let ref_y = bodies
        .as_y_plane_ref(body_id)
        .ok_or(color_eyre::eyre::eyre!("Should get Y ref"))?;
    let ref_z = bodies
        .as_z_plane_ref(body_id)
        .ok_or(color_eyre::eyre::eyre!("Should get Z ref"))?;

    // normal vector will use for culling, this simple fix to avoid disappearing of planes
    for dir in [Dir3::Z, Dir3::NEG_Z] {
        let plane = meshes.add(Plane3d::default().mesh().size(10.0, 10.0).normal(dir));
        let mut entity = commands.spawn((
            Mesh3d(plane),
            MeshMaterial3d(mat_normal.clone()),
            Transform::from_xyz(0., 0., 0.),
            RenderLayers::layer(CAMERA_3D_LAYER),
            Visibility::Hidden,
            BodyBasePlane(ref_z),
        ));
        entity.observe(update_plane_over(mat_over.clone()));
        entity.observe(update_plane_out(mat_normal.clone()));
        entities.push(entity.id());
    }

    for dir in [Dir3::X, Dir3::NEG_X] {
        let plane = meshes.add(Plane3d::default().mesh().size(10.0, 10.0).normal(dir));
        let mut entity = commands.spawn((
            Mesh3d(plane),
            MeshMaterial3d(mat_normal.clone()),
            Transform::from_xyz(0., 0., 0.),
            RenderLayers::layer(CAMERA_3D_LAYER),
            Visibility::Hidden,
            BodyBasePlane(ref_x),
        ));
        entity.observe(update_plane_over(mat_over.clone()));
        entity.observe(update_plane_out(mat_normal.clone()));
        entities.push(entity.id());
    }

    for dir in [Dir3::Y, Dir3::NEG_Y] {
        let plane = meshes.add(Plane3d::default().mesh().size(10.0, 10.0).normal(dir));
        let mut entity = commands.spawn((
            Mesh3d(plane),
            MeshMaterial3d(mat_normal.clone()),
            Transform::from_xyz(0., 0., 0.),
            RenderLayers::layer(CAMERA_3D_LAYER),
            Visibility::Hidden,
            BodyBasePlane(ref_y),
        ));
        entity.observe(update_plane_over(mat_over.clone()));
        entity.observe(update_plane_out(mat_normal.clone()));
        entities.push(entity.id());
    }

    Ok(entities)
}

pub(super) fn on_create_body(
    trigger: On<CreateBodyCommand>,
    mut engine: ResMut<EngineState>,
    mut app_state: ResMut<EngineAppState>,
    mut writer: MessageWriter<Notifications>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result<(), BevyError> {
    let command = trigger.event();
    let mut transaction = engine.0.begin();

    let Some(body) = transaction.modify::<BodyPerspective>() else {
        return Err(color_eyre::eyre::eyre!("Can not get body perspective").into());
    };

    let body_id = body.add_body();
    let mut name = (*command.name).clone();
    if body.rename_body(&body_id, &name).is_err() {
        let count = body.bodies().count();
        name = format!("{}{:03}", &name, count + 1);
        // TODO should fallback notification when get error.
        body.rename_body(&body_id, &name)?;
    }

    writer.write(
        BodyCreatedNotification {
            correlation_id: command.id.clone(),
            body_id: body_id.into(),
            name: name.into(),
        }
        .into(),
    );

    if let Ok(entities) =
        register_body_base_planes(body, &body_id, &mut commands, &mut meshes, &mut materials)
    {
        app_state.body_planes_map.insert(body_id, entities);
    }

    transaction.commit();

    Ok(())
}

/// A command handler of [SwitchActiveBodyCommand]
pub(super) fn on_switch_active_body(
    trigger: On<SwitchActiveBodyCommand>,
    mut engine: ResMut<EngineState>,
    mut app_state: ResMut<EngineAppState>,
    mut writer: MessageWriter<Notifications>,
) {
    let command = trigger.event();
    let transaction = engine.0.begin();

    let Some(body) = transaction.read::<BodyPerspective>() else {
        tracing::info!("Can not get body perspective");
        return;
    };

    if body.get(&command.body_id).is_some() {
        app_state.active_body = Some(*command.body_id.clone());

        writer.write(
            BodyActivatedNotification {
                correlation_id: command.id.clone(),
                body_id: command.body_id.clone(),
            }
            .into(),
        );
    } else {
        // This case occurs sometimes. Do not do anything in this
    }
}

/// Update all plane visibilities of the app
pub(super) fn update_plane_visibilities(
    mut commands: Commands,
    app_state: Res<EngineAppState>,
    q_planes: Query<Entity, With<BodyBasePlane>>,
) {
    // Make all entities hidden
    for plane in q_planes {
        commands.entity(plane).insert(Visibility::Hidden);
    }

    // When app has active body, active the planes
    let Some(body_id) = app_state.active_body else {
        return;
    };

    for &plane in app_state
        .body_planes_map
        .get(&body_id)
        .unwrap_or(&Vec::<Entity>::new())
    {
        commands.entity(plane).insert(Visibility::Visible);
    }
}

#[cfg(test)]
mod tests {
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
}
