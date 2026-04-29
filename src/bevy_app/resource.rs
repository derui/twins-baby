use std::collections::HashMap;

use bevy::ecs::{entity::Entity, resource::Resource};
use cad_base::{
    CadEngine,
    id::{BodyId, FaceId},
};

/// Global system registry.
#[derive(Resource, Default)]
pub struct EngineState(pub CadEngine);

#[derive(Resource, Default)]
pub struct EngineAppState {
    /// An active body. This is the source of some operations.
    pub active_body: Option<BodyId>,

    /// An current selected face.
    pub active_face: Option<FaceId>,

    /// A management of body-based plane map
    pub body_planes_map: HashMap<BodyId, Vec<Entity>>,
}
