use bevy::ecs::resource::Resource;
use cad_base::CadEngine;

/// Global system registry.
#[derive(Resource)]
pub struct EngineState(CadEngine);

impl Default for EngineState {
    fn default() -> Self {
        Self(CadEngine::new())
    }
}
