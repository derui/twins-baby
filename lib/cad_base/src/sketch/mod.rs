#[cfg(test)]
mod tests;

mod constraint;
mod geometry;
mod point2;
mod scope;

use std::collections::HashMap;

use crate::{
    id::{GeometryId, IdStore, PlaneId, SketchId},
    sketch::scope::{ConstraintScope, VariableScope},
};

use anyhow::{Result, anyhow};
pub use constraint::*;
pub use geometry::*;
use immutable::Im;
pub use point2::*;

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
        let sketch = Sketch::new(&id.to_string(), plane);

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

/// The sketch of base of modeling.
///
/// [Sketch] has these values:
///
/// - geometries defined as some basic geometres
/// - attached Plane with plane id.
/// - constraints equations for points (not implemented yet)
///
#[derive(Debug, Clone)]
pub struct Sketch {
    /// Name of this sketch
    pub name: Im<String>,

    geometory_id_gen: IdStore<GeometryId>,

    /// Geometries in this sketch
    geometries: HashMap<GeometryId, Box<Geometry>>,

    /// variable scope.
    variables: VariableScope,

    /// Constraint scope
    constraints: ConstraintScope,

    /// A plane atteched to sketch
    attached_plane: PlaneId,
}

impl Sketch {
    /// Create a new sketch with builder
    fn new(name: &str, attached_plane: &PlaneId) -> Self {
        Sketch {
            name: name.to_string().into(),
            geometory_id_gen: IdStore::of(),
            geometries: HashMap::new(),
            variables: VariableScope::new(),
            constraints: ConstraintScope::new(),
            attached_plane: *attached_plane,
        }
    }

    /// Set name for the sketch.
    ///
    /// # Errors
    /// Returns error when the new name is empty string.
    fn set_name(&mut self, new_name: &str) -> Result<()> {
        if new_name.trim().is_empty() {
            return Err(anyhow!("Do not allow empty string"));
        }

        self.name = new_name.trim().to_string().into();

        Ok(())
    }

    /// Add a geometry to this sketch with a geometry maker function
    pub fn add_geometry<F>(&mut self, maker: F) -> GeometryId
    where
        F: FnOnce(&mut VariableScope) -> Geometry,
    {
        let geometry = maker(&mut self.variables);

        let id = self.geometory_id_gen.generate();
        self.geometries.insert(id, Box::new(geometry));
        id
    }

    /// Remove a geometry from this sketch
    pub fn remove_geometry(&mut self, id: &GeometryId) -> Option<Box<Geometry>> {
        self.geometries.remove(id)
    }
}
