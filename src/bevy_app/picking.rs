use bevy::prelude::*;
use immutable::Im;
use ui_event::server::{ObjectSelectionChangeServerIntent, ServerIntents};

use crate::bevy_app::{component::BodyPartType, resource::AppSelections};

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
    p_query: Query<&PickingMaterials, With<BodyPartType>>,
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
    p_query: Query<&PickingMaterials, With<BodyPartType>>,
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
    mut selections: ResMut<AppSelections>,
    query: Query<&BodyPartType>,
    mut writer: MessageWriter<ServerIntents>,
) {
    for event in reader.read() {
        if selections.contains(*event.entity) {
            selections.remove(*event.entity);
        } else {
            let Ok(object_type) = query.get(*event.entity) else {
                tracing::warn!("Can not get object type from selectable entity");
                continue;
            };
            selections.insert(*event.entity, object_type.clone());
        }
    }

    writer.write(
        ObjectSelectionChangeServerIntent {
            selections: (*selections).iter().cloned().map(|v| v.1.0).collect(),
        }
        .into(),
    );
}

#[cfg(test)]
mod tests {
    use bevy::ecs::{
        message::{MessageReader, MessageWriter, Messages},
        system::RunSystemOnce,
        world::World,
    };
    use cad_base::id::EdgeId;
    use pretty_assertions::assert_eq;
    use ui_event::{
        ObjectType,
        server::{ObjectSelectionChangeServerIntent, ServerIntent, ServerIntents},
    };

    use crate::bevy_app::{component::BodyPartType, resource::AppSelections};

    use super::*;

    #[derive(bevy::ecs::resource::Resource, Default)]
    struct IntentCapture(Vec<ServerIntents>);

    fn capture_intents_system(
        mut reader: MessageReader<ServerIntents>,
        mut capture: ResMut<IntentCapture>,
    ) {
        capture.0.clear();
        for intent in reader.read() {
            capture.0.push(intent.clone());
        }
    }

    fn make_world() -> World {
        let mut world = World::new();
        world.init_resource::<Messages<SelectObject>>();
        world.init_resource::<Messages<ServerIntents>>();
        world.init_resource::<AppSelections>();
        world.init_resource::<IntentCapture>();
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
        world.run_system_once(capture_intents_system).unwrap();
    }

    fn point_type() -> BodyPartType {
        BodyPartType(ObjectType::Point)
    }

    fn edge_type() -> BodyPartType {
        BodyPartType(ObjectType::Edge(EdgeId::new(1)))
    }

    fn captured_selection_intent(world: &World) -> Option<ObjectSelectionChangeServerIntent> {
        world
            .resource::<IntentCapture>()
            .0
            .iter()
            .find_map(|i| i.select_ref::<ObjectSelectionChangeServerIntent>())
            .cloned()
    }

    #[test]
    fn toggling_selection_adds_entity_with_object_type_when_not_selected() {
        // Arrange
        let mut world = make_world();
        let entity = world.spawn(point_type()).id();

        // Act
        send_select_entity(&mut world, entity);

        // Assert
        let selections = world.resource::<AppSelections>();
        assert_eq!(**selections, vec![(entity, point_type())]);
    }

    #[test]
    fn toggling_selection_removes_entity_when_already_selected() {
        // Arrange
        let mut world = make_world();
        let entity = world.spawn(point_type()).id();
        world
            .resource_mut::<AppSelections>()
            .insert(entity, point_type());

        // Act
        send_select_entity(&mut world, entity);

        // Assert
        let selections = world.resource::<AppSelections>();
        assert_eq!(**selections, vec![]);
    }

    #[test]
    fn toggling_selection_ignores_entity_without_object_type() {
        // Arrange
        let mut world = make_world();
        let entity = world.spawn_empty().id();

        // Act
        send_select_entity(&mut world, entity);

        // Assert
        let selections = world.resource::<AppSelections>();
        assert_eq!(**selections, vec![]);
    }

    #[test]
    fn toggling_selection_accumulates_multiple_distinct_entities() {
        // Arrange
        let mut world = make_world();
        let entity1 = world.spawn(point_type()).id();
        let entity2 = world.spawn(edge_type()).id();

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
        let selections = world.resource::<AppSelections>();
        assert_eq!((**selections).len(), 2);
        assert!((**selections).contains(&(entity1, point_type())));
        assert!((**selections).contains(&(entity2, edge_type())));
    }

    #[test]
    fn toggling_selection_removes_only_the_deselected_entity() {
        // Arrange
        let mut world = make_world();
        let entity1 = world.spawn(point_type()).id();
        let entity2 = world.spawn(edge_type()).id();
        {
            let mut selections = world.resource_mut::<AppSelections>();
            selections.insert(entity1, point_type());
            selections.insert(entity2, edge_type());
        }

        // Act
        send_select_entity(&mut world, entity1);

        // Assert
        let selections = world.resource::<AppSelections>();
        assert_eq!(**selections, vec![(entity2, edge_type())]);
    }

    #[test]
    fn sends_intent_with_selected_object_types_when_entity_selected() {
        // Arrange
        let mut world = make_world();
        let entity = world.spawn(point_type()).id();

        // Act
        send_select_entity(&mut world, entity);

        // Assert
        let intent = captured_selection_intent(&world).expect("intent should be sent");
        assert_eq!(intent.selections, vec![ObjectType::Point]);
    }

    #[test]
    fn sends_intent_with_empty_selections_when_entity_deselected() {
        // Arrange
        let mut world = make_world();
        let entity = world.spawn(point_type()).id();
        world
            .resource_mut::<AppSelections>()
            .insert(entity, point_type());

        // Act
        send_select_entity(&mut world, entity);

        // Assert
        let intent = captured_selection_intent(&world).expect("intent should be sent");
        assert_eq!(intent.selections, vec![]);
    }

    #[test]
    fn sends_intent_with_remaining_selections_after_partial_deselection() {
        // Arrange
        let mut world = make_world();
        let entity1 = world.spawn(point_type()).id();
        let entity2 = world.spawn(edge_type()).id();
        {
            let mut selections = world.resource_mut::<AppSelections>();
            selections.insert(entity1, point_type());
            selections.insert(entity2, edge_type());
        }

        // Act
        send_select_entity(&mut world, entity1);

        // Assert
        let intent = captured_selection_intent(&world).expect("intent should be sent");
        assert_eq!(intent.selections, vec![ObjectType::Edge(EdgeId::new(1))]);
    }
}
