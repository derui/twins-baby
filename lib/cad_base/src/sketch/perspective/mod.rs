#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::{
    id::{IdStore, PlaneId, SketchId},
    sketch::{AttachableTarget, Sketch},
};

use anyhow::{Result, anyhow};

/// The root data model of Sketch perspective
#[derive(Debug, Clone)]
pub struct SketchPerspective {
    sketches: HashMap<SketchId, Sketch>,
    sketch_id_gen: IdStore<SketchId>,
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
    pub fn add_sketch(&mut self, plane: &PlaneId) -> SketchId {
        let id = self.sketch_id_gen.generate();
        let sketch = Sketch::new(&id.to_string(), &AttachableTarget::Plane(plane.clone()));

        self.sketches.insert(id, sketch);
        id
    }

    /// Remove sketch from perspective
    pub fn remove_sketch(&mut self, id: &SketchId) -> Option<Sketch> {
        self.sketches.remove(id)
    }

    /// Rename a sketch
    pub fn remane_sketch(&mut self, id: &SketchId, new_name: &str) -> Result<()> {
        if new_name.trim().is_empty() {
            return Err(anyhow!("Do not allow empty string"));
        }

        if self.sketches.values().any(|s| s.name.as_str() == new_name) {
            return Err(anyhow!("Sketch with name '{new_name}' already exists"));
        }

        let sketch = self
            .get_mut(id)
            .ok_or_else(|| anyhow!("Sketch with id {id} not found"))?;

        sketch.set_name(new_name)
    }
}
