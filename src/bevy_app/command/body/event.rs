// internal events

use bevy::ecs::{event::Event, message::Message};
use cad_base::body::PlaneRef;

/// An internal event to change Active Plane
#[derive(Debug, Clone, PartialEq, Eq, Message)]
pub struct InternalChangeActivePlane {
    pub plane_ref: PlaneRef,
}
