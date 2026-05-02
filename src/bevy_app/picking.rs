use bevy::prelude::*;
use immutable::Im;

use crate::bevy_app::{component::ObjectType, resource::EngineAppState};

/// Component to handle materials pickable object.
///
/// This struct do not handle selection style, because selection style can not be defined by
/// point event only.
#[derive(Debug, Component, PartialEq, Eq, Clone)]
pub struct PickingMaterials {
    /// A material for normal state
    pub normal: Handle<StandardMaterial>,

    /// A material for hover state
    pub over: Handle<StandardMaterial>,
}

/// An event to change Active Plane
#[derive(Debug, Clone, PartialEq, Eq, Message)]
pub struct SelectObject {
    /// Entity id
    pub entity: Im<Entity>,
}

// common observers

/// A observer for [Pointer<Over>] event to update material to `over` of [PickingMaterials]
pub fn update_pointer_over(
    event: On<Pointer<Over>>,
    mut m_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    p_query: Query<&PickingMaterials, With<ObjectType>>,
) {
    if let Ok(mut material) = m_query.get_mut(event.event_target())
        && let Ok(picking) = p_query.get(event.event_target())
    {
        material.0 = picking.over.clone();
    }
}

/// A observer for [Pointer<Out>] event to update material to `normal` of [PickingMaterials]
pub fn update_pointer_out(
    event: On<Pointer<Out>>,
    mut m_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    p_query: Query<&PickingMaterials, With<ObjectType>>,
) {
    if let Ok(mut material) = m_query.get_mut(event.event_target())
        && let Ok(picking) = p_query.get(event.event_target())
    {
        material.0 = picking.normal.clone();
    }
}

/// A observer for [Pointer<Click>] event to send [SelectObject] message
pub fn update_pointer_click(event: On<Pointer<Click>>, mut commands: MessageWriter<SelectObject>) {
    commands.write(SelectObject {
        entity: event.event_target().into(),
    });
}

/// Update selections of something of body
pub fn update_toggling_selection(
    mut reader: MessageReader<SelectObject>,
    mut app_state: ResMut<EngineAppState>,
    query: Query<&ObjectType>,
) {
    for event in reader.read() {
        if let Some(p) = app_state
            .selections
            .iter()
            .position(|(e, _)| *e == *event.entity)
        {
            app_state.selections.remove(p);
        } else {
            let Ok(object_type) = query.get(*event.entity) else {
                tracing::warn!("Can not get object type from selectable entity");
                continue;
            };
            app_state
                .selections
                .push((*event.entity, object_type.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::{
        message::{MessageWriter, Messages},
        system::RunSystemOnce,
        world::World,
    };
    use pretty_assertions::assert_eq;

    use crate::bevy_app::{component::ObjectType, resource::EngineAppState};

    use super::*;

    fn make_world() -> World {
        let mut world = World::new();
        world.init_resource::<Messages<SelectObject>>();
        world.init_resource::<EngineAppState>();
        world
    }

    fn send_select_entity(world: &mut World, entity: bevy::ecs::entity::Entity) {
        world
            .run_system_once(move |mut writer: MessageWriter<SelectObject>| {
                writer.write(SelectObject {
                    entity: entity.into(),
                });
            })
            .unwrap();
        world.run_system_once(update_toggling_selection).unwrap();
    }

    #[test]
    fn toggling_selection_adds_entity_with_object_type_when_not_selected() {
        // Arrange
        let mut world = make_world();
        let entity = world.spawn(ObjectType::Point).id();

        // Act
        send_select_entity(&mut world, entity);

        // Assert
        let app_state = world.resource::<EngineAppState>();
        assert_eq!(app_state.selections, vec![(entity, ObjectType::Point)]);
    }

    #[test]
    fn toggling_selection_removes_entity_when_already_selected() {
        // Arrange
        let mut world = make_world();
        let entity = world.spawn(ObjectType::Point).id();
        world
            .resource_mut::<EngineAppState>()
            .selections
            .push((entity, ObjectType::Point));

        // Act
        send_select_entity(&mut world, entity);

        // Assert
        let app_state = world.resource::<EngineAppState>();
        assert_eq!(app_state.selections, vec![]);
    }

    #[test]
    fn toggling_selection_ignores_entity_without_object_type() {
        // Arrange
        let mut world = make_world();
        let entity = world.spawn_empty().id();

        // Act
        send_select_entity(&mut world, entity);

        // Assert
        let app_state = world.resource::<EngineAppState>();
        assert_eq!(app_state.selections, vec![]);
    }

    #[test]
    fn toggling_selection_accumulates_multiple_distinct_entities() {
        // Arrange
        let mut world = make_world();
        let entity1 = world.spawn(ObjectType::Point).id();
        let entity2 = world.spawn(ObjectType::SketchPoint).id();

        // Act — write both messages in one batch so a single reader cursor covers them
        world
            .run_system_once(move |mut writer: MessageWriter<SelectObject>| {
                writer.write(SelectObject {
                    entity: entity1.into(),
                });
                writer.write(SelectObject {
                    entity: entity2.into(),
                });
            })
            .unwrap();
        world.run_system_once(update_toggling_selection).unwrap();

        // Assert
        let app_state = world.resource::<EngineAppState>();
        assert_eq!(app_state.selections.len(), 2);
        assert!(app_state.selections.contains(&(entity1, ObjectType::Point)));
        assert!(
            app_state
                .selections
                .contains(&(entity2, ObjectType::SketchPoint))
        );
    }

    #[test]
    fn toggling_selection_removes_only_the_deselected_entity() {
        // Arrange
        let mut world = make_world();
        let entity1 = world.spawn(ObjectType::Point).id();
        let entity2 = world.spawn(ObjectType::SketchPoint).id();
        {
            let mut app_state = world.resource_mut::<EngineAppState>();
            app_state.selections.push((entity1, ObjectType::Point));
            app_state
                .selections
                .push((entity2, ObjectType::SketchPoint));
        }

        // Act
        send_select_entity(&mut world, entity1);

        // Assert
        let app_state = world.resource::<EngineAppState>();
        assert_eq!(
            app_state.selections,
            vec![(entity2, ObjectType::SketchPoint)]
        );
    }
}
