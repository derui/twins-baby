//! Resource for global state of application whole.
//! State for application working is added `App` prefix.

use bevy::ecs::{entity::Entity, resource::Resource};
use cad_base::{CadEngine, id::BodyId};

use crate::bevy_app::component::BodyPartType;

/// Global system registry.
#[derive(Resource, Default)]
pub struct EngineState(pub CadEngine);

/// Current active body. Can not active multiple bodies at once.
#[derive(Resource, Default)]
pub struct AppActiveBody(pub Option<BodyId>);

/// Selected entity/body part mapping.
#[derive(Resource, Default)]
pub struct AppSelections(Vec<(Entity, BodyPartType)>);

impl AppSelections {
    /// Insert the entity to selection. If the entity is already selected, do nothing.
    pub fn insert(&mut self, entity: Entity, part: BodyPartType) {
        if self.0.iter().any(|(e, _)| *e == entity) {
            return;
        }
        self.0.push((entity, part));
    }

    /// Remove the entity from selection. If the entity is not selected, do nothing.
    pub fn remove(&mut self, entity: Entity) {
        self.0.retain(|(e, _)| *e != entity);
    }
}

impl From<Vec<(Entity, BodyPartType)>> for AppSelections {
    fn from(value: Vec<(Entity, BodyPartType)>) -> Self {
        Self(value)
    }
}

#[derive(Resource, Default)]
pub struct EngineAppState {
    /// An active body. This is the source of some operations.
    pub active_body: Option<BodyId>,

    /// Current selected objects
    pub selections: Vec<(Entity, BodyPartType)>,
}
