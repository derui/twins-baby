use bevy::ecs::resource::Resource;
use cad_base::CadEngine;

/// Global system registry.
#[derive(Resource, Default)]
pub struct EngineState(pub CadEngine);
