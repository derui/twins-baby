//! Resource for global state of application whole.
//! State for application working is added `App` prefix.

use std::ops::Deref;

use bevy::{
    app::App,
    ecs::{entity::Entity, resource::Resource},
};
use cad_base::{
    CadEngine,
    id::{BodyId, SketchId},
};

use crate::bevy_app::component::BodyPartType;

/// Extention trait to initialize all resources for application state. This should be called when app is created.
pub trait AppResourceExt {
    /// Initialize all resources for application state. This should be called when app is created.
    fn init_app_resources(&mut self) -> &mut Self;
}

impl AppResourceExt for App {
    fn init_app_resources(&mut self) -> &mut Self {
        self.init_resource::<EngineState>()
            .init_resource::<AppActiveBody>()
            .init_resource::<AppSelections>()
            .init_resource::<AppActiveSketch>()
    }
}

/// Global system registry.
#[derive(Resource, Default)]
pub struct EngineState(pub CadEngine);

/// Current active body. Can not active multiple bodies at once.
#[derive(Resource, Default, Debug)]
pub struct AppActiveBody(pub Option<BodyId>);

/// Current active sketch. Can not active multiple sketches at once.
#[derive(Resource, Default, Debug)]
pub struct AppActiveSketch(pub Option<SketchId>);

/// Selected entity/body part mapping.
#[derive(Resource, Default, Debug)]
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

    /// Clear all selections.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.0.iter().any(|(e, _)| *e == entity)
    }
}

impl From<Vec<(Entity, BodyPartType)>> for AppSelections {
    fn from(value: Vec<(Entity, BodyPartType)>) -> Self {
        Self(value)
    }
}

impl Deref for AppSelections {
    type Target = Vec<(Entity, BodyPartType)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
