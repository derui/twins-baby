// List of components

use bevy::ecs::component::Component;
use cad_base::{
    body::PlaneRef,
    id::{EdgeId, FaceId},
};
use ui_event::ObjectType;

/// Enum of types of object in CAD. This uses to pick, edit, and move.
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct BodyPartType(pub ObjectType);
