// List of components

use bevy::ecs::component::Component;
use ui_event::ObjectType;

/// Enum of types of object in CAD. This uses to pick, edit, and move.
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct BodyPartType(pub ObjectType);
