use std::collections::HashMap;

use bevy::ecs::{entity::Entity, resource::Resource};
use cad_base::{CadEngine, id::BodyId};

/// Global system registry.
#[derive(Resource, Default)]
pub struct EngineState(pub CadEngine);

#[derive(Resource, Default)]
pub struct EngineAppState {
    /// An active body. This is the source of some operations.
    pub active_body: Option<BodyId>,

    /// A management of body-based plane map
    pub body_planes_map: HashMap<BodyId, Vec<Entity>>,
}
