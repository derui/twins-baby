// internal events

use bevy::ecs::{entity::Entity, message::Message};
use immutable::Im;

/// An internal event to change Active Plane
#[derive(Debug, Clone, PartialEq, Eq, Message)]
pub struct InternalSelectObject {
    /// Entity id
    pub entity: Im<Entity>,
}
