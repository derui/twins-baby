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
pub(super) fn update_toggling_selection(
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
