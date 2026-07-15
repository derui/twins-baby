#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::{
    id::{BodyId, IdStore, SketchId},
    sketch::{AttachableTarget, Sketch},
};

use color_eyre::eyre::{Result, eyre};
use tracing::instrument;

/// The root data model of Sketch perspective
#[derive(Debug, Clone)]
pub struct SketchPerspective {
    sketches: HashMap<SketchId, Sketch>,
    sketch_id_gen: IdStore,
}

impl Default for SketchPerspective {
    fn default() -> Self {
        Self {
            sketches: Default::default(),
            sketch_id_gen: IdStore::of(),
        }
    }
}

impl SketchPerspective {
    /// Create a new perspective
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a sketch reference of the id
    pub fn get(&self, id: &SketchId) -> Option<&Sketch> {
        self.sketches.get(id)
    }

    /// Get a sketch mutable reference of the id
    pub fn get_mut(&mut self, id: &SketchId) -> Option<&mut Sketch> {
        self.sketches.get_mut(id)
    }

    /// Add a new sketch to the perpective
    pub fn add_sketch(&mut self, body: BodyId, target: &AttachableTarget) -> SketchId {
        let id = self.sketch_id_gen.generate();
        let sketch = Sketch::new(&format!("Sketch{}", id), body, target);

        self.sketches.insert(id, sketch);
        id
    }

    /// Remove sketch from perspective
    pub fn remove_sketch(&mut self, id: &SketchId) -> Option<Sketch> {
        self.sketches.remove(id)
    }

    /// Rename a sketch
    #[instrument(err)]
    pub fn remane_sketch(&mut self, id: &SketchId, new_name: &str) -> Result<()> {
        if new_name.trim().is_empty() {
            return Err(eyre!("Do not allow empty string"));
        }

        if self.sketches.values().any(|s| s.name.as_str() == new_name) {
            return Err(eyre!("Sketch with name '{new_name}' already exists"));
        }

        let sketch = self
            .get_mut(id)
            .ok_or_else(|| eyre!("Sketch with id {id} not found"))?;

        sketch.set_name(new_name)
    }
}
