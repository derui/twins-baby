use std::collections::HashMap;

use bevy::ecs::{entity::Entity, resource::Resource};
use cad_base::{CadEngine, id::BodyId, sketch::AttachableTarget};

use crate::bevy_app::component::ObjectType;

/// Global system registry.
#[derive(Resource, Default)]
pub struct EngineState(pub CadEngine);

#[derive(Resource, Default)]
pub struct EngineAppState {
    /// An active body. This is the source of some operations.
    pub active_body: Option<BodyId>,

    /// Current selected objects
    pub selections: Vec<(Entity, ObjectType)>,

    /// A management of body-based plane map
    pub body_planes_map: HashMap<BodyId, Vec<Entity>>,
}
