use bevy::ecs::resource::Resource;
use cad_base::{CadEngine, id::BodyId};

/// Global system registry.
#[derive(Resource, Default)]
pub struct EngineState(pub CadEngine);

#[derive(Resource, Default)]
pub struct EngineAppState {
    /// An active body. This is the source of some operations.
    pub active_body: Option<BodyId>,
}
