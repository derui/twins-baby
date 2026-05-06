use bevy::ecs::{entity::Entity, resource::Resource};
use cad_base::{CadEngine, id::BodyId};

use crate::bevy_app::component::BodyPartType;

/// Global system registry.
#[derive(Resource, Default)]
pub struct EngineState(pub CadEngine);

#[derive(Resource, Default)]
pub struct EngineAppState {
    /// An active body. This is the source of some operations.
    pub active_body: Option<BodyId>,

    /// Current selected objects
    pub selections: Vec<(Entity, BodyPartType)>,
}
