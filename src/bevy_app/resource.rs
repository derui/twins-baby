use bevy::ecs::resource::Resource;
use cad_base::transaction::registry::PerspectiveRegistry;

/// Global system registry.
#[derive(Resource)]
pub struct EngineState(PerspectiveRegistry);

impl Default for EngineState {
    fn default() -> Self {
        Self(PerspectiveRegistry::new())
    }
}
