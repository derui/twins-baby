// internal events

use bevy::ecs::event::Event;
use cad_base::body::PlaneRef;

/// An internal event to change Active Plane
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct InternalChangeActivePlane {
    pub plane_ref: PlaneRef,
}
